use {
  crate::{
    bytecode::Bytecode,
    instructions::{
      arithmetic,
      bitwise,
      boolean,
      call,
      context,
      control,
      hash,
      log::log,
      memory,
      stack::{self, dup, push, push1, push32, swap},
      storage,
    },
    memory::Memory,
    message::{CallKind, Message, Output, StatusCode},
    opcode::OpCode,
    platform::Platform,
    stack::Stack,
  },
  bytes::Bytes,
};

/// Maximum allowed EVM bytecode size.
/// The contract code size limit is 24kB.
pub const MAX_CODE_SIZE: usize = 0x6000;

/// EVM execution runtime.
#[derive(Clone, Debug)]
pub struct ExecutionState<'m> {
  pub gas_left: i64,
  pub stack: Stack,
  pub memory: Memory,
  pub message: &'m Message,
  pub return_data: Bytes,
  pub output_data: Bytes,
}

impl<'m> ExecutionState<'m> {
  pub fn new(message: &'m Message) -> Self {
    Self {
      gas_left: message.gas,
      stack: Stack::default(),
      memory: Memory::default(),
      message,
      return_data: Default::default(),
      output_data: Bytes::new(),
    }
  }
}

pub fn execute(
  bytecode: &Bytecode,
  runtime: &mut ExecutionState,
  system: &mut Platform,
) -> Result<Output, StatusCode> {
  let mut pc = 0; // program counter
  let mut reverted = false;

  loop {
    if pc >= bytecode.len() {
      break;
    }

    let op = OpCode::try_from(bytecode[pc])?;
    match op {
      OpCode::STOP => break,
      OpCode::ADD => arithmetic::add(&mut runtime.stack),
      OpCode::MUL => arithmetic::mul(&mut runtime.stack),
      OpCode::SUB => arithmetic::sub(&mut runtime.stack),
      OpCode::DIV => arithmetic::div(&mut runtime.stack),
      OpCode::SDIV => arithmetic::sdiv(&mut runtime.stack),
      OpCode::MOD => arithmetic::modulo(&mut runtime.stack),
      OpCode::SMOD => arithmetic::smod(&mut runtime.stack),
      OpCode::ADDMOD => arithmetic::addmod(&mut runtime.stack),
      OpCode::MULMOD => arithmetic::mulmod(&mut runtime.stack),
      OpCode::EXP => arithmetic::exp(runtime)?,
      OpCode::SIGNEXTEND => arithmetic::signextend(&mut runtime.stack),
      OpCode::LT => boolean::lt(&mut runtime.stack),
      OpCode::GT => boolean::gt(&mut runtime.stack),
      OpCode::SLT => boolean::slt(&mut runtime.stack),
      OpCode::SGT => boolean::sgt(&mut runtime.stack),
      OpCode::EQ => boolean::eq(&mut runtime.stack),
      OpCode::ISZERO => boolean::iszero(&mut runtime.stack),
      OpCode::AND => boolean::and(&mut runtime.stack),
      OpCode::OR => boolean::or(&mut runtime.stack),
      OpCode::XOR => boolean::xor(&mut runtime.stack),
      OpCode::NOT => boolean::not(&mut runtime.stack),
      OpCode::BYTE => bitwise::byte(&mut runtime.stack),
      OpCode::SHL => bitwise::shl(&mut runtime.stack),
      OpCode::SHR => bitwise::shr(&mut runtime.stack),
      OpCode::SAR => bitwise::sar(&mut runtime.stack),
      OpCode::KECCAK256 => hash::keccak256(runtime)?,
      OpCode::ADDRESS => context::address(state, platform)?,
      OpCode::BALANCE => storage::balance(runtime, platform)?,
      OpCode::CALLER => context::caller(state, platform)?,
      OpCode::CALLVALUE => context::call_value(state, platform)?,
      OpCode::CALLDATALOAD => call::calldataload(runtime),
      OpCode::CALLDATASIZE => call::calldatasize(runtime),
      OpCode::CALLDATACOPY => call::calldatacopy(runtime)?,
      OpCode::CODESIZE => call::codesize(&mut runtime.stack, bytecode.as_ref()),
      OpCode::CODECOPY => call::codecopy(runtime, bytecode.as_ref())?,
      OpCode::EXTCODESIZE => storage::extcodesize(state, platform)?,
      OpCode::EXTCODECOPY => memory::extcodecopy(state, platform)?,
      OpCode::RETURNDATASIZE => control::returndatasize(runtime),
      OpCode::RETURNDATACOPY => control::returndatacopy(runtime)?,
      OpCode::EXTCODEHASH => storage::extcodehash(state, platform)?,
      OpCode::BLOCKHASH => context::blockhash(state, platform)?,
      OpCode::ORIGIN => context::origin(state, platform)?,
      OpCode::COINBASE => context::coinbase(state, platform)?,
      OpCode::GASPRICE => context::gas_price(state, platform)?,
      OpCode::TIMESTAMP => context::timestamp(state, platform)?,
      OpCode::NUMBER => context::block_number(state, platform)?,
      OpCode::DIFFICULTY => context::difficulty(state, platform)?,
      OpCode::GASLIMIT => context::gas_limit(state, platform)?,
      OpCode::CHAINID => context::chain_id(state, platform)?,
      OpCode::BASEFEE => context::base_fee(state, platform)?,
      OpCode::SELFBALANCE => storage::selfbalance(state, platform)?,
      OpCode::POP => stack::pop(&mut runtime.stack),
      OpCode::MLOAD => memory::mload(runtime)?,
      OpCode::MSTORE => memory::mstore(runtime)?,
      OpCode::MSTORE8 => memory::mstore8(runtime)?,
      OpCode::JUMP => {
        pc = control::jump(&mut runtime.stack, &bytecode)?;
        continue; // don't increment PC after the jump
      }
      OpCode::JUMPI => {
        // conditional jump
        if let Some(dest) = control::jumpi(&mut runtime.stack, &bytecode)? {
          pc = dest; // condition met, set program counter
          continue; // don't increment PC after jump
        }
      }
      OpCode::PC => control::pc(&mut runtime.stack, pc),
      OpCode::MSIZE => memory::msize(runtime),
      OpCode::SLOAD => storage::sload(runtime, platform)?,
      OpCode::SSTORE => storage::sstore(runtime, platform)?,
      OpCode::GAS => control::gas(runtime),
      OpCode::JUMPDEST => {} // marker opcode for valid jumps addresses
      OpCode::PUSH1 => pc += push1(&mut runtime.stack, bytecode[pc + 1]),
      OpCode::PUSH2 => pc += push::<2>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH3 => pc += push::<3>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH4 => pc += push::<4>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH5 => pc += push::<5>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH6 => pc += push::<6>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH7 => pc += push::<7>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH8 => pc += push::<8>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH9 => pc += push::<9>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH10 => pc += push::<10>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH11 => pc += push::<11>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH12 => pc += push::<12>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH13 => pc += push::<13>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH14 => pc += push::<14>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH15 => pc += push::<15>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH16 => pc += push::<16>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH17 => pc += push::<17>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH18 => pc += push::<18>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH19 => pc += push::<19>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH20 => pc += push::<20>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH21 => pc += push::<21>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH22 => pc += push::<22>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH23 => pc += push::<23>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH24 => pc += push::<24>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH25 => pc += push::<25>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH26 => pc += push::<26>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH27 => pc += push::<27>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH28 => pc += push::<28>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH29 => pc += push::<29>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH30 => pc += push::<30>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH31 => pc += push::<31>(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::PUSH32 => pc += push32(&mut runtime.stack, &bytecode[pc + 1..]),
      OpCode::DUP1 => dup::<1>(&mut runtime.stack),
      OpCode::DUP2 => dup::<2>(&mut runtime.stack),
      OpCode::DUP3 => dup::<3>(&mut runtime.stack),
      OpCode::DUP4 => dup::<4>(&mut runtime.stack),
      OpCode::DUP5 => dup::<5>(&mut runtime.stack),
      OpCode::DUP6 => dup::<6>(&mut runtime.stack),
      OpCode::DUP7 => dup::<7>(&mut runtime.stack),
      OpCode::DUP8 => dup::<8>(&mut runtime.stack),
      OpCode::DUP9 => dup::<9>(&mut runtime.stack),
      OpCode::DUP10 => dup::<10>(&mut runtime.stack),
      OpCode::DUP11 => dup::<11>(&mut runtime.stack),
      OpCode::DUP12 => dup::<12>(&mut runtime.stack),
      OpCode::DUP13 => dup::<13>(&mut runtime.stack),
      OpCode::DUP14 => dup::<14>(&mut runtime.stack),
      OpCode::DUP15 => dup::<15>(&mut runtime.stack),
      OpCode::DUP16 => dup::<16>(&mut runtime.stack),
      OpCode::SWAP1 => swap::<1>(&mut runtime.stack),
      OpCode::SWAP2 => swap::<2>(&mut runtime.stack),
      OpCode::SWAP3 => swap::<3>(&mut runtime.stack),
      OpCode::SWAP4 => swap::<4>(&mut runtime.stack),
      OpCode::SWAP5 => swap::<5>(&mut runtime.stack),
      OpCode::SWAP6 => swap::<6>(&mut runtime.stack),
      OpCode::SWAP7 => swap::<7>(&mut runtime.stack),
      OpCode::SWAP8 => swap::<8>(&mut runtime.stack),
      OpCode::SWAP9 => swap::<9>(&mut runtime.stack),
      OpCode::SWAP10 => swap::<10>(&mut runtime.stack),
      OpCode::SWAP11 => swap::<11>(&mut runtime.stack),
      OpCode::SWAP12 => swap::<12>(&mut runtime.stack),
      OpCode::SWAP13 => swap::<13>(&mut runtime.stack),
      OpCode::SWAP14 => swap::<14>(&mut runtime.stack),
      OpCode::SWAP15 => swap::<15>(&mut runtime.stack),
      OpCode::SWAP16 => swap::<16>(&mut runtime.stack),
      OpCode::LOG0 => log(runtime, platform, 0)?,
      OpCode::LOG1 => log(runtime, platform, 1)?,
      OpCode::LOG2 => log(runtime, platform, 2)?,
      OpCode::LOG3 => log(runtime, platform, 3)?,
      OpCode::LOG4 => log(runtime, platform, 4)?,
      OpCode::CREATE => storage::create(state, platform, false)?,
      OpCode::CREATE2 => storage::create(state, platform, true)?,
      OpCode::CALL => call::call(runtime, platform, CallKind::Call, false)?,
      OpCode::CALLCODE => call::call(runtime, platform, CallKind::CallCode, false)?,
      OpCode::DELEGATECALL => {
        call::call(runtime, platform, CallKind::DelegateCall, false)?
      }
      OpCode::STATICCALL => call::call(runtime, platform, CallKind::Call, true)?,
      OpCode::RETURN | OpCode::REVERT => {
        control::ret(runtime)?;
        reverted = op == OpCode::REVERT;
        break;
      }
      OpCode::INVALID => return Err(StatusCode::InvalidInstruction),
      OpCode::SELFDESTRUCT => storage::selfdestruct(state, platform)?,
      _ => return Err(StatusCode::UndefinedInstruction),
    }

    pc += 1; // advance
  }

  Ok(Output {
    reverted,
    status_code: StatusCode::Success,
    gas_left: runtime.gas_left,
    output_data: runtime.output_data.clone(),
    create_address: None,
  })
}
