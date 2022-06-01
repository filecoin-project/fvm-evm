use {
  crate::store::Blockstore,
  cid::Cid,
  fvm_evm::{abort, U256},
  fvm_ipld_encoding::{to_vec, Cbor, CborStore, DAG_CBOR},
  fvm_ipld_hamt::Hamt,
  fvm_sdk::{ipld, sself},
  multihash::Code,
  serde_tuple::{Deserialize_tuple, Serialize_tuple},
};

/// Data stored by an EVM contract.
/// This runs on the fvm-evm-runtime actor code cid.
#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct ContractState {
  /// The EVM contract bytecode resulting from calling the
  /// initialization code by the constructor.
  bytecode: Cid,

  /// The EVM contract state dictionary.
  /// All eth contract state is a map of U256 -> U256 values.
  ///
  /// HAMT<U256, U256>
  state: Cid,
}
impl Cbor for ContractState {}

impl ContractState {
  /// Called by the actor constructor during the creation of a new
  /// EVM contract. This method will execute the initialization code
  /// and store the contract bytecode, and the EVM constructor state
  /// in the state HAMT.
  pub fn new(bytecode: &impl AsRef<[u8]>) -> Self {
    let bytecode_cid = match ipld::put(
      Code::Blake2b256.into(),
      32,
      DAG_CBOR,
      to_vec(bytecode.as_ref()).unwrap().as_slice(),
    ) {
      Ok(cid) => cid,
      Err(err) => abort!(
        USR_SERIALIZATION,
        "failed to store EVM contract bytecode: {err}"
      ),
    };

    let state_cid =
      match Hamt::<Blockstore, U256, U256>::new(Blockstore).flush() {
        Ok(cid) => cid,
        Err(err) => abort!(
          USR_SERIALIZATION,
          "failed to initialize EVM contract state HAMT: {err}"
        ),
      };

    let this = Self {
      bytecode: bytecode_cid,
      state: state_cid,
    };

    let serialized = match to_vec(&this) {
      Ok(s) => s,
      Err(err) => abort!(
        USR_SERIALIZATION,
        "failed to serialize initial state: {err}"
      ),
    };

    let root_cid = match ipld::put(
      Code::Blake2b256.into(),
      32,
      DAG_CBOR,
      serialized.as_slice(),
    ) {
      Ok(cid) => cid,
      Err(err) => {
        abort!(USR_SERIALIZATION, "failed to store initial state: {err}")
      }
    };

    if let Err(err) = sself::set_root(&root_cid) {
      abort!(USR_ILLEGAL_STATE, "failed to initialize state root: {err}");
    }

    this
  }

  pub fn _load() -> Self {
    // First, load the current state root.
    let root = match sself::root() {
      Ok(root) => root,
      Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get root: {:?}", err),
    };

    // Load the actor state from the state tree.
    match Blockstore.get_cbor::<Self>(&root) {
      Ok(Some(state)) => state,
      Ok(None) => abort!(USR_ILLEGAL_STATE, "state does not exist"),
      Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get state: {}", err),
    }
  }

  pub fn save(&self) -> Cid {
    let serialized = match to_vec(self) {
      Ok(s) => s,
      Err(err) => {
        abort!(USR_SERIALIZATION, "failed to serialize state: {:?}", err)
      }
    };
    let cid = match ipld::put(
      Code::Blake2b256.into(),
      32,
      DAG_CBOR,
      serialized.as_slice(),
    ) {
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
