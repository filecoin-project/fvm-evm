use {
  crate::{
    construct_actor,
    create_tester,
    invoke_actor,
    REGISTRY_ACTOR_ADDRESS,
    RUNTIME_ACTOR_ADDRESS,
  },
  anyhow::Result,
  fvm_evm::{EthereumAccount, H160, U256},
  fvm_ipld_encoding::{from_slice, to_vec, RawBytes},
};

#[test]
fn cross_contract_smoke() -> Result<()> {
  const RETREIVE_METHOD_NUM: u64 = 2;
  const CONSTRUCT_ZERO_ACCOUNT_NUM: u64 = 2;

  let (mut tester, accounts) = create_tester::<1>()?;

  // construct registry
  let output = construct_actor(
    &mut tester, //
    REGISTRY_ACTOR_ADDRESS,
    RawBytes::default(),
  )?;

  // registry constructor does not return anything
  assert_eq!(RawBytes::default(), output);

  let runtime_params = (vec![1u8; 2], REGISTRY_ACTOR_ADDRESS);

  let output = construct_actor(
    &mut tester, //
    RUNTIME_ACTOR_ADDRESS,
    RawBytes::new(to_vec(&runtime_params)?),
  )?;

  // runtime constructor returns nothing
  assert_eq!(RawBytes::default(), output);

  // make sure that the zero address is empty
  // before invoking the cross-contract call
  let eth_account = from_slice(&invoke_actor(
    &mut tester,
    accounts[0].1,
    REGISTRY_ACTOR_ADDRESS,
    RETREIVE_METHOD_NUM,
    RawBytes::serialize(H160::zero())?,
    0,
  )?)?;

  // not present, should return a synthesized empty/unused account
  assert_eq!(EthereumAccount::default(), eth_account);

  // invoke a runtime method that invokes
  // the registry and creates a new entry
  // for the zero address.
  let output = invoke_actor(
    &mut tester,
    accounts[0].1,
    RUNTIME_ACTOR_ADDRESS,
    CONSTRUCT_ZERO_ACCOUNT_NUM,
    RawBytes::default(),
    1,
  )?;
  assert_eq!(RawBytes::default(), output);

  // now query again, it should have the inserted value
  let eth_account = from_slice(&invoke_actor(
    &mut tester,
    accounts[0].1,
    REGISTRY_ACTOR_ADDRESS,
    RETREIVE_METHOD_NUM,
    RawBytes::serialize(H160::zero())?,
    2,
  )?)?;

  assert_ne!(EthereumAccount::default(), eth_account);
  assert_eq!(5, eth_account.nonce);
  assert_eq!(U256::from(999), eth_account.balance);

  Ok(())
}
