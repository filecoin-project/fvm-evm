use {
  crate::{execution::ExecutionState, message::StatusCode, stack::Stack},
  fvm_evm::{U256, U512},
};

#[inline]
pub fn add(stack: &mut Stack) {
  let a = stack.pop();
  let b = stack.pop();
  stack.push(a.overflowing_add(b).0);
}

#[inline]
pub fn mul(stack: &mut Stack) {
  let a = stack.pop();
  let b = stack.pop();
  stack.push(a.overflowing_mul(b).0);
}

#[inline]
pub fn sub(stack: &mut Stack) {
  let a = stack.pop();
  let b = stack.pop();
  stack.push(a.overflowing_sub(b).0);
}

#[inline]
pub fn div(stack: &mut Stack) {
  let a = stack.pop();
  let b = stack.get_mut(0);
  if *b == U256::zero() {
    *b = U256::zero()
  } else {
    *b = a / *b
  }
}

#[inline]
pub fn sdiv(stack: &mut Stack) {
  let a = stack.pop();
  let b = stack.pop();
  let v = i256_div(a, b);
  stack.push(v);
}

#[inline]
pub fn modulo(stack: &mut Stack) {
  let a = stack.pop();
  let b = stack.get_mut(0);
  *b = if *b == U256::zero() {
    U256::zero()
  } else {
    a % *b
  };
}

#[inline]
pub fn smod(stack: &mut Stack) {
  let a = stack.pop();
  let b = stack.get_mut(0);

  if *b == U256::zero() {
    *b = U256::zero()
  } else {
    *b = i256_mod(a, *b);
  };
}

#[inline]
pub fn addmod(stack: &mut Stack) {
  let a = stack.pop();
  let b = stack.pop();
  let c = stack.pop();

  let v = if c == U256::zero() {
    U256::zero()
  } else {
    let mut a_be = [0u8; 32];
    let mut b_be = [0u8; 32];
    let mut c_be = [0u8; 32];

    a.to_big_endian(&mut a_be);
    b.to_big_endian(&mut b_be);
    c.to_big_endian(&mut c_be);

    let a = U512::from_big_endian(&a_be);
    let b = U512::from_big_endian(&b_be);
    let c = U512::from_big_endian(&c_be);

    let v = a + b % c;
    let mut v_be = [0u8; 64];
    v.to_big_endian(&mut v_be);
    U256::from_big_endian(&v_be)
  };

  stack.push(v);
}

#[inline]
pub fn mulmod(stack: &mut Stack) {
  let a = stack.pop();
  let b = stack.pop();
  let c = stack.pop();

  let v = if c == U256::zero() {
    U256::zero()
  } else {
    let mut a_be = [0u8; 32];
    let mut b_be = [0u8; 32];
    let mut c_be = [0u8; 32];

    a.to_big_endian(&mut a_be);
    b.to_big_endian(&mut b_be);
    c.to_big_endian(&mut c_be);

    let a = U512::from_big_endian(&a_be);
    let b = U512::from_big_endian(&b_be);
    let c = U512::from_big_endian(&c_be);

    let v = a * b % c;
    let mut v_be = [0u8; 64];
    v.to_big_endian(&mut v_be);
    U256::from_big_endian(&v_be)
  };

  stack.push(v);
}

#[inline]
pub fn signextend(stack: &mut Stack) {
  let a = stack.pop();
  let b = stack.get_mut(0);

  if a < U256::from(32) {
    let bit_index = (8 * u256_low(a) as u8 + 7) as u16;
    let hi = u256_high(*b);
    let lo = u256_low(*b);
    let bit =
      if bit_index > 0x7f { hi } else { lo } & (1 << (bit_index % 128)) != 0;
    let mask = (U256::from(1) << bit_index) - U256::from(1);
    *b = if bit { *b | !mask } else { *b & mask }
  }
}

#[inline]
pub fn exp(state: &mut ExecutionState) -> Result<(), StatusCode> {
  let mut base = state.stack.pop();
  let mut power = state.stack.pop();

  if power > U256::zero() {
    let factor = 50;
    let additional_gas = factor * (log2floor(power) / 8 + 1);
    state.gas_left -= additional_gas as i64;
    if state.gas_left < 0 {
      return Err(StatusCode::OutOfGas);
    }
  }

  let mut v = U256::from(1);

  while power > U256::zero() {
    if (power & U256::from(1)) != U256::zero() {
      v = v.overflowing_mul(base).0;
    }
    power >>= 1;
    base = base.overflowing_mul(base).0;
  }

  state.stack.push(v);

  Ok(())
}

const SIGN_BITMASK_U128: u128 = 0x8000_0000_0000_0000_0000_0000_0000_0000;
const FLIPH_BITMASK_U128: u128 = 0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Sign {
  Plus,
  Minus,
  Zero,
}

