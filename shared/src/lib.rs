//! Shared types between the EVM address registry and EVM runtime actors

mod account;
mod bytecode;
mod execution;
mod instructions;
mod memory;
mod message;
mod opcode;
mod output;
mod stack;
mod system;
mod transaction;
pub mod uints;

pub use {
  account::{AccountKind, EthereumAccount},
  bytecode::Bytecode,
  execution::{execute, ExecutionState},
  message::{CallKind, Message},
  output::{Output, StatusCode},
  system::System,
  transaction::{
    SignedTransaction,
    Transaction,
    TransactionAction,
    TransactionRecoveryId,
    TransactionSignature,
  },
  uints::{H160, H256, U256, U512},
};

/// This type is used to construct a new instance of an EVM contract.
/// Instances of this type are created by the bridge actor after a successful
/// invocation of EVM contract constructor.
#[derive(Debug, serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub struct EvmContractRuntimeConstructor {
  pub initial_state: cid::Cid,
  pub bytecode: bytes::Bytes,
  pub registry: fvm_shared::address::Address,
}

#[macro_export]
macro_rules! abort {
  ($code:ident, $msg:literal $(, $ex:expr)*) => {
      fvm_sdk::vm::abort(
          fvm_shared::error::ExitCode::$code.value(),
          Some(format!($msg, $($ex,)*).as_str()),
      )
  };
}
