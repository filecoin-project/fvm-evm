use {
  fvm::executor::{ApplyKind, Executor},
  fvm_integration_tests::tester::Account,
  fvm_ipld_encoding::{Cbor, RawBytes},
  fvm_shared::{message::Message, MethodNum, METHOD_CONSTRUCTOR},
};

#[cfg(test)]
mod bridge;

#[cfg(test)]
mod runtime;

use {
  anyhow::Result,
  fvm_integration_tests::tester::Tester,
  fvm_ipld_blockstore::MemoryBlockstore,
  fvm_shared::{
    address::Address,
    bigint::{BigInt, Zero},
    state::StateTreeVersion,
    version::NetworkVersion,
  },
  serde::{Deserialize, Serialize},
  std::{fs, sync::Arc},
};

pub const INIT_ACTOR_ADDRESS: Address = Address::new_id(1);

const RUNTIME_ACTOR_ADDRESS: Address = Address::new_id(10001);
const RUNTIME_ACTOR_WASM_PATH: &str = "../wasm/fvm_evm_runtime.compact.wasm";

const BRIDGE_ACTOR_ADDRESS: Address = Address::new_id(10002);
const BRIDGE_ACTOR_WASM_PATH: &str = "../wasm/fvm_evm_bridge.compact.wasm";

/// Creates an FVM simulator with both actors loaded from WASM
/// ready to execute messages from external sources
pub fn create_tester<const N: usize>() -> Result<(Tester<MemoryBlockstore>, [Account; N])>
{
  let mut tester = Tester::new(
    NetworkVersion::V16,
    StateTreeVersion::V4,
    MemoryBlockstore::default(),
  )?;

  let accounts: [Account; N] = tester.create_accounts()?;

  let runtime_actor_wasm = fs::read(RUNTIME_ACTOR_WASM_PATH)?;
  let bridge_actor_wasm = fs::read(BRIDGE_ACTOR_WASM_PATH)?;

  #[derive(Debug, Serialize, Deserialize)]
  struct State {
    empty: bool,
  }

  let empty_state = State { empty: true };
  let state_root = tester.set_state(&empty_state)?;

  tester.set_actor_from_bin(
    &runtime_actor_wasm,
    state_root,
    RUNTIME_ACTOR_ADDRESS,
    BigInt::zero(),
  )?;

  tester.set_actor_from_bin(
    &bridge_actor_wasm,
    state_root,
    BRIDGE_ACTOR_ADDRESS,
    BigInt::zero(),
  )?;

  tester.instantiate_machine()?;

  Ok((tester, accounts))
}

pub fn run_in_large_stack(
  op: impl FnOnce() -> Result<()> + Send + Sync + 'static,
) -> Result<()> {
  let mut result = Arc::new(None);
  let mut result_clone = result.clone();
  std::thread::Builder::new()
    .stack_size(64 << 21)
    .spawn(move || {
      let result = Arc::get_mut(&mut result_clone).unwrap();
      result.replace(op());
    })?
    .join()
    .unwrap();

  let result = Arc::get_mut(&mut result).unwrap();
  (*result).take().unwrap()
}

pub fn send_message(
  tester: &mut Tester<MemoryBlockstore>,
  from: Address,
  to: Address,
  method_num: MethodNum,
  params: RawBytes,
  kind: ApplyKind,
  sequence: u64,
) -> Result<RawBytes> {
  let message = Message {
    from,
    to,
    gas_limit: 1000000000,
    method_num,
    params,
    sequence,
    ..Message::default()
  };

  let message_len = message.marshal_cbor()?;
  let output = match tester.executor {
    Some(ref mut executor) => {
      let ret = executor.execute_message(message, kind, message_len.len())?;
      if ret.msg_receipt.exit_code.is_success() {
        ret.msg_receipt.return_data
      } else {
        return Err(anyhow::anyhow!(
          "message failed with exit code {} ({:?})",
          ret.msg_receipt.exit_code,
          ret.failure_info
        ));
      }
    }
    None => return Err(anyhow::anyhow!("executor not initialized")),
  };

  Ok(output)
}

pub fn send_explicit_message(
  tester: &mut Tester<MemoryBlockstore>,
  from: Address,
  to: Address,
  method: MethodNum,
  params: RawBytes,
  sequence: u64,
) -> Result<RawBytes> {
  send_message(
    tester,
    from,
    to,
    method,
    params,
    ApplyKind::Explicit,
    sequence,
  )
}

pub fn send_implicit_message(
  tester: &mut Tester<MemoryBlockstore>,
  from: Address,
  to: Address,
  method: MethodNum,
  params: RawBytes,
  sequence: u64,
) -> Result<RawBytes> {
  send_message(
    tester,
    from,
    to,
    method,
    params,
    ApplyKind::Implicit,
    sequence,
  )
}

pub fn construct_actor(
  tester: &mut Tester<MemoryBlockstore>,
  address: Address,
  params: RawBytes,
) -> Result<RawBytes> {
  send_implicit_message(
    tester,
    INIT_ACTOR_ADDRESS,
    address,
    METHOD_CONSTRUCTOR,
    params,
    0,
  )
}

pub fn invoke_actor(
  tester: &mut Tester<MemoryBlockstore>,
  caller: Address,
  address: Address,
  method: MethodNum,
  params: RawBytes,
  sequence: u64,
) -> Result<RawBytes> {
  send_explicit_message(tester, caller, address, method, params, sequence)
}
