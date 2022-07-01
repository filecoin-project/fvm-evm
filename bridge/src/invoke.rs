use {
  crate::state::BridgeState,
  fil_actors_runtime::{runtime::Runtime, ActorError},
  fvm_evm::{EthereumAccount, SignedTransaction, TransactionAction},
  fvm_ipld_blockstore::Blockstore,
  fvm_ipld_encoding::RawBytes,
};

pub fn invoke_contract<BS, RT>(
  rt: &mut RT,
  tx: SignedTransaction,
) -> anyhow::Result<RawBytes>
where
  BS: Blockstore,
  RT: Runtime<BS>,
{
  if let TransactionAction::Call(address) = tx.action() {
    fvm_sdk::debug::log(format!("invoking contract at {address:?}"));

    let bridge_state = BridgeState::load(rt)?;
    let accounts = bridge_state.accounts(rt)?;

    if let Some(account) = accounts.get(&address)? {
      let account: &EthereumAccount = account;
      Ok(RawBytes::serialize(account.balance)?)
    } else {
      return Err(anyhow::anyhow!(ActorError::not_found(format!(
        "contract {address} not found"
      ))));
    }
  } else {
    unreachable!("Create transactions should never arrive here")
  }
}
