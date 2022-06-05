use {
  crate::state::ContractState,
  fil_actors_runtime::{
    actor_error,
    runtime::{ActorCode, Runtime},
    ActorError,
    INIT_ACTOR_ADDR,
  },
  fvm_evm::{EthereumAccount, H160, U256},
  fvm_ipld_blockstore::Blockstore,
  fvm_ipld_encoding::{from_slice, RawBytes},
  fvm_shared::{
    address::Address,
    econ::TokenAmount,
    MethodNum,
    METHOD_CONSTRUCTOR,
  },
  num_derive::FromPrimitive,
  num_traits::{FromPrimitive, Zero},
};

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
  Constructor = METHOD_CONSTRUCTOR,
  CreateZeroAddress = 2,
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
    ContractState::new(bytecode, registry, rt.store())
      .map_err(|e| ActorError::illegal_state(e.to_string()))?;
    Ok(())
  }

  pub fn create_zero_address<BS, RT>(rt: &mut RT) -> Result<(), ActorError>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    rt.validate_immediate_caller_accept_any()?;
    let state: ContractState = rt.state()?;

    let eth_addr = H160::zero();
    let eth_acc = EthereumAccount {
      nonce: 5,
      balance: U256::from(999u64),
      ..Default::default()
    };

    const UPSERT_METHOD_NUM: u64 = 3;

    let params = RawBytes::serialize((eth_addr, eth_acc))
      .map_err(|e| ActorError::illegal_argument(e.to_string()))?;

    // register new address through cross-contract call
    rt.send(
      state.registry,
      UPSERT_METHOD_NUM,
      params,
      TokenAmount::zero(),
    )?;

    Ok(())
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
      Some(Method::CreateZeroAddress) => {
        Self::create_zero_address(rt)?;
        fvm_sdk::debug::log(format!("created zero address"));
        Ok(RawBytes::default())
      }
      None => Err(actor_error!(unhandled_message; "Invalid method")),
    }
  }
}
