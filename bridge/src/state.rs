use {
  anyhow::Context,
  cid::Cid,
  fil_actors_runtime::runtime::Runtime,
  fvm_evm::{EthereumAccount, H160},
  fvm_ipld_blockstore::Blockstore,
  fvm_ipld_encoding::{to_vec, Cbor, CborStore, DAG_CBOR},
  fvm_ipld_hamt::Hamt,
  fvm_sdk::{ipld, sself},
  multihash::Code,
  serde_tuple::{Deserialize_tuple, Serialize_tuple},
};

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct BridgeState {
  /// Populated during construction
  runtime_cid: Cid,

  /// Hamt H160 -> EthereumAccount
  accounts: Cid,
}

impl Cbor for BridgeState {}

impl BridgeState {
  pub fn create<BS, RT>(rt: &RT, runtime_cid: &Cid) -> anyhow::Result<(Self, Cid)>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    let instance = BridgeState {
      runtime_cid: *runtime_cid,
      accounts: Hamt::<_, EthereumAccount, H160>::new(rt.store()).flush()?,
    };

    let serialized = to_vec(&instance)?;
    let cid = ipld::put(Code::Blake2b256.into(), 32, DAG_CBOR, serialized.as_slice())?;
    sself::set_root(&cid)?;

    Ok((instance, cid))
  }

  pub fn load<BS, RT>(rt: &RT) -> anyhow::Result<Self>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    rt.store()
      .get_cbor(&sself::root()?)?
      .context("bridge state not initialized")
  }

  pub fn accounts<'r, BS, RT>(
    &self,
    rt: &'r RT,
  ) -> anyhow::Result<Hamt<&'r BS, EthereumAccount, H160>>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    Ok(Hamt::<_, EthereumAccount, H160>::load(
      &self.accounts,
      rt.store(),
    )?)
  }

  pub fn runtime_cid(&self) -> &Cid {
    &self.runtime_cid
  }

  pub fn update_accounts<BS: Blockstore>(
    &mut self,
    accounts: &mut Hamt<BS, EthereumAccount, H160>,
  ) -> anyhow::Result<()> {
    self.accounts = accounts.flush()?;
    Ok(sself::set_root(&ipld::put(
      Code::Blake2b256.into(),
      32,
      DAG_CBOR,
      &to_vec(self)?,
    )?)?)
  }
}
