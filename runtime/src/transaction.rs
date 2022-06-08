#[cfg(test)]
mod tests {
  use hex_literal::hex;

  #[test]
  fn decode_rlp_transaction() -> Result<(), ()> {
    let rlp_encoded = hex!("f86d8202b38477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ba0699ff162205967ccbabae13e07cdd4284258d46ec1051a70a51be51ec2bc69f3a04e6944d508244ea54a62ebf9a72683eeadacb73ad7c373ee542f1998147b220e");

    Ok(())
  }
}
