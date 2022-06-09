use {
  crate::{execution::ExecutionState, message::StatusCode, system::System, H160, U256},
  fvm_ipld_blockstore::Blockstore,
};

#[inline]
fn u256_to_address(v: U256) -> H160 {
  let mut bytes = [0u8; 32];
  v.to_big_endian(&mut bytes);
  H160::from_slice(&bytes)
}

#[inline]
fn address_to_u256(v: H160) -> U256 {
  U256::from_big_endian(v.as_bytes())
}

#[inline]
pub fn sload<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!();
}

#[inline]
pub fn sstore<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!();
}

#[inline]
pub fn balance<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline]
pub fn selfbalance<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline(always)]
fn ok_or_out_of_gas(gas_left: i64) -> Result<(), StatusCode> {
  match gas_left >= 0 {
    true => Ok(()),
    false => Err(StatusCode::OutOfGas),
  }
}

#[inline]
pub fn extcodesize<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}

pub fn extcodehash<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!();
}

#[inline]
pub fn create<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
  _create2: bool,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline]
pub fn selfdestruct<'r, BS: Blockstore>(
  _state: &mut ExecutionState,
  _platform: &'r System<'r, BS>,
) -> Result<(), StatusCode> {
  todo!()
}
