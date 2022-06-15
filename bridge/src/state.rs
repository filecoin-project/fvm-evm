use {
  cid::Cid,
  fil_actors_runtime::{runtime::Runtime, ActorError},
  fvm_evm::{EthereumAccount, H160},
  fvm_ipld_blockstore::Blockstore,
  fvm_ipld_hamt::Hamt,
  fvm_sdk::sself,
};

/// Creates the initial bridge accounts map state, including precompiles.
pub fn initialize_bridge_state<BS, RT>(rt: &mut RT) -> Result<Cid, ActorError>
where
  BS: Blockstore,
  RT: Runtime<BS>,
{
  Hamt::<_, EthereumAccount, H160>::new(rt.store())
    .flush()
    .map_err(|e| ActorError::illegal_state(format!("Failed to create empty map: {e}")))
}

pub fn load_bridge_state<BS, RT>(
  rt: &RT,
) -> Result<Hamt<&BS, EthereumAccount, H160>, ActorError>
where
  BS: Blockstore,
  RT: Runtime<BS>,
{
  Hamt::<_, EthereumAccount, H160>::load(
    &sself::root().map_err(|e| ActorError::not_found(e.to_string()))?,
    rt.store(),
  )
  .map_err(|e| ActorError::illegal_state(e.to_string()))
}
