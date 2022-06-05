use {
  crate::{memory::Memory, message::Message, stack::Stack},
  bytes::Bytes,
};

/// Maximum allowed EVM bytecode size.
/// The contract code size limit is 24kB.
pub const MAX_CODE_SIZE: usize = 0x6000;

/// EVM execution state.
#[derive(Clone, Debug)]
pub struct ExecutionState<'m> {
  pub(crate) gas_left: i64,
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
