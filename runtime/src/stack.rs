use {arrayvec::ArrayVec, fvm_evm::U256, serde::Serialize};

/// Ethereum Yellow Paper (9.1)
pub const MAX_STACK_SIZE: usize = 1024;

/// EVM stack.
#[derive(Clone, Debug, Default, Serialize)]
pub struct Stack(pub ArrayVec<U256, MAX_STACK_SIZE>);

impl Stack {
  #[inline]
  pub const fn new() -> Self {
    Self(ArrayVec::new_const())
  }

  #[inline]
  const fn get_pos(&self, pos: usize) -> usize {
    self.len() - 1 - pos
  }

  #[inline]
  pub fn get(&self, pos: usize) -> &U256 {
    let pos = self.get_pos(pos);
    self.0.get(pos).unwrap()
  }

  #[inline]
  pub fn get_mut(&mut self, pos: usize) -> &mut U256 {
    let pos = self.get_pos(pos);
    self.0.get_mut(pos).unwrap()
  }

  #[inline(always)]
  pub const fn len(&self) -> usize {
    self.0.len()
  }

  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  #[inline]
  pub fn push(&mut self, v: U256) {
    self.0.push(v)
  }

  #[inline]
  pub fn pop(&mut self) -> U256 {
    self.0.pop().unwrap()
  }

  #[inline]
  pub fn swap_top(&mut self, pos: usize) {
    let top = self.0.len() - 1;
    let pos = self.get_pos(pos);
    self.0.swap(top, pos);
  }
}
