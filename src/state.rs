use {
  crate::{abort::abort, storage::Blockstore},
  cid::{multihash::Code, Cid},
  ethereum_types::U256,
  fvm_ipld_encoding::{to_vec, Cbor, CborStore, DAG_CBOR},
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
  pub balance: U256,

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
  pub code_hash: U256,

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
  pub storage_root: U256,
}

/// The state object.
#[derive(Debug, Default, Serialize_tuple, Deserialize_tuple)]
pub struct State {
  /// The state of all account headers of the system
  ///
  /// Hamt<Address, Account>
  pub accounts: Cid,

  /// For contract accounts, this map holds the EVM bytecode of
  /// that contract. This value corresponds to the [`code_hash`] field
  /// defined in the account header.
  ///
  /// The value is a CID that points to the EVM bytecode blob.
  ///
  /// Hamt<Cid, RawBytes>
  pub bytecodes: Cid,

  /// State held by individual contracts.
  ///
  /// The key is the address of the contracts that owns the state.
  /// The value is a map of 256-bit int index and a 256-bit value.
  ///
  /// Hamt<H160, Hamt<U256, U256>>
  pub contracts_state: Cid,
}

impl Cbor for State {}

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
