use {
  crate::{execution::ExecutionState, output::StatusCode, system::System, U256},
  fvm_ipld_blockstore::Blockstore,
  std::num::NonZeroUsize,
};

/// The size of the EVM 256-bit word in bytes.
const WORD_SIZE: usize = 32;

pub struct MemoryRegion {
  pub offset: usize,
  pub size: NonZeroUsize,
}

/// Returns number of words what would fit to provided number of bytes,
/// i.e. it rounds up the number bytes to number of words.
#[inline]
pub fn num_words(size_in_bytes: usize) -> usize {
  (size_in_bytes + (WORD_SIZE - 1)) / WORD_SIZE
}

#[inline]
fn grow_memory(state: &mut ExecutionState, new_size: usize) -> Result<(), ()> {
  let new_words = num_words(new_size);
  let current_words = state.memory.len() / WORD_SIZE;
  let new_cost = 3 * new_words + new_words * new_words / 512;
  let current_cost = 3 * current_words + current_words * current_words / 512;
  let cost = new_cost - current_cost;

  state.gas_left -= cost as i64;

  if state.gas_left < 0 {
    return Err(());
  }

  state.memory.grow((new_words * WORD_SIZE) as usize);

  Ok(())
}

#[inline]
fn get_memory_region_u64(
  state: &mut ExecutionState,
  offset: U256,
  size: NonZeroUsize,
) -> Result<MemoryRegion, ()> {
  if offset > U256::from(u32::MAX) {
    return Err(());
  }

  let new_size = offset.as_usize() + size.get();
  let current_size = state.memory.len();
  if new_size > current_size {
    grow_memory(state, new_size)?;
  }

  Ok(MemoryRegion {
    offset: offset.as_usize(),
    size,
  })
}

#[inline]
pub fn get_memory_region(
  state: &mut ExecutionState,
  offset: U256,
  size: U256,
) -> Result<Option<MemoryRegion>, ()> {
  if size == U256::zero() {
    return Ok(None);
  }

  if size > U256::from(u32::MAX) {
    return Err(());
  }

  get_memory_region_u64(state, offset, NonZeroUsize::new(size.as_usize()).unwrap())
    .map(Some)
}

#[inline]
pub fn mload(state: &mut ExecutionState) -> Result<(), StatusCode> {
  let index = state.stack.pop();

  let region = get_memory_region_u64(state, index, NonZeroUsize::new(WORD_SIZE).unwrap())
    .map_err(|_| StatusCode::OutOfGas)?;
  let value = U256::from_big_endian(
    &state.memory[region.offset..region.offset + region.size.get()],
  );

  state.stack.push(value);

  Ok(())
}

#[inline]
pub fn mstore(state: &mut ExecutionState) -> Result<(), StatusCode> {
  let index = state.stack.pop();
  let value = state.stack.pop();

  let region = get_memory_region_u64(state, index, NonZeroUsize::new(WORD_SIZE).unwrap())
    .map_err(|_| StatusCode::OutOfGas)?;

  let mut bytes = [0u8; WORD_SIZE];
  value.to_big_endian(&mut bytes);
  state.memory[region.offset..region.offset + WORD_SIZE].copy_from_slice(&bytes);

  Ok(())
}

#[inline]
pub fn mstore8(state: &mut ExecutionState) -> Result<(), StatusCode> {
  let index = state.stack.pop();
  let value = state.stack.pop();

  let region = get_memory_region_u64(state, index, NonZeroUsize::new(1).unwrap())
    .map_err(|_| StatusCode::OutOfGas)?;

  let value = (value.low_u32() & 0xff) as u8;

  state.memory[region.offset] = value;

  Ok(())
}

#[inline]
pub fn msize(state: &mut ExecutionState) {
  state
    .stack
    .push(u64::try_from(state.memory.len()).unwrap().into());
}

pub fn extcodecopy<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!();
}
