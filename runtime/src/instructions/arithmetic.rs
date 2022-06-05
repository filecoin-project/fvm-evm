use {crate::stack::Stack, fvm_evm::U256};

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

// #[inline]
// pub fn modulo(stack: &mut Stack) {
//   let a = stack.pop();
//   let b = stack.get_mut(0);
//   *b = if *b == 0 { U256::ZERO } else { a % *b };
// }
