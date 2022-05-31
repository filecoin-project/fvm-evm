mod abort;
mod state;
mod storage;

use {
  abort::abort,
  fvm_ipld_encoding::{RawBytes, DAG_CBOR},
  fvm_sdk::{ipld, message, NO_DATA_BLOCK_ID},
  fvm_shared::ActorID,
  state::EVMContractState,
};

#[no_mangle]
pub fn invoke(params_ptr: u32) -> u32 {
  // Conduct method dispatch. Handle input parameters and return data.
  let ret: Option<RawBytes> = match message::method_number() {
    1 => constructor(params_ptr),
    2 => hello_there(params_ptr),
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

  let (codec, bytes) = fvm_sdk::message::params_raw(params_ptr).unwrap();
  if codec != DAG_CBOR {
    abort!(
      USR_SERIALIZATION,
      "invalid input format, expected DAG-CBOR, got {codec}"
    );
  }

  let state = EVMContractState::new(&bytes);
  state.save();
  Some(RawBytes::new(b"EVM Constructor called!".to_vec()))
}

pub fn get_bytecode() -> Option<RawBytes> {
  let state = EVMContractState::load();
}

pub fn hello_there(params_ptr: u32) -> Option<RawBytes> {
  let (codec, bytes) = fvm_sdk::message::params_raw(params_ptr).unwrap();
  if codec != DAG_CBOR {
    abort!(
      USR_SERIALIZATION,
      "invalid input format, expected DAG-CBOR, got {codec}"
    );
  }

  let logmsg = format!("codec: {codec} (DAG-CBOR), bytes: {bytes:?}");
  let param_string = String::from_utf8(bytes);
  Some(RawBytes::new(
    format!("params: {logmsg} [string: {param_string:?}]").into_bytes(),
  ))
}

#[cfg(test)]
mod tests {}
