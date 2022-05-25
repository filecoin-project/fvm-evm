mod abort;
mod state;
mod storage;

use {
  abort::abort,
  fvm_ipld_encoding::{to_vec, RawBytes, DAG_CBOR},
  fvm_sdk::{ipld, message, NO_DATA_BLOCK_ID},
  fvm_shared::ActorID,
  state::State,
};

#[no_mangle]
pub fn invoke(_: u32) -> u32 {
  // Conduct method dispatch. Handle input parameters and return data.
  let ret: Option<RawBytes> = match message::method_number() {
    1 => constructor(),
    2 => say_hello(),
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

pub fn constructor() -> Option<RawBytes> {
  // This constant should be part of the SDK.
  const INIT_ACTOR_ADDR: ActorID = 1;

  // Should add SDK sugar to perform ACL checks more succinctly.
  // i.e. the equivalent of the validate_* builtin-actors runtime methods.
  // https://github.com/filecoin-project/builtin-actors/blob/master/actors/runtime/src/runtime/fvm.rs#L110-L146
  if message::caller() != INIT_ACTOR_ADDR {
    abort!(USR_FORBIDDEN, "constructor invoked by non-init actor");
  }

  let state = State::default();
  state.save();
  None
}

pub fn say_hello() -> Option<RawBytes> {
  let mut state = State::load();
  state.count += 1;
  state.save();

  let ret = to_vec(format!("Hello world #{}!", &state.count).as_str());
  match ret {
    Ok(ret) => Some(RawBytes::new(ret)),
    Err(err) => {
      abort!(
        USR_ILLEGAL_STATE,
        "failed to serialize return value: {:?}",
        err
      );
    }
  }
}

#[cfg(test)]
mod tests {
  use {
    ethereum_types::{Address, U256},
    evmodin::{
      host::DummyHost,
      tracing::NoopTracer,
      util::Bytecode,
      AnalyzedCode,
      CallKind,
      Message,
      Output,
      Revision,
      StatusCode,
    },
  };

  #[test]
  fn evmodin_smoke_test() {
    let code = Bytecode::new()
      .mstore8_value(0, b'h')
      .mstore8_value(1, b'e')
      .mstore8_value(2, b'l')
      .mstore8_value(3, b'l')
      .mstore8_value(4, b'o')
      .ret(0, 5)
      .build();

    let message = Message {
      kind: CallKind::Call,
      is_static: true,
      depth: 0,
      gas: 200,
      recipient: Address::zero(),
      code_address: Address::zero(),
      sender: Address::zero(),
      input_data: vec![].into(),
      value: U256::zero(),
    };

    assert_eq!(
      AnalyzedCode::analyze(code).execute(
        &mut DummyHost,
        &mut NoopTracer,
        None,
        message,
        Revision::latest()
      ),
      Output {
        status_code: StatusCode::Success,
        gas_left: 146,
        output_data: b"hello".to_vec().into(),
        create_address: None,
      }
    )
  }
}
