use {
  crate::state::ContractState,
  bytes::Bytes,
  fil_actors_runtime::{
    actor_error,
    runtime::{ActorCode, Runtime},
    ActorError,
    INIT_ACTOR_ADDR,
  },
  fvm_evm::{
    execute,
    Bytecode,
    EthereumAccount,
    ExecutionState,
    Message,
    System,
    H160,
    U256,
  },
  fvm_ipld_blockstore::Blockstore,
  fvm_ipld_encoding::{from_slice, RawBytes},
  fvm_sdk::{debug::log, ipld},
  fvm_shared::{address::Address, econ::TokenAmount, MethodNum, METHOD_CONSTRUCTOR},
  num_derive::FromPrimitive,
  num_traits::FromPrimitive,
};

mod state;

#[cfg(feature = "fil-actor")]
fil_actors_runtime::wasm_trampoline!(EvmRuntimeActor);

/// Maximum allowed EVM bytecode size.
/// The contract code size limit is 24kB.
const _MAX_CODE_SIZE: usize = 0x6000;

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
  Constructor = METHOD_CONSTRUCTOR,
  InvokeContract = 2,
  GetStorageValue = 3,
  GetCodeHash = 4,
  GetCodeSize = 5,
}

pub struct EvmRuntimeActor;
impl EvmRuntimeActor {
  pub fn constructor<BS, RT>(
    rt: &mut RT,
    bytecode: &[u8],
    registry: Address,
  ) -> Result<(), ActorError>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    rt.validate_immediate_caller_is(std::iter::once(&*INIT_ACTOR_ADDR))?;
    ContractState::new(bytecode, registry, rt.store(), H160::zero())
      .map_err(|e| ActorError::illegal_state(e.to_string()))?;
    Ok(())
  }

  pub fn invoke_contract<BS, RT>(rt: &mut RT) -> Result<RawBytes, ActorError>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    rt.validate_immediate_caller_accept_any()?;
    let state: ContractState = rt.state()?;
    let message = Message {
      kind: fvm_evm::CallKind::Call,
      is_static: false,
      depth: 1,
      gas: 2100,
      recipient: H160::zero(),
      sender: H160::zero(),
      input_data: Bytes::new(),
      value: U256::zero(),
    };

    let bytecode: Vec<_> = from_slice(&ipld::get(&state.bytecode).map_err(|e| {
      ActorError::illegal_state(format!("failed to load bytecode: {e:?}"))
    })?)
    .map_err(|e| ActorError::unspecified(format!("failed to load bytecode: {e:?}")))?;

    // EVM contract bytecode
    let bytecode = Bytecode::new(&bytecode)
      .map_err(|e| ActorError::unspecified(format!("invalid bytecode: {e:?}")))?;

    // the execution state of the EVM, stack, heap, etc.
    let mut runtime = ExecutionState::new(&message);

    // the interface between the EVM interpretter and the FVM system
    let mut system = System::new(state.state, rt, state.bridge, state.self_address)
      .map_err(|e| ActorError::unspecified(format!("failed to create runtime: {e:?}")))?;

    // invoke the bytecode using the current state and the platform interface
    let output = execute(&bytecode, &mut runtime, &mut system)
      .map_err(|e| ActorError::unspecified(format!("contract execution error: {e:?}")))?;

    log(format!("evm output: {output:?}"));
    Ok(RawBytes::default())
  }

  pub fn get_storage_value<BS, RT>(rt: &mut RT) -> Result<RawBytes, ActorError>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    rt.validate_immediate_caller_accept_any()?;
    todo!()
  }

  pub fn get_code_hash<BS, RT>(rt: &mut RT) -> Result<RawBytes, ActorError>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    rt.validate_immediate_caller_accept_any()?;
    todo!()
  }

  pub fn get_code_size<BS, RT>(rt: &mut RT) -> Result<RawBytes, ActorError>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    rt.validate_immediate_caller_accept_any()?;
    todo!()
  }
}

impl ActorCode for EvmRuntimeActor {
  fn invoke_method<BS, RT>(
    rt: &mut RT,
    method: MethodNum,
    params: &RawBytes,
  ) -> Result<RawBytes, ActorError>
  where
    BS: Blockstore + Clone,
    RT: Runtime<BS>,
  {
    match FromPrimitive::from_u64(method) {
      Some(Method::Constructor) => {
        let (bytecode, registry): (Vec<u8>, Address) = from_slice(&params)?;
        Self::constructor(rt, &bytecode, registry)?;
        Ok(RawBytes::default())
      }
      Some(Method::InvokeContract) => Self::invoke_contract(rt),
      Some(Method::GetStorageValue) => Self::get_storage_value(rt),
      Some(Method::GetCodeHash) => Self::get_code_hash(rt),
      Some(Method::GetCodeSize) => Self::get_code_size(rt),
      None => Err(actor_error!(unhandled_message; "Invalid method")),
    }
  }
}
