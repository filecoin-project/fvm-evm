use {
  crate::{sign_evm_transaction, EVMTester},
  anyhow::Result,
  fvm_evm::{SignedTransaction, Transaction, TransactionAction, H160, U256},
  fvm_ipld_encoding::{from_slice, RawBytes},
  libsecp256k1::SecretKey,
  sha3::{Digest, Keccak256},
};

const PROCESS_TRANSACTION_METHOD_NUM: u64 = 2;

fn extract_sender(tx: &SignedTransaction) -> H160 {
  let mut sig = [0u8; 65];
  sig[..32].copy_from_slice(tx.signature.r.as_bytes());
  sig[32..64].copy_from_slice(tx.signature.s.as_bytes());

  if matches!(tx.transaction, Transaction::Legacy { .. }) {
    sig[64] = tx.signature.v.odd_y_parity();
  } else {
    sig[64] = tx.signature.v.0 as u8;
  }

  let pubkey = fvm_shared::crypto::signature::ops::recover_secp_public_key(
    &tx.hash().to_fixed_bytes(),
    &sig,
  )
  .unwrap()
  .serialize();

  let address_slice = &Keccak256::digest(&pubkey[1..])[12..];
  H160::from_slice(address_slice)
}

#[test]
fn deploy_and_invoke_contract() -> Result<()> {
  pretty_env_logger::init();

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
  let sender_address = extract_sender(&signed_tx);
  let raw_tx = signed_tx.serialize();

  let contract_address: H160 = from_slice(
    // deploy actor and get the address it was stored at
    &tester.invoke_actor(
      tester.accounts()[0].1,
      EVMTester::BRIDGE_ACTOR_ADDRESS,
      PROCESS_TRANSACTION_METHOD_NUM,
      RawBytes::serialize(raw_tx)?,
    )?,
  )?;

  println!("Contract deployed at: {contract_address:?}");
  println!("Sender address: {sender_address:?}");
  println!();

  // solidity coding covention, calling getBalance with sender address
  // should 10000, because it is set in the constructor

  let mut solidity_params = vec![];
  solidity_params.append(&mut hex::decode("f8b2cb4f").unwrap()); // function selector
  let mut arg0 = vec![0u8; 12];
  arg0.append(&mut sender_address.to_fixed_bytes().to_vec());
  solidity_params.append(&mut arg0);

  let invoke_tx = Transaction::Legacy {
    chain_id: Some(8889),
    nonce: 2,
    gas_price: 150000000000u64.into(),
    gas_limit: 500000,
    action: TransactionAction::Call(contract_address),
    value: 0.into(),
    input: solidity_params.into(),
  };

  let signed_tx = sign_evm_transaction(invoke_tx, seckey);
  let raw_tx = signed_tx.serialize();

  let balance_output: U256 = from_slice(
    // invoke getBalance(sender)
    &tester.invoke_actor(
      tester.accounts()[0].1,
      EVMTester::BRIDGE_ACTOR_ADDRESS,
      PROCESS_TRANSACTION_METHOD_NUM,
      RawBytes::serialize(raw_tx)?,
    )?,
  )?;

  println!("balance output: {balance_output:?}");

  Ok(())
}
