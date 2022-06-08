use {
  cid::Cid,
  fvm_evm::{abort, H160, U256},
  fvm_ipld_blockstore::Blockstore,
  fvm_ipld_encoding::{to_vec, Cbor, RawBytes, DAG_CBOR},
  fvm_ipld_hamt::Hamt,
  fvm_sdk::{ipld, sself},
  fvm_shared::address::Address,
  multihash::Code,
  serde_tuple::{Deserialize_tuple, Serialize_tuple},
};

/// Data stored by an EVM contract.
/// This runs on the fvm-evm-runtime actor code cid.
#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct ContractState {
  /// Address of the registry actor that stores EVM <--> FVM
  /// account mappings.
  pub registry: Address,

  /// The EVM contract bytecode resulting from calling the
  /// initialization code by the constructor.
  pub bytecode: Cid,

  /// The EVM contract state dictionary.
  /// All eth contract state is a map of U256 -> U256 values.
  ///
  /// HAMT<U256, U256>
  pub state: Cid,

  /// EVM address of the current contract
  pub self_address: H160,
}

impl Cbor for ContractState {}

impl ContractState {
  /// Called by the actor constructor during the creation of a new
  /// EVM contract. This method will execute the initialization code
  /// and store the contract bytecode, and the EVM constructor state
  /// in the state HAMT.
  pub fn new<'r, BS: Blockstore>(
    bytecode: &(impl AsRef<[u8]> + ?Sized),
    registry: Address,
    blockstore: BS,
    self_address: H160,
  ) -> anyhow::Result<Self> {
    let this = Self {
      registry,
      self_address,
      bytecode: ipld::put(
        Code::Blake2b256.into(),
        32,
        DAG_CBOR,
        &RawBytes::serialize(bytecode.as_ref())?,
      )?,
      state: Hamt::<_, U256, U256>::new(&blockstore).flush()?,
    };

    sself::set_root(&ipld::put(
      Code::Blake2b256.into(),
      32,
      DAG_CBOR,
      &RawBytes::serialize(&this)?,
    )?)?;
    Ok(this)
  }

  pub fn _save(&self) -> Cid {
    let serialized = match to_vec(self) {
      Ok(s) => s,
      Err(err) => {
        abort!(USR_SERIALIZATION, "failed to serialize state: {:?}", err)
      }
    };
    let cid =
      match ipld::put(Code::Blake2b256.into(), 32, DAG_CBOR, serialized.as_slice()) {
        Ok(cid) => cid,
        Err(err) => {
          abort!(USR_SERIALIZATION, "failed to store initial state: {:}", err)
        }
      };
    if let Err(err) = sself::set_root(&cid) {
      abort!(USR_ILLEGAL_STATE, "failed to set root ciid: {:}", err);
    }
    cid
  }
}
