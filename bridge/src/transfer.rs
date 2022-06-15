use {
  fil_actors_runtime::{runtime::Runtime, ActorError},
  fvm_evm::SignedTransaction,
  fvm_ipld_blockstore::Blockstore,
  fvm_ipld_encoding::RawBytes,
};

pub fn transfer_tokens<BS, RT>(
  _rt: &mut RT,
  _tx: SignedTransaction,
) -> Result<RawBytes, ActorError>
where
  BS: Blockstore,
  RT: Runtime<BS>,
{
  todo!()
}
