mod state;
mod store;

use {
  fvm_evm::abort,
  fvm_ipld_encoding::{RawBytes, DAG_CBOR},
  fvm_sdk::{ipld, message, NO_DATA_BLOCK_ID},
  fvm_shared::ActorID,
  state::ContractState,
};

#[no_mangle]
pub fn invoke(params_ptr: u32) -> u32 {
  // Conduct method dispatch. Handle input parameters and return data.
  let ret: Option<RawBytes> = match message::method_number() {
    1 => constructor(params_ptr),
    _ => abort!(USR_UNHANDLED_MESSAGE, "unrecognized method"),
  };

  // Insert the return data block if necessary, and return the correct
  // block ID.
  match ret {
    None => NO_DATA_BLOCK_ID,
    Some(v) => match ipld::put_block(DAG_CBOR, v.bytes()) {
      Ok(id) => id,
      Err(err) => {
        abort!(USR_SERIALIZATION, "failed to store return value: {}", err)
      }
    },
  }
}

pub fn constructor(params_ptr: u32) -> Option<RawBytes> {
  const INIT_ACTOR_ADDR: ActorID = 1;
  if message::caller() != INIT_ACTOR_ADDR {
    abort!(USR_FORBIDDEN, "constructor invoked by non-init actor");
  }

  let (codec, bytes) = message::params_raw(params_ptr).unwrap();
  if codec != DAG_CBOR {
    abort!(
      USR_SERIALIZATION,
      "invalid input format, expected DAG-CBOR, got {codec}"
    );
  }

  let state = ContractState::new(&bytes);
  state.save();
  Some(RawBytes::new(b"EVM Constructor called!".to_vec()))
}
