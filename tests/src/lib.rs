use {
  anyhow::Result,
  cid::Cid,
  fvm::executor::{ApplyKind, Executor},
  fvm_evm::{
    SignedTransaction,
    Transaction,
    TransactionRecoveryId,
    TransactionSignature,
    H256,
  },
  fvm_integration_tests::tester::{Account, Tester},
  fvm_ipld_blockstore::MemoryBlockstore,
  fvm_ipld_encoding::{Cbor, RawBytes},
  fvm_shared::{
    address::Address,
    bigint::{BigInt, Zero},
    message::Message,
    state::StateTreeVersion,
    version::NetworkVersion,
    MethodNum,
    METHOD_CONSTRUCTOR,
  },
  libsecp256k1::{sign, Message as SecpMessage, SecretKey},
  serde::{Deserialize, Serialize},
  std::{
    collections::{hash_map::Entry, HashMap},
    fs,
  },
};

#[cfg(test)]
mod bridge;

#[cfg(test)]
mod runtime;

pub struct EVMTester {
  _bridge_code_cid: Cid,
  runtime_code_cid: Cid,
  instance: Tester<MemoryBlockstore>,
  accounts: Vec<Account>,
  sequences: HashMap<Address, u64>,
}

// constants
impl EVMTester {
  pub const BRIDGE_ACTOR_ADDRESS: Address = Address::new_id(10002);
  pub const BRIDGE_ACTOR_WASM_PATH: &'static str = "../wasm/fvm_evm_bridge.compact.wasm";
  pub const INIT_ACTOR_ADDRESS: Address = Address::new_id(1);
  pub const RUNTIME_ACTOR_ADDRESS: Address = Address::new_id(10001);
  pub const RUNTIME_ACTOR_WASM_PATH: &'static str =
    "../wasm/fvm_evm_runtime.compact.wasm";
}

impl EVMTester {
  /// Creates an FVM simulator with both actors loaded from WASM
  /// ready to execute messages from external sources
  pub fn new<const N: usize>() -> Result<Self> {
    let mut instance = Tester::new(
      NetworkVersion::V16,
      StateTreeVersion::V4,
      MemoryBlockstore::default(),
    )?;

    let accounts = instance.create_accounts::<N>()?.to_vec();

    let runtime_actor_wasm = fs::read(Self::RUNTIME_ACTOR_WASM_PATH)?;
    let bridge_actor_wasm = fs::read(Self::BRIDGE_ACTOR_WASM_PATH)?;

    #[derive(Debug, Serialize, Deserialize)]
    struct State {
      empty: bool,
    }

    let empty_state = State { empty: true };
    let state_root = instance.set_state(&empty_state)?;

    let runtime_code_cid = instance.set_actor_from_bin(
      &runtime_actor_wasm,
      state_root,
      Self::RUNTIME_ACTOR_ADDRESS,
      BigInt::zero(),
    )?;

    let bridge_code_cid = instance.set_actor_from_bin(
      &bridge_actor_wasm,
      state_root,
      Self::BRIDGE_ACTOR_ADDRESS,
      BigInt::zero(),
    )?;

    instance.instantiate_machine()?;

    Ok(Self {
      accounts,
      _bridge_code_cid: bridge_code_cid,
      runtime_code_cid,
      instance,
      sequences: HashMap::new(),
    })
  }

  pub fn runtime_code_cid(&self) -> &Cid {
    &self.runtime_code_cid
  }

  pub fn accounts(&self) -> &[Account] {
    &self.accounts
  }

  pub fn send_message(
    &mut self,
    from: Address,
    to: Address,
    method_num: MethodNum,
    params: RawBytes,
    kind: ApplyKind,
  ) -> Result<RawBytes> {
    let sequence = match self.sequences.entry(from) {
      Entry::Occupied(mut o) => {
        *o.get_mut() += 1;
        *o.get()
      }
      Entry::Vacant(v) => *v.insert(0),
    };
    let message = Message {
      from,
      to,
      gas_limit: 10000000000,
      method_num,
      params,
      sequence,
      ..Message::default()
    };

    let message_len = message.marshal_cbor()?;
    let output = match self.instance.executor {
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
    &mut self,
    from: Address,
    to: Address,
    method: MethodNum,
    params: RawBytes,
  ) -> Result<RawBytes> {
    self.send_message(from, to, method, params, ApplyKind::Explicit)
  }

  pub fn send_implicit_message(
    &mut self,
    from: Address,
    to: Address,
    method: MethodNum,
    params: RawBytes,
  ) -> Result<RawBytes> {
    self.send_message(from, to, method, params, ApplyKind::Implicit)
  }

  pub fn construct_actor(
    &mut self,
    address: Address,
    params: RawBytes,
  ) -> Result<RawBytes> {
    self.send_implicit_message(
      Self::INIT_ACTOR_ADDRESS,
      address,
      METHOD_CONSTRUCTOR,
      params,
    )
  }

  pub fn invoke_actor(
    &mut self,
    caller: Address,
    address: Address,
    method: MethodNum,
    params: RawBytes,
  ) -> Result<RawBytes> {
    self.send_explicit_message(caller, address, method, params)
  }
}

pub fn sign_evm_transaction(
  transaction: Transaction,
  seckey: SecretKey,
) -> SignedTransaction {
  let hash = transaction.hash();
  let (signature, recovery_id) =
    sign(&SecpMessage::parse(&hash.as_fixed_bytes()), &seckey);
  let recovery_id = recovery_id.serialize() as u64;
  let signature = TransactionSignature {
    v: TransactionRecoveryId(match transaction {
      Transaction::Legacy { .. } => match transaction.chain_id() {
        Some(chain_id) => chain_id * 2 + 35 + recovery_id,
        None => recovery_id,
      },
      Transaction::EIP2930 { .. } => recovery_id,
      Transaction::EIP1559 { .. } => recovery_id,
    }),
    r: H256::from_slice(&signature.r.b32()),
    s: H256::from_slice(&signature.s.b32()),
  };

  SignedTransaction {
    transaction,
    signature,
  }
}
