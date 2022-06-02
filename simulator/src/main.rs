use {
  anyhow::Result,
  clap::Parser,
  fvm::executor::{ApplyKind, Executor},
  fvm_evm::{EthereumAccount, H160, U256},
  fvm_integration_tests::tester::{Account, Tester},
  fvm_ipld_blockstore::MemoryBlockstore,
  fvm_ipld_encoding::{from_slice, to_vec, RawBytes},
  fvm_shared::{
    address::Address,
    bigint::{BigInt, Zero},
    message::Message,
    state::StateTreeVersion,
    version::NetworkVersion,
  },
  serde::{Deserialize, Serialize},
  std::fs,
};

#[derive(Debug, Parser)]
struct Input {
  #[clap(long = "registry")]
  registry: String,

  #[clap(long = "runtime")]
  runtime: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct State {
  empty: bool,
}

fn main() -> Result<()> {
  println!("EVM on FileCoin Testbed");
  let args = Input::parse();

  let mut simulator = Tester::new(
    NetworkVersion::V16,
    StateTreeVersion::V4,
    MemoryBlockstore::default(),
  )?;

  let runtime_actor_address = Address::new_id(10001);
  let runtime_actor_wasm = fs::read(&args.runtime)?;

  let registry_actor_address = Address::new_id(10000);
  let registry_actor_wasm = fs::read(&args.registry)?;

  let sender: [Account; 1] = simulator.create_accounts()?;

  println!("runtime address: {runtime_actor_address}");
  println!("registry address: {registry_actor_address}");

  let empty_state = State { empty: true };
  let state_root = simulator.set_state(&empty_state)?;

  simulator.set_actor_from_bin(
    &registry_actor_wasm,
    state_root,
    registry_actor_address,
    BigInt::zero(),
  )?;

  simulator.set_actor_from_bin(
    &runtime_actor_wasm,
    state_root,
    runtime_actor_address,
    BigInt::zero(),
  )?;

  const INIT_ACTOR_ADDR: Address = Address::new_id(1);

  let runner_thread =
    std::thread::Builder::new()
      .stack_size(64 << 21)
      .spawn(move || {
        simulator.instantiate_machine().unwrap();

        // Send constructor message
        let message = Message {
          from: INIT_ACTOR_ADDR,
          to: registry_actor_address,
          gas_limit: 1000000000,
          method_num: 1, // constructor
          ..Message::default()
        };

        let mut executor = simulator.executor.unwrap();

        let ret = executor
          .execute_message(message, ApplyKind::Implicit, 100)
          .unwrap();

        println!("constructor ret: {ret:#?}");

        let eth_addr = H160::zero();
        let eth_acc = EthereumAccount {
          balance: U256::from(99u64),
          ..Default::default()
        };

        let upsert_params = to_vec(&(eth_addr, eth_acc)).unwrap();

        // send upsert message
        let message = Message {
          from: sender[0].1,
          to: registry_actor_address,
          gas_limit: 1000000000,
          method_num: 3, // upsert
          params: RawBytes::new(upsert_params),
          ..Message::default()
        };

        let ret = executor
          .execute_message(message, ApplyKind::Explicit, 100)
          .unwrap();

        println!("upsert ret: {ret:#?}");

        let retreive_params = to_vec(&eth_addr).unwrap();

        // send retreive message
        let message = Message {
          from: sender[0].1,
          to: registry_actor_address,
          gas_limit: 1000000000,
          method_num: 2, // retreive
          params: RawBytes::new(retreive_params),
          sequence: 1,
          ..Message::default()
        };

        let ret = executor
          .execute_message(message, ApplyKind::Explicit, 100)
          .unwrap();

        println!("retreive ret: {ret:#?}");

        let retreived_account: EthereumAccount =
          from_slice(&ret.msg_receipt.return_data).unwrap();
        println!("retreived eth account: {retreived_account:#?}");
        assert_eq!(99, retreived_account.balance.as_u64());
      })?;

  runner_thread
    .join()
    .expect("can't join simulator runner thread");
  Ok(())
}
