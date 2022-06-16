use {
  crate::{sign_evm_transaction, EVMTester},
  anyhow::Result,
  fvm_evm::{Transaction, TransactionAction},
  fvm_ipld_encoding::RawBytes,
  libsecp256k1::SecretKey,
};

const PROCESS_TRANSACTION_METHOD_NUM: u64 = 2;

#[test]
fn deploy_contract() -> Result<()> {
  let mut tester = EVMTester::new::<1>()?;

  // create the bridge actor and instantiate it with the evm runtime code CID.
  let output = tester.construct_actor(
    EVMTester::BRIDGE_ACTOR_ADDRESS,
    RawBytes::serialize(tester.runtime_code_cid())?,
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
  let signed_tx = sign_evm_transaction(create_tx, seckey);
  let raw_tx = signed_tx.serialize();

  tester.invoke_actor(
    tester.accounts()[0].1,
    EVMTester::BRIDGE_ACTOR_ADDRESS,
    PROCESS_TRANSACTION_METHOD_NUM,
    RawBytes::serialize(raw_tx)?,
  )?;

  Ok(())
}
