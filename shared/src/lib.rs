//! Shared types between the EVM address registry and EVM runtime actors

mod account;
mod uints;

pub use {
  account::{AccountKind, EthereumAccount},
  uints::{H160, H256, U256, U512},
};

#[macro_export]
macro_rules! abort {
  ($code:ident, $msg:literal $(, $ex:expr)*) => {
      fvm_sdk::vm::abort(
          fvm_shared::error::ExitCode::$code.value(),
          Some(format!($msg, $($ex,)*).as_str()),
      )
  };
}
