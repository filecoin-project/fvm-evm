use {
  crate::{abort::abort, storage::Blockstore},
  cid::{multihash::Code, Cid},
  fvm_ipld_encoding::{
    to_vec,
    tuple::{Deserialize_tuple, Serialize_tuple},
    CborStore,
    DAG_CBOR,
  },
  fvm_sdk::{ipld, sself},
};

/// The state object.
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, Default)]
pub struct State {
  pub count: u64,
}

/// We should probably have a derive macro to mark an object as a state object,
/// and have load and save methods automatically generated for them as part of a
/// StateObject trait (i.e. impl StateObject for State).
impl State {
  pub fn load() -> Self {
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
