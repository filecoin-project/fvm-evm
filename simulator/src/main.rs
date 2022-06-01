use {
  anyhow::Result,
  clap::Parser,
  fvm::executor::{ApplyKind, Executor},
  fvm_integration_tests::tester::{Account, Tester},
  fvm_ipld_blockstore::{Blockstore, MemoryBlockstore},
  fvm_shared::{
    address::Address,
    message::Message,
    state::StateTreeVersion,
    version::NetworkVersion,
  },
};
#[derive(Debug, Parser)]
struct Input {
  #[clap(long = "registry")]
  registry: String,

  #[clap(long = "runtime")]
  runtime: String,
}

fn configure_registry_actor<B: Blockstore>(
  args: &Input,
  _simulator: &mut Tester<B>,
) -> Result<Address> {
  let _registry_wasm = std::fs::read(&args.registry)?;
  let actor_address = Address::new_id(10000);

  Ok(actor_address)
}

fn main() -> Result<()> {
  println!("EVM on FileCoin Testbed");
  let args = Input::parse();

  let mut simulator = Tester::new(
    NetworkVersion::V16,
    StateTreeVersion::V4,
    MemoryBlockstore::default(),
  )?;

  let registry_actor_address = configure_registry_actor(&args, &mut simulator)?;

  let sender: [Account; 1] = simulator.create_accounts()?;

  simulator.instantiate_machine()?;

  // Send message
  let message = Message {
    from: sender[0].1,
    to: registry_actor_address,
    gas_limit: 1000000000,
    method_num: 1,
    ..Message::default()
  };

  let ret = simulator.executor.unwrap().execute_message(
    message,
    ApplyKind::Explicit,
    100,
  )?;

  println!("ret: {ret:?}");

  Ok(())
}
