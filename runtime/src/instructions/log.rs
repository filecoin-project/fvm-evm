use crate::{execution::ExecutionState, message::StatusCode, platform::Platform};

#[inline]
pub fn log(
  state: &mut ExecutionState,
  platform: &Platform,
  num_topics: usize,
) -> Result<(), StatusCode> {
  todo!()
}
