use {
  crate::{execution::ExecutionState, message::StatusCode, platform::Platform},
  fvm_evm::{H160, H256, U256},
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
pub fn sload(state: &mut ExecutionState, platform: &Platform) -> Result<(), StatusCode> {
  todo!();
}

#[inline]
pub fn sstore(state: &mut ExecutionState, platform: &Platform) -> Result<(), StatusCode> {
  todo!();
}

#[inline]
pub fn balance(
  state: &mut ExecutionState,
  platform: &Platform,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline]
pub fn selfbalance(
  state: &mut ExecutionState,
  platform: &Platform,
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
pub fn extcodesize(
  state: &mut ExecutionState,
  platform: &Platform,
) -> Result<(), StatusCode> {
  todo!()
}

pub fn extcodehash(
  state: &mut ExecutionState,
  platform: &Platform,
) -> Result<(), StatusCode> {
  todo!();
}

#[inline]
pub fn create(
  state: &mut ExecutionState,
  platform: &Platform,
  create2: bool,
) -> Result<(), StatusCode> {
  todo!()
}

#[inline]
pub fn selfdestruct(
  state: &mut ExecutionState,
  platform: &Platform,
) -> Result<(), StatusCode> {
  todo!()
}
