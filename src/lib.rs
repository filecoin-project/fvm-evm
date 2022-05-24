mod storage;

use {
  evmodin::{host::DummyHost, tracing::NoopTracer},
  fil_actors_runtime::runtime::ActorCode,
};

#[cfg(feature = "fil-actor")]
fil_actors_runtime::wasm_trampoline!(EvmActor);

pub struct EvmActor;

#[cfg(test)]
impl Default for EvmActor {
  fn default() -> Self {
    Self
  }
}

#[cfg(not(test))]
impl Default for EvmActor {
  fn default() -> Self {
    Self
  }
}

impl EvmActor {
  pub fn execute_bytecode(
    &self,
    code: &[u8],
    message: evmodin::Message,
  ) -> evmodin::Output {
    evmodin::AnalyzedCode::analyze(code).execute(
      &mut DummyHost,
      &mut NoopTracer,
      None,
      message,
      evmodin::Revision::latest(),
    )
  }
}

impl ActorCode for EvmActor {
  fn invoke_method<BS, RT>(
    _rt: &mut RT,
    _method: fvm_shared::MethodNum,
    _params: &fvm_ipld_encoding::RawBytes,
  ) -> Result<fvm_ipld_encoding::RawBytes, fil_actors_runtime::ActorError>
  where
    // TODO: remove the clone requirement on the blockstore when we fix "replica
    // update" to not hold onto state between transactions.
    // https://github.com/filecoin-project/builtin-actors/issues/133
    BS: fvm_ipld_blockstore::Blockstore + Clone,
    RT: fil_actors_runtime::runtime::Runtime<BS>,
  {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use {
    crate::EvmActor,
    ethereum_types::{Address, U256},
    evmodin::{util::Bytecode, Message},
  };

  #[test]
  fn instantiate_evm_actor() {
    EvmActor::default();
  }

  #[test]
  fn execute_hello_world() {
    let code = Bytecode::new()
      .mstore8_value(0, b'h')
      .mstore8_value(1, b'e')
      .mstore8_value(2, b'l')
      .mstore8_value(3, b'l')
      .mstore8_value(4, b'o')
      .ret(0, 5)
      .build();

    let message = Message {
      kind: evmodin::CallKind::Call,
      is_static: true,
      depth: 0,
      gas: 200,
      recipient: Address::zero(),
      code_address: Address::zero(),
      sender: Address::zero(),
      input_data: vec![].into(),
      value: U256::zero(),
    };

    let evm = EvmActor::default();
    let output = evm.execute_bytecode(&code, message);
    println!("output: {output:?}");
  }
}
