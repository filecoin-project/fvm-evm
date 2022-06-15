use {
  crate::{
    construct_actor,
    create_tester,
    invoke_actor,
    sign_transaction,
    BRIDGE_ACTOR_ADDRESS,
  },
  anyhow::Result,
  fvm_evm::{Transaction, TransactionAction},
  fvm_ipld_encoding::RawBytes,
  libsecp256k1::SecretKey,
};

const PROCESS_TRANSACTION_METHOD_NUM: u64 = 2;

#[test]
fn deploy_contract() -> Result<()> {
  let (mut tester, accounts) = create_tester::<1>()?;

  let output = construct_actor(
    &mut tester, //
    BRIDGE_ACTOR_ADDRESS,
    RawBytes::default(),
  )?;

  // bridge constructor does not return anything
  assert_eq!(RawBytes::default(), output);

  let create_tx = Transaction::Legacy {
    chain_id: Some(8889),
    nonce: 1,
    gas_price: 150000000000u64.into(),
    gas_limit: 500000,
    action: TransactionAction::Create,
    value: 0.into(),
    input: hex::decode(include_str!("../contracts/simplecoin.hex"))
      .unwrap()
      .into(),
  };

  let seckey = SecretKey::random(&mut rand::thread_rng());
  let signed_tx = sign_transaction(create_tx, seckey);
  let raw_tx = signed_tx.serialize();

  invoke_actor(
    &mut tester,
    accounts[0].1,
    BRIDGE_ACTOR_ADDRESS,
    PROCESS_TRANSACTION_METHOD_NUM,
    RawBytes::serialize(raw_tx)?,
    0,
  )?;

  Ok(())
}
