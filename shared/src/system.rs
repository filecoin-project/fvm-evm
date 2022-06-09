use {
  crate::{
    message::{CreateMessage, Message, Output},
    H160,
    U256,
  },
  bytes::Bytes,
  cid::Cid,
  fil_actors_runtime::runtime::Runtime,
  fvm_ipld_blockstore::Blockstore,
  fvm_ipld_hamt::Hamt,
  fvm_shared::address::Address,
};

/// State access status (EIP-2929).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccessStatus {
  Cold,
  Warm,
}

impl Default for AccessStatus {
  fn default() -> Self {
    Self::Cold
  }
}

#[derive(Clone, Copy, Debug)]
pub enum StorageStatus {
  /// The value of a storage item has been left unchanged: 0 -> 0 and X -> X.
  Unchanged,
  /// The value of a storage item has been modified: X -> Y.
  Modified,
  /// A storage item has been modified after being modified before: X -> Y -> Z.
  ModifiedAgain,
  /// A new storage item has been added: 0 -> X.
  Added,
  /// A storage item has been deleted: X -> 0.
  Deleted,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Call<'a> {
  Call(&'a Message),
  Create(&'a CreateMessage),
}

/// Platform Abstraction Layer
/// that bridges the FVM world to EVM world
pub struct System<'r, BS: Blockstore> {
  _state: Hamt<&'r BS, U256, U256>,
  _bridge: Address,
  _self_address: H160,
}

impl<'r, BS: Blockstore> System<'r, BS> {
  pub fn new<RT: Runtime<BS>>(
    state_cid: Cid,
    runtime: &'r mut RT,
    bridge: Address,
    self_address: H160,
  ) -> anyhow::Result<Self> {
    Ok(Self {
      _bridge: bridge,
      _self_address: self_address,
      _state: Hamt::load(&state_cid, runtime.store())?,
    })
  }
}

impl<'r, BS: Blockstore> System<'r, BS> {
  /// Check if an account exists.
  pub fn account_exists(&self, _address: H160) -> bool {
    todo!()
  }

  /// Get value of a storage key.
  ///
  /// Returns `Ok(U256::zero())` if does not exist.
  pub fn get_storage(&self, _address: H160, _key: U256) -> U256 {
    todo!();
  }

  /// Set value of a storage key.
  pub fn set_storage(
    &mut self,
    _address: H160,
    _key: U256,
    _value: U256,
  ) -> StorageStatus {
    todo!()
  }

  /// Get balance of an account.
  ///
  /// Returns `Ok(0)` if account does not exist.
  pub fn get_balance(&mut self, _address: H160) -> U256 {
    todo!()
  }

  /// Get code size of an account.
  ///
  /// Returns `Ok(0)` if account does not exist.
  pub fn get_code_size(&mut self, _address: H160) -> U256 {
    todo!()
  }

  /// Get code hash of an account.
  ///
  /// Returns `Ok(0)` if account does not exist.
  pub fn get_code_hash(&mut self, _address: H160) -> U256 {
    todo!();
  }

  /// Copy code of an account.
  ///
  /// Returns `Ok(0)` if offset is invalid.
  pub fn copy_code(
    &mut self,
    _address: H160,
    _offset: usize,
    _buffer: &mut [u8],
  ) -> usize {
    todo!()
  }

  /// Self-destruct account.
  pub fn selfdestruct(&mut self, _address: H160, _beneficiary: H160) {
    todo!()
  }

  /// Call to another account.
  pub fn call(&mut self, _msg: Call) -> Output {
    todo!();
  }

  /// Get block hash.
  ///
  /// Returns `Ok(U256::zero())` if block does not exist.
  pub fn get_block_hash(&mut self, _block_number: u64) -> U256 {
    todo!();
  }

  /// Emit a log.
  pub fn emit_log(&mut self, _address: H160, _data: Bytes, _topics: &[U256]) {
    todo!();
  }

  /// Mark account as warm, return previous access status.
  ///
  /// Returns `Ok(AccessStatus::Cold)` if account does not exist.
  pub fn access_account(&mut self, _address: H160) -> AccessStatus {
    todo!();
  }

  /// Mark storage key as warm, return previous access status.
  ///
  /// Returns `Ok(AccessStatus::Cold)` if account does not exist.
  pub fn access_storage(&mut self, _address: H160, _key: U256) -> AccessStatus {
    todo!();
  }
}