#[inline]
fn log2floor(value: U256) -> u64 {
  debug_assert!(value != U256::zero());
  let mut l: u64 = 256;
  for v in [u256_high(value), u256_low(value)] {
    if v == 0 {
      l -= 128;
    } else {
      l -= v.leading_zeros() as u64;
      if l == 0 {
        return l;
      } else {
        return l - 1;
      }
    }
  }
  l
}

#[inline(always)]
fn two_compl(op: U256) -> U256 {
  !op + U256::from(1)
}

#[inline(always)]
fn two_compl_mut(op: &mut U256) {
  *op = two_compl(*op);
}

#[inline(always)]
fn i256_sign<const DO_TWO_COMPL: bool>(val: &mut U256) -> Sign {
  if u256_high(*val) & SIGN_BITMASK_U128 == 0 {
    if *val == U256::zero() {
      Sign::Zero
    } else {
      Sign::Plus
    }
  } else {
    if DO_TWO_COMPL {
      two_compl_mut(val);
    }
    Sign::Minus
  }
}

#[inline(always)]
fn i256_div(mut first: U256, mut second: U256) -> U256 {
  let min_negative_value: U256 = u128_words_to_u256(SIGN_BITMASK_U128, 0);
  let second_sign = i256_sign::<true>(&mut second);
  if second_sign == Sign::Zero {
    return U256::zero();
  }
  let first_sign = i256_sign::<true>(&mut first);
  if first_sign == Sign::Minus
    && first == min_negative_value
    && second == U256::from(1)
  {
    return two_compl(min_negative_value);
  }

  let mut d = first / second;

  u256_remove_sign(&mut d);

  if d == U256::zero() {
    return U256::zero();
  }

  match (first_sign, second_sign) {
    (Sign::Zero, Sign::Plus)
    | (Sign::Plus, Sign::Zero)
    | (Sign::Zero, Sign::Zero)
    | (Sign::Plus, Sign::Plus)
    | (Sign::Minus, Sign::Minus) => d,
    (Sign::Zero, Sign::Minus)
    | (Sign::Plus, Sign::Minus)
    | (Sign::Minus, Sign::Zero)
    | (Sign::Minus, Sign::Plus) => two_compl(d),
  }
}

#[inline(always)]
fn i256_mod(mut first: U256, mut second: U256) -> U256 {
  let first_sign = i256_sign::<true>(&mut first);
  if first_sign == Sign::Zero {
    return U256::zero();
  }

  let _ = i256_sign::<true>(&mut second);
  let mut r = first % second;
  u256_remove_sign(&mut r);
  if r == U256::zero() {
    return U256::zero();
  }
  if first_sign == Sign::Minus {
    two_compl(r)
  } else {
    r
  }
}

#[inline(always)]
fn u256_high(val: U256) -> u128 {
  let mut bytes = [0u8; 32];
  val.to_big_endian(&mut bytes);
  u128::from_be_bytes(bytes[0..16].try_into().unwrap())
}

#[inline(always)]
fn u256_low(val: U256) -> u128 {
  let mut bytes = [0u8; 32];
  val.to_big_endian(&mut bytes);
  u128::from_be_bytes(bytes[16..32].try_into().unwrap())
}

#[inline(always)]
fn u128_words_to_u256(high: u128, low: u128) -> U256 {
  let high = high.to_be_bytes();
  let low = low.to_be_bytes();
  let bytes = high.into_iter().chain(low.into_iter()).collect::<Vec<_>>();
  U256::from_big_endian(&bytes)
}

#[inline(always)]
fn u256_remove_sign(val: &mut U256) {
  let low = u256_low(*val);
  let mut high = u256_high(*val);
  high &= FLIPH_BITMASK_U128;
  *val = u128_words_to_u256(high, low)
}

#[cfg(test)]
mod tests {
  use {super::*, core::num::Wrapping};

  #[test]
  fn div_i256() {
    let min_negative_value: U256 = u128_words_to_u256(SIGN_BITMASK_U128, 0);

    assert_eq!(Wrapping(i8::MIN) / Wrapping(-1), Wrapping(i8::MIN));
    assert_eq!(i8::MAX / -1, -i8::MAX);

    let one = U256::from(1);
    let one_hundred = U256::from(100);
    let fifty = U256::from(50);
    let _fifty_sign = Sign::Plus;
    let two = U256::from(2);
    let neg_one_hundred = U256::from(100);
    let _neg_one_hundred_sign = Sign::Minus;
    let minus_one = U256::from(1);
    let max_value = U256::from(2).pow(255.into()) - 1;
    let neg_max_value = U256::from(2).pow(255.into()) - 1;

    assert_eq!(i256_div(min_negative_value, minus_one), min_negative_value);
    assert_eq!(i256_div(min_negative_value, one), min_negative_value);
    assert_eq!(i256_div(max_value, one), max_value);
    assert_eq!(i256_div(max_value, minus_one), neg_max_value);
    assert_eq!(i256_div(one_hundred, minus_one), neg_one_hundred);
    assert_eq!(i256_div(one_hundred, two), fifty);
  }
}
