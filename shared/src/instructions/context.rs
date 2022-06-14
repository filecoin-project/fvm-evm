use {
  crate::{execution::ExecutionState, message::StatusCode, system::System},
  fvm_ipld_blockstore::Blockstore,
};

#[inline]
pub fn blockhash<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline]
pub fn caller<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline]
pub fn call_value<'r, BS: Blockstore>(
  state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  state.stack.push(state.message.value);
  Ok(())
}

#[inline]
pub fn address<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline]
pub fn origin<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline]
pub fn coinbase<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline]
pub fn gas_price<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline]
pub fn timestamp<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline]
pub fn block_number<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline]
pub fn difficulty<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline]
pub fn gas_limit<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline]
pub fn chain_id<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline]
pub fn base_fee<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}
