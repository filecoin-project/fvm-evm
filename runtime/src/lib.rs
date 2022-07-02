use {
  crate::state::ContractState,
  fil_actors_runtime::{
    actor_error,
    runtime::{ActorCode, Runtime},
    ActorError,
  },
  fvm_evm::{
    execute,
    Bytecode,
    EvmContractRuntimeConstructor,
    ExecutionState,
    StatusCode,
    System,
    U256,
  },
  fvm_ipld_blockstore::Blockstore,
  fvm_ipld_encoding::{from_slice, RawBytes},
  fvm_ipld_hamt::Hamt,
  fvm_sdk::{debug, ipld},
  fvm_shared::{MethodNum, METHOD_CONSTRUCTOR},
  num_derive::FromPrimitive,
  num_traits::FromPrimitive,
};

mod state;

#[cfg(feature = "fil-actor")]
fil_actors_runtime::wasm_trampoline!(EvmRuntimeActor);

/// Maximum allowed EVM bytecode size.
/// The contract code size limit is 24kB.
const MAX_CODE_SIZE: usize = 0x6000;

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
  Constructor = METHOD_CONSTRUCTOR,
  ProcessTransaction = 2,
  GetStorageValue = 3,
  GetCodeHash = 4,
  GetCodeSize = 5,
}

pub struct EvmRuntimeActor;
impl EvmRuntimeActor {
  pub fn constructor<BS, RT>(
    rt: &mut RT,
    args: &EvmContractRuntimeConstructor,
  ) -> Result<(), ActorError>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    fvm_sdk::debug::log(format!(
      "Inside FVM Runtime actor constructor! params: {args:#?}"
    ));
    rt.validate_immediate_caller_accept_any()?;

    if args.bytecode.len() > MAX_CODE_SIZE {
      return Err(ActorError::illegal_argument(format!(
        "EVM byte code length ({}) is exceeding the maximum allowed of {MAX_CODE_SIZE}",
        args.bytecode.len()
      )));
    }

    if args.bytecode.is_empty() {
      return Err(ActorError::illegal_argument("no bytecode provided".into()));
    }

    if args.initial_state == cid::Cid::default() {
      return Err(ActorError::illegal_state(
        "EVM Actor must be initialized to some initial state".into(),
      ));
    }

    ContractState::new(
      &args.bytecode,
      args.registry,
      args.address,
      args.initial_state,
    )
    .map_err(|e| ActorError::illegal_state(e.to_string()))?;

    Ok(())
  }

  pub fn process_transaction<BS, RT>(
    rt: &mut RT,
    rlp: &[u8],
  ) -> Result<RawBytes, ActorError>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    rt.validate_immediate_caller_accept_any()?;

    let transaction = fvm_evm::SignedTransaction::try_from(rlp)
      .map_err(|e| ActorError::illegal_argument(format!("{e:?}")))?;
    debug::log(format!("Runtime processing tx: {transaction:#?}"));

    // Load current actor state tree that contains its bytecode,
    // state Hamt and metadata about the bridge address and its own EVM address.
    let state: ContractState = rt.state()?;

    // retreive the EVM bytecode from actor state tree
    let bytecode_block = ipld::get(&state.bytecode)
      .map_err(|e| ActorError::illegal_state(format!("{e:?}")))?;

    // Deserialize bytecode CBOR block into byte array
    let bytecode_bytes: Vec<u8> = from_slice(&bytecode_block)
      .map_err(|e| ActorError::serialization(format!("{e:?}")))?;

    // Analize raw EVM bytecode and identify all valid jump destinations
    let bytecode = Bytecode::new(&bytecode_bytes)
      .map_err(|e| ActorError::unspecified(format!("EVM bytecode error: {e:?}")))?;

    let hamt = Hamt::<_, U256, U256>::load(&state.state, rt.store()).unwrap();
    hamt
      .for_each(|k, v| {
        let mut key_bytes = [0u8; 32];
        k.to_big_endian(&mut key_bytes);
        debug::log(format!("hamt entry: 0x{} -> {v:?}", hex::encode(key_bytes)));
        Ok(())
      })
      .unwrap();

    // Create an instance of a platform abstraction layer
    let system = System::new(
      state.state,
      rt,
      state.bridge,
      state.self_address,
      &transaction,
    )
    .map_err(|e| ActorError::unspecified(format!("EVM system error: {e:?}")))?;

    // Create an execution context for the incoming transaction
    let message = transaction.try_into()?;
    let mut exec_state = ExecutionState::new(&message);

    // invoke the transaction on the bytecode of this actor
    let exec_status = execute(&bytecode, &mut exec_state, &system)
      .map_err(|e| ActorError::unspecified(format!("EVM execution error: {e:?}")))?;

    debug::log(format!("Exec Status: {exec_status:#?}"));

    if !exec_status.reverted && exec_status.status_code == StatusCode::Success {
      Ok(
        RawBytes::serialize(U256::from_big_endian(&exec_status.output_data))
          .map_err(|e| ActorError::serialization(format!("{e:?}")))?,
      )
    } else {
      Err(ActorError::unspecified(format!(
        "EVM error: {:?}",
        exec_status
      )))
    }
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
        Self::constructor(rt, &from_slice(&params)?)?;
        Ok(RawBytes::default())
      }
      Some(Method::ProcessTransaction) => {
        let rlp: Vec<u8> = from_slice(&params)?;
        Self::process_transaction(rt, &rlp)
      }
      Some(Method::GetStorageValue) => Self::get_storage_value(rt),
      Some(Method::GetCodeHash) => Self::get_code_hash(rt),
      Some(Method::GetCodeSize) => Self::get_code_size(rt),
      None => Err(actor_error!(unhandled_message; "Invalid method")),
    }
  }
}
