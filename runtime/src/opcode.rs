#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpCode {
  /// the byte representing the opcode in binary
  pub code: u8,

  /// cot of executing the opcode, subtracted from the
  /// total gas limit when running bytecode.
  pub gas_cost: u16,

  /// The number of stack items the instruction accesses during execution.
  pub stack_height_required: u8,

  /// The stack height change caused by the instruction execution. Can be
  /// negative.
  pub stack_height_change: i8,

  /// Human readable name of the opcode.
  pub name: &'static str,
}

impl OpCode {
  #[inline]
  pub const fn to_u8(self) -> u8 {
    self.code
  }

  #[inline]
  pub const fn to_usize(self) -> usize {
    self.to_u8() as usize
  }
}

impl OpCode {
  pub const ADD: OpCode = OpCode {
    code: 0x01,
    gas_cost: todo!(),
    stack_height_required: 2,
    stack_height_change: -1,
    name: "ADD",
  };
  pub const ADDMOD: OpCode = OpCode {
    code: 0x08,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const ADDRESS: OpCode = OpCode {
    code: 0x30,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const AND: OpCode = OpCode {
    code: 0x16,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const BALANCE: OpCode = OpCode {
    code: 0x31,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const BASEFEE: OpCode = OpCode {
    code: 0x48,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const BLOCKHASH: OpCode = OpCode {
    code: 0x40,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const BYTE: OpCode = OpCode {
    code: 0x1a,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const CALL: OpCode = OpCode {
    code: 0xf1,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const CALLCODE: OpCode = OpCode {
    code: 0xf2,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const CALLDATACOPY: OpCode = OpCode {
    code: 0x37,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const CALLDATALOAD: OpCode = OpCode {
    code: 0x35,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const CALLDATASIZE: OpCode = OpCode {
    code: 0x36,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const CALLER: OpCode = OpCode {
    code: 0x33,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const CALLVALUE: OpCode = OpCode {
    code: 0x34,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const CHAINID: OpCode = OpCode {
    code: 0x46,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const CODECOPY: OpCode = OpCode {
    code: 0x39,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const CODESIZE: OpCode = OpCode {
    code: 0x38,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const COINBASE: OpCode = OpCode {
    code: 0x41,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const CREATE: OpCode = OpCode {
    code: 0xf0,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const CREATE2: OpCode = OpCode {
    code: 0xf5,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DELEGATECALL: OpCode = OpCode {
    code: 0xf4,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DIFFICULTY: OpCode = OpCode {
    code: 0x44,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DIV: OpCode = OpCode {
    code: 0x04,
    gas_cost: todo!(),
    stack_height_required: 2,
    stack_height_change: -1,
    name: todo!(),
  };
  pub const DUP1: OpCode = OpCode {
    code: 0x80,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DUP10: OpCode = OpCode {
    code: 0x89,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DUP11: OpCode = OpCode {
    code: 0x8a,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DUP12: OpCode = OpCode {
    code: 0x8b,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DUP13: OpCode = OpCode {
    code: 0x8c,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DUP14: OpCode = OpCode {
    code: 0x8d,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DUP15: OpCode = OpCode {
    code: 0x8e,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DUP16: OpCode = OpCode {
    code: 0x8f,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DUP2: OpCode = OpCode {
    code: 0x81,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DUP3: OpCode = OpCode {
    code: 0x82,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DUP4: OpCode = OpCode {
    code: 0x83,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DUP5: OpCode = OpCode {
    code: 0x84,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DUP6: OpCode = OpCode {
    code: 0x85,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DUP7: OpCode = OpCode {
    code: 0x86,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DUP8: OpCode = OpCode {
    code: 0x87,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const DUP9: OpCode = OpCode {
    code: 0x88,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const EQ: OpCode = OpCode {
    code: 0x14,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const EXP: OpCode = OpCode {
    code: 0x0a,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const EXTCODECOPY: OpCode = OpCode {
    code: 0x3c,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const EXTCODEHASH: OpCode = OpCode {
    code: 0x3f,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const EXTCODESIZE: OpCode = OpCode {
    code: 0x3b,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const GAS: OpCode = OpCode {
    code: 0x5a,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const GASLIMIT: OpCode = OpCode {
    code: 0x45,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const GASPRICE: OpCode = OpCode {
    code: 0x3a,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const GT: OpCode = OpCode {
    code: 0x11,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const INVALID: OpCode = OpCode {
    code: 0xfe,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const ISZERO: OpCode = OpCode {
    code: 0x15,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const JUMP: OpCode = OpCode {
    code: 0x56,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const JUMPDEST: OpCode = OpCode {
    code: 0x5b,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const JUMPI: OpCode = OpCode {
    code: 0x57,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const KECCAK256: OpCode = OpCode {
    code: 0x20,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const LOG0: OpCode = OpCode {
    code: 0xa0,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const LOG1: OpCode = OpCode {
    code: 0xa1,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const LOG2: OpCode = OpCode {
    code: 0xa2,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const LOG3: OpCode = OpCode {
    code: 0xa3,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const LOG4: OpCode = OpCode {
    code: 0xa4,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const LT: OpCode = OpCode {
    code: 0x10,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const MLOAD: OpCode = OpCode {
    code: 0x51,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const MOD: OpCode = OpCode {
    code: 0x06,
    gas_cost: todo!(),
    stack_height_required: 2,
    stack_height_change: -1,
    name: todo!(),
  };
  pub const MSIZE: OpCode = OpCode {
    code: 0x59,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const MSTORE: OpCode = OpCode {
    code: 0x52,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const MSTORE8: OpCode = OpCode {
    code: 0x53,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const MUL: OpCode = OpCode {
    code: 0x02,
    gas_cost: todo!(),
    stack_height_required: 2,
    stack_height_change: -1,
    name: todo!(),
  };
  pub const MULMOD: OpCode = OpCode {
    code: 0x09,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const NOT: OpCode = OpCode {
    code: 0x19,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const NUMBER: OpCode = OpCode {
    code: 0x43,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const OR: OpCode = OpCode {
    code: 0x17,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const ORIGIN: OpCode = OpCode {
    code: 0x32,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PC: OpCode = OpCode {
    code: 0x58,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const POP: OpCode = OpCode {
    code: 0x50,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH1: OpCode = OpCode {
    code: 0x60,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH10: OpCode = OpCode {
    code: 0x69,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH11: OpCode = OpCode {
    code: 0x6a,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH12: OpCode = OpCode {
    code: 0x6b,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH13: OpCode = OpCode {
    code: 0x6c,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH14: OpCode = OpCode {
    code: 0x6d,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH15: OpCode = OpCode {
    code: 0x6e,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH16: OpCode = OpCode {
    code: 0x6f,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH17: OpCode = OpCode {
    code: 0x70,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH18: OpCode = OpCode {
    code: 0x71,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH19: OpCode = OpCode {
    code: 0x72,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH2: OpCode = OpCode {
    code: 0x61,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH20: OpCode = OpCode {
    code: 0x73,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH21: OpCode = OpCode {
    code: 0x74,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH22: OpCode = OpCode {
    code: 0x75,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH23: OpCode = OpCode {
    code: 0x76,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH24: OpCode = OpCode {
    code: 0x77,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH25: OpCode = OpCode {
    code: 0x78,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH26: OpCode = OpCode {
    code: 0x79,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH27: OpCode = OpCode {
    code: 0x7a,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH28: OpCode = OpCode {
    code: 0x7b,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH29: OpCode = OpCode {
    code: 0x7c,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH3: OpCode = OpCode {
    code: 0x62,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH30: OpCode = OpCode {
    code: 0x7d,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH31: OpCode = OpCode {
    code: 0x7e,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH32: OpCode = OpCode {
    code: 0x7f,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH4: OpCode = OpCode {
    code: 0x63,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH5: OpCode = OpCode {
    code: 0x64,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH6: OpCode = OpCode {
    code: 0x65,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH7: OpCode = OpCode {
    code: 0x66,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH8: OpCode = OpCode {
    code: 0x67,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const PUSH9: OpCode = OpCode {
    code: 0x68,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const RETURN: OpCode = OpCode {
    code: 0xf3,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const RETURNDATACOPY: OpCode = OpCode {
    code: 0x3e,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const RETURNDATASIZE: OpCode = OpCode {
    code: 0x3d,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const REVERT: OpCode = OpCode {
    code: 0xfd,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SAR: OpCode = OpCode {
    code: 0x1d,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SDIV: OpCode = OpCode {
    code: 0x05,
    gas_cost: todo!(),
    stack_height_required: 2,
    stack_height_change: -1,
    name: todo!(),
  };
  pub const SELFBALANCE: OpCode = OpCode {
    code: 0x47,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SELFDESTRUCT: OpCode = OpCode {
    code: 0xff,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SGT: OpCode = OpCode {
    code: 0x13,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SHL: OpCode = OpCode {
    code: 0x1b,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SHR: OpCode = OpCode {
    code: 0x1c,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SIGNEXTEND: OpCode = OpCode {
    code: 0x0b,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SLOAD: OpCode = OpCode {
    code: 0x54,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SLT: OpCode = OpCode {
    code: 0x12,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SMOD: OpCode = OpCode {
    code: 0x07,
    gas_cost: todo!(),
    stack_height_required: 2,
    stack_height_change: -1,
    name: todo!(),
  };
  pub const SSTORE: OpCode = OpCode {
    code: 0x55,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const STATICCALL: OpCode = OpCode {
    code: 0xfa,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const STOP: OpCode = OpCode {
    code: 0x00,
    gas_cost: todo!(),
    stack_height_required: 0,
    stack_height_change: 0,
    name: todo!(),
  };
  pub const SUB: OpCode = OpCode {
    code: 0x03,
    gas_cost: todo!(),
    stack_height_required: 2,
    stack_height_change: -1,
    name: todo!(),
  };
  pub const SWAP1: OpCode = OpCode {
    code: 0x90,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SWAP10: OpCode = OpCode {
    code: 0x99,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SWAP11: OpCode = OpCode {
    code: 0x9a,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SWAP12: OpCode = OpCode {
    code: 0x9b,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SWAP13: OpCode = OpCode {
    code: 0x9c,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SWAP14: OpCode = OpCode {
    code: 0x9d,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SWAP15: OpCode = OpCode {
    code: 0x9e,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SWAP16: OpCode = OpCode {
    code: 0x9f,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SWAP2: OpCode = OpCode {
    code: 0x91,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SWAP3: OpCode = OpCode {
    code: 0x92,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SWAP4: OpCode = OpCode {
    code: 0x93,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SWAP5: OpCode = OpCode {
    code: 0x94,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SWAP6: OpCode = OpCode {
    code: 0x95,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SWAP7: OpCode = OpCode {
    code: 0x96,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SWAP8: OpCode = OpCode {
    code: 0x97,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const SWAP9: OpCode = OpCode {
    code: 0x98,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const TIMESTAMP: OpCode = OpCode {
    code: 0x42,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
  pub const XOR: OpCode = OpCode {
    code: 0x18,
    gas_cost: todo!(),
    stack_height_required: todo!(),
    stack_height_change: todo!(),
    name: todo!(),
  };
}

impl std::fmt::Display for OpCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}
