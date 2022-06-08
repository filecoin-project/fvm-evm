use {
  crate::message::{CreateMessage, Message, Output},
  bytes::Bytes,
  cid::Cid,
  fil_actors_runtime::runtime::Runtime,
  fvm_evm::{H160, U256},
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
  state: Hamt<&'r BS, U256, U256>,
  registry: Address,
}

impl<'r, BS: Blockstore> System<'r, BS> {
  pub fn new<RT: Runtime<BS>>(
    state_cid: Cid,
    runtime: &'r mut RT,
    registry: Address,
    self_address: H160,
  ) -> anyhow::Result<Self> {
    Ok(Self {
      registry,
      state: Hamt::load(&state_cid, runtime.store())?,
    })
  }
}

impl<'r, BS: Blockstore> System<'r, BS> {
  /// Check if an account exists.
  pub fn account_exists(&self, address: H160) -> bool {
    todo!()
  }

  /// Get value of a storage key.
  ///
  /// Returns `Ok(U256::zero())` if does not exist.
  pub fn get_storage(&self, address: H160, key: U256) -> U256 {
    todo!();
  }

  /// Set value of a storage key.
  pub fn set_storage(&mut self, address: H160, key: U256, value: U256) -> StorageStatus {
    todo!()
  }

  /// Get balance of an account.
  ///
  /// Returns `Ok(0)` if account does not exist.
  pub fn get_balance(&mut self, address: H160) -> U256 {
    todo!()
  }

  /// Get code size of an account.
  ///
  /// Returns `Ok(0)` if account does not exist.
  pub fn get_code_size(&mut self, address: H160) -> U256 {
    todo!()
  }

  /// Get code hash of an account.
  ///
  /// Returns `Ok(0)` if account does not exist.
  pub fn get_code_hash(&mut self, address: H160) -> U256 {
    todo!();
  }

  /// Copy code of an account.
  ///
  /// Returns `Ok(0)` if offset is invalid.
  pub fn copy_code(&mut self, address: H160, offset: usize, buffer: &mut [u8]) -> usize {
    todo!()
  }

  /// Self-destruct account.
  pub fn selfdestruct(&mut self, address: H160, beneficiary: H160) {
    todo!()
  }

  /// Call to another account.
  pub fn call(&mut self, msg: Call) -> Output {
    todo!();
  }

  /// Get block hash.
  ///
  /// Returns `Ok(U256::zero())` if block does not exist.
  pub fn get_block_hash(&mut self, block_number: u64) -> U256 {
    todo!();
  }

  /// Emit a log.
  pub fn emit_log(&mut self, address: H160, data: Bytes, topics: &[U256]) {
    todo!();
  }

  /// Mark account as warm, return previous access status.
  ///
  /// Returns `Ok(AccessStatus::Cold)` if account does not exist.
  pub fn access_account(&mut self, address: H160) -> AccessStatus {
    todo!();
  }

  /// Mark storage key as warm, return previous access status.
  ///
  /// Returns `Ok(AccessStatus::Cold)` if account does not exist.
  pub fn access_storage(&mut self, address: H160, key: U256) -> AccessStatus {
    todo!();
  }
}
