use {
  crate::{H160, U256},
  bytes::Bytes,
  strum_macros::Display,
};

/// The kind of call-like instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CallKind {
  Call,
  DelegateCall,
  CallCode,
  Create,
  Create2 { salt: U256 },
}

/// The message describing an EVM call,
/// including a zero-depth call from transaction origin.
#[derive(Clone, Debug, PartialEq)]
pub struct Message {
  /// The kind of the call. For zero-depth calls `CallKind::Call` SHOULD be
  /// used.
  pub kind: CallKind,

  /// Static call mode.
  pub is_static: bool,

  /// The call depth.
  pub depth: i32,

  /// The amount of gas for message execution.
  pub gas: i64,

  /// The destination (recipient) of the message.
  pub recipient: H160,

  /// The sender of the message.
  pub sender: H160,

  /// Message input data.
  pub input_data: Bytes,

  /// The amount of Ether transferred with the message.
  pub value: U256,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateMessage {
  pub salt: Option<U256>,
  pub gas: i64,
  pub depth: i32,
  pub initcode: Bytes,
  pub sender: H160,
  pub endowment: U256,
}

impl From<CreateMessage> for Message {
  fn from(msg: CreateMessage) -> Self {
    Self {
      kind: match msg.salt {
        Some(salt) => CallKind::Create2 { salt },
        None => CallKind::Create,
      },
      is_static: false,
      depth: msg.depth,
      gas: msg.gas,
      recipient: H160::zero(),
      sender: msg.sender,
      input_data: msg.initcode,
      value: msg.endowment,
    }
  }
}

/// Output of EVM execution.
#[derive(Clone, Debug, PartialEq)]
pub struct Output {
  /// EVM exited with this status code.
  pub status_code: StatusCode,
  /// How much gas was left after execution
  pub gas_left: i64,
  /// Output data returned.
  pub output_data: Bytes,
  /// Contract creation address.
  pub create_address: Option<H160>,
  // indicates if revert was requested
  pub reverted: bool,
}

/// Message status code.
#[must_use]
#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum StatusCode {
  /// Execution finished with success.
  #[strum(serialize = "success")]
  Success,

  /// Generic execution failure.
  #[strum(serialize = "failure")]
  Failure,

  /// Execution terminated with REVERT opcode.
  #[strum(serialize = "revert")]
  Revert,

  /// The execution has run out of gas.
  #[strum(serialize = "out of gas")]
  OutOfGas,

  /// The designated INVALID instruction has been hit during execution.
  ///
  /// [EIP-141](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-141.md)
  /// defines the instruction 0xfe as INVALID instruction to indicate execution
  /// abortion coming from high-level languages. This status code is reported
  /// in case this INVALID instruction has been encountered.
  #[strum(serialize = "invalid instruction")]
  InvalidInstruction,

  /// An undefined instruction has been encountered.
  #[strum(serialize = "undefined instruction")]
  UndefinedInstruction,

  /// The execution has attempted to put more items on the EVM stack
  /// than the specified limit.
  #[strum(serialize = "stack overflow")]
  StackOverflow,

  /// Execution of an opcode has required more items on the EVM stack.
  #[strum(serialize = "stack underflow")]
  StackUnderflow,

  /// Execution has violated the jump destination restrictions.
  #[strum(serialize = "bad jump destination")]
  BadJumpDestination,

  /// Tried to read outside memory bounds.
  ///
  /// An example is RETURNDATACOPY reading past the available buffer.
  #[strum(serialize = "invalid memory access")]
  InvalidMemoryAccess,

  /// Call depth has exceeded the limit (if any)
  #[strum(serialize = "call depth exceeded")]
  CallDepthExceeded,

  /// Tried to execute an operation which is restricted in static mode.
  #[strum(serialize = "static mode violation")]
  StaticModeViolation,

  /// A call to a precompiled or system contract has ended with a failure.
  ///
  /// An example: elliptic curve functions handed invalid EC points.
  #[strum(serialize = "precompile failure")]
  PrecompileFailure,

  /// Contract validation has failed.
  #[strum(serialize = "contract validation failure")]
  ContractValidationFailure,

  /// An argument to a state accessing method has a value outside of the
  /// accepted range of values.
  #[strum(serialize = "argument out of range")]
  ArgumentOutOfRange,

  /// The caller does not have enough funds for value transfer.
  #[strum(serialize = "insufficient balance")]
  InsufficientBalance,

  /// EVM implementation generic internal error.
  #[strum(serialize = "internal error")]
  InternalError(&'static str),
}
