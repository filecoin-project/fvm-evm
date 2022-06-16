use {
  crate::EVMTester,
  anyhow::Result,
  fvm_evm::{EthereumAccount, H160, U256},
  fvm_ipld_encoding::{from_slice, to_vec, RawBytes},
};

#[test]
#[ignore]
fn cross_contract_smoke() -> Result<()> {
  const RETREIVE_METHOD_NUM: u64 = 2;
  const CONSTRUCT_ZERO_ACCOUNT_NUM: u64 = 2;

  let mut tester = EVMTester::new::<1>()?;

  // construct registry
  let output =
    tester.construct_actor(EVMTester::BRIDGE_ACTOR_ADDRESS, RawBytes::default())?;

  // registry constructor does not return anything
  assert_eq!(RawBytes::default(), output);

  let runtime_params = (vec![1u8; 2], EVMTester::BRIDGE_ACTOR_ADDRESS);

  let output = tester.construct_actor(
    EVMTester::RUNTIME_ACTOR_ADDRESS,
    RawBytes::new(to_vec(&runtime_params)?),
  )?;

  // runtime constructor returns nothing
  assert_eq!(RawBytes::default(), output);

  // make sure that the zero address is empty
  // before invoking the cross-contract call
  let eth_account = from_slice(&tester.invoke_actor(
    tester.accounts()[0].1,
    EVMTester::BRIDGE_ACTOR_ADDRESS,
    RETREIVE_METHOD_NUM,
    RawBytes::serialize(H160::zero())?,
  )?)?;

  // not present, should return a synthesized empty/unused account
  assert_eq!(EthereumAccount::default(), eth_account);

  // invoke a runtime method that invokes
  // the registry and creates a new entry
  // for the zero address.
  let output = tester.invoke_actor(
    tester.accounts()[0].1,
    EVMTester::RUNTIME_ACTOR_ADDRESS,
    CONSTRUCT_ZERO_ACCOUNT_NUM,
    RawBytes::default(),
  )?;
  assert_eq!(RawBytes::default(), output);

  // now query again, it should have the inserted value
  let eth_account = from_slice(&tester.invoke_actor(
    tester.accounts()[0].1,
    EVMTester::BRIDGE_ACTOR_ADDRESS,
    RETREIVE_METHOD_NUM,
    RawBytes::serialize(H160::zero())?,
  )?)?;

  assert_ne!(EthereumAccount::default(), eth_account);
  assert_eq!(5, eth_account.nonce);
  assert_eq!(U256::from(999), eth_account.balance);

  Ok(())
}
