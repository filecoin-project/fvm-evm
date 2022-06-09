use {
  crate::StatusCode,
  fvm_ipld_encoding::Cbor,
  serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub gas_left: i64,
  pub status_code: StatusCode,
  pub logs: Vec<String>,
}

impl Cbor for Output {}
