use {
  crate::{construct_actor, create_tester, invoke_actor, BRIDGE_ACTOR_ADDRESS},
  anyhow::Result,
  fvm_evm::{EthereumAccount, H160, U256},
  fvm_ipld_encoding::{from_slice, RawBytes},
};

#[test]
fn bridge_smoke() -> Result<()> {
  let (mut tester, accounts) = create_tester::<1>()?;

  let output = construct_actor(
    &mut tester, //
    BRIDGE_ACTOR_ADDRESS,
    RawBytes::default(),
  )?;

  // registry constructor does not return anything
  assert_eq!(RawBytes::default(), output);

  const RETREIVE_METHOD_NUM: u64 = 2;
  const UPSERT_METHOD_NUM: u64 = 3;

  let eth_account = from_slice(&invoke_actor(
    &mut tester,
    accounts[0].1,
    BRIDGE_ACTOR_ADDRESS,
    RETREIVE_METHOD_NUM,
    RawBytes::serialize(H160::zero())?,
    0,
  )?)?;

  // not present, should return a synthesized empty/unused account
  assert_eq!(EthereumAccount::default(), eth_account);

  // now insert new account with nonce 3 and balance 99
  // at address zero

  let eth_addr = H160::zero();
  let eth_acc = EthereumAccount {
    nonce: 3,
    balance: U256::from(99u64),
    ..Default::default()
  };

  let ret = invoke_actor(
    &mut tester,
    accounts[0].1,
    BRIDGE_ACTOR_ADDRESS,
    UPSERT_METHOD_NUM,
    RawBytes::serialize(&(eth_addr, eth_acc))?,
    1,
  )?;

  // upsert method does not return anything
  assert_eq!(RawBytes::default(), ret);

  // now query again, it should have the inserted value
  let eth_account = from_slice(&invoke_actor(
    &mut tester,
    accounts[0].1,
    BRIDGE_ACTOR_ADDRESS,
    RETREIVE_METHOD_NUM,
    RawBytes::serialize(H160::zero())?,
    2,
  )?)?;

  assert_ne!(EthereumAccount::default(), eth_account);
  assert_eq!(3, eth_account.nonce);
  assert_eq!(U256::from(99), eth_account.balance);

  Ok(())
}
