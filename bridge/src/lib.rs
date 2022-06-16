use {
  cid::Cid,
  create::create_contract,
  fil_actors_runtime::{
    actor_error,
    runtime::{ActorCode, Runtime},
    ActorError,
    INIT_ACTOR_ADDR,
  },
  fvm_evm::TransactionAction,
  fvm_ipld_blockstore::Blockstore,
  fvm_ipld_encoding::{from_slice, RawBytes},
  fvm_sdk::debug,
  fvm_shared::{MethodNum, METHOD_CONSTRUCTOR},
  invoke::invoke_contract,
  num_derive::FromPrimitive,
  num_traits::FromPrimitive,
  transfer::transfer_tokens,
};

mod create;
mod invoke;
mod state;
mod transfer;

#[cfg(feature = "fil-actor")]
fil_actors_runtime::wasm_trampoline!(BridgeActor);

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
  Constructor = METHOD_CONSTRUCTOR,
  ProcessTransaction = 2,
}

pub struct BridgeActor;
impl BridgeActor {
  pub fn constructor<BS, RT>(rt: &mut RT, runtime_cid: Cid) -> Result<(), ActorError>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    rt.validate_immediate_caller_is(std::iter::once(&*INIT_ACTOR_ADDR))?;

    // Initialize the global state of the bridge to an empty map.
    // todo: in later iterations initialize with precompiles.
    state::BridgeState::create(rt, &runtime_cid)
      .map_err(|e| ActorError::illegal_argument(format!("{e:?}")))?;
    Ok(())
  }

  /// This is the entry point to interacting with the bridge from RPC nodes
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
    debug::log(format!("FVM transaction: {transaction:#?}"));

    match transaction.action() {
      TransactionAction::Call(_) => invoke_contract(rt, transaction),
      TransactionAction::Create => {
        if transaction.input().is_empty() {
          transfer_tokens(rt, transaction) // transaction is burning tokens
        } else {
          // transaction is creating new contract
          create_contract(rt, transaction)
        }
      }
    }
    .map_err(|e| ActorError::unspecified(format!("EVM Error: {e:?}")))
  }
}

impl ActorCode for BridgeActor {
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
        let runtime_cid: Cid = from_slice(&params)?;
        Self::constructor(rt, runtime_cid)?;
        Ok(RawBytes::default())
      }
      Some(Method::ProcessTransaction) => {
        let rlp: Vec<u8> = from_slice(&params)?;
        Self::process_transaction(rt, &rlp)
      }
      None => Err(actor_error!(unhandled_message; "Invalid method")),
    }
  }
}
