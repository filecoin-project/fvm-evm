use {
  crate::{abort::abort, storage::Blockstore},
  cid::{multihash::Code, Cid},
  fvm_ipld_encoding::{to_vec, Cbor, CborStore, DAG_CBOR},
  fvm_ipld_hamt::Hamt,
  fvm_sdk::{ipld, sself},
  serde_tuple::{Deserialize_tuple, Serialize_tuple},
};

/// Represents the data held by a single account.
/// It could be either an externally owned account or a contract account.
#[derive(PartialEq, Debug, Serialize_tuple, Deserialize_tuple)]
pub struct Account {
  /// A counter that indicates the number of transactions sent from the
  /// account. This ensures transactions are only processed once.
  /// In a contract account, this number represents the number of contracts
  /// created by the account.
  pub nonce: u64,

  /// The number of wei owned by this address.
  /// Wei is a denomination of ETH and there are 1e+18 wei per ETH.
  pub balance: u64,

  /// This hash refers to the code of an account on the Ethereum virtual
  /// machine (EVM). Contract accounts have code fragments programmed in that
  /// can perform different operations. This EVM code gets executed if the
  /// account gets a message call.
  ///
  /// It cannot be changed, unlike the other account fields.
  /// All such code fragments are contained in the state database under their
  /// corresponding hashes for later retrieval. This hash value is known as a
  /// codeHash.
  ///
  /// For externally owned accounts, the codeHash field is the hash of an empty
  /// string.
  pub code_hash: u64,

  /// Also known as a storage hash.
  ///
  /// A 256-bit hash of the root node of a Merkle Patricia trie that encodes
  /// the storage contents of the account (a mapping between 256-bit integer
  /// values), encoded into the trie as a mapping from the Keccak 256-bit
  /// hash of the 256-bit integer keys to the RLP-encoded 256-bit integer
  /// values.
  ///
  /// This trie encodes the hash of the storage contents of this account,
  /// and is empty by default.
  pub storage_root: u64,
}

/// The state object.
#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct EVMContractState {
  bytecode: Cid,
  state: Cid,
}

impl Cbor for EVMContractState {}

impl EVMContractState {
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

    let mut state_dict = Hamt::<Blockstore, String, String>::new(Blockstore);

    if let Err(err) =
      state_dict.set("length".to_string(), bytecode.as_ref().len().to_string())
    {
      abort!(
        USR_SERIALIZATION,
        "failed to write initial state in state HAMT: {err}"
      )
    }

    let state_cid = match state_dict.flush() {
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

  pub fn get_bytecode(&self) -> anyhow::Result<Option<Vec<u8>>> {
    Blockstore.get_cbor::<Vec<u8>>(&self.bytecode)
  }

  pub fn get_length(&self) -> Result<Option<u32>, fvm_ipld_hamt::Error> {
    let dict =
      Hamt::<Blockstore, String, String>::load(&self.state, Blockstore)?;
    dict
      .get("length")
      .map(|op| op.map(|s| s.parse::<u32>().unwrap()))
  }
}
