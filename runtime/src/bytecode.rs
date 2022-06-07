use {
  crate::{message::StatusCode, opcode::OpCode},
  std::ops::Deref,
};

pub struct Bytecode<'c> {
  code: &'c [u8],
  jumpdest: Vec<bool>,
}

impl<'c> Bytecode<'c> {
  pub fn new(bytecode: &'c [u8]) -> Result<Self, StatusCode> {
    // only jumps to those addresses are valid. This is a security
    // feature by EVM to disallow jumps to arbitary code addresses.
    let mut jumpdest = Vec::<bool>::with_capacity(bytecode.len());

    let mut i = 0;
    while i < bytecode.len() {
      let opcode = OpCode::try_from(bytecode[i])?;
      i += match opcode {
        OpCode::JUMPDEST => {
          jumpdest[i] = true;
          1
        }
        OpCode::PUSH1
        | OpCode::PUSH2
        | OpCode::PUSH3
        | OpCode::PUSH4
        | OpCode::PUSH5
        | OpCode::PUSH6
        | OpCode::PUSH7
        | OpCode::PUSH8
        | OpCode::PUSH9
        | OpCode::PUSH10
        | OpCode::PUSH11
        | OpCode::PUSH12
        | OpCode::PUSH13
        | OpCode::PUSH14
        | OpCode::PUSH15
        | OpCode::PUSH16
        | OpCode::PUSH17
        | OpCode::PUSH18
        | OpCode::PUSH19
        | OpCode::PUSH20
        | OpCode::PUSH21
        | OpCode::PUSH22
        | OpCode::PUSH23
        | OpCode::PUSH24
        | OpCode::PUSH25
        | OpCode::PUSH26
        | OpCode::PUSH27
        | OpCode::PUSH28
        | OpCode::PUSH29
        | OpCode::PUSH30
        | OpCode::PUSH31
        | OpCode::PUSH32 => (opcode.code - OpCode::PUSH1.code + 2) as usize,
        _ => 1,
      };
    }

    Ok(Self {
      code: bytecode,
      jumpdest,
    })
  }

  /// Checks if the EVM is allowed to jump to this location.
  ///
  /// This location must begin with a JUMPDEST opcode that
  /// marks a valid jump destination
  pub fn valid_jump_destination(&self, offset: usize) -> bool {
    offset < self.jumpdest.len() && self.jumpdest[offset]
  }
}

impl<'c> Deref for Bytecode<'c> {
  type Target = [u8];

  fn deref(&self) -> &'c Self::Target {
    self.code
  }
}

impl<'c> AsRef<[u8]> for Bytecode<'c> {
  fn as_ref(&self) -> &'c [u8] {
    self.code
  }
}
