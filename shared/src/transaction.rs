use {
  crate::{H160, H256, U256},
  bytes::Bytes,
  fil_actors_runtime::ActorError,
  fvm_sdk::crypto::recover_public_key,
  fvm_shared::crypto::signature::SECP_PUB_LEN,
  sha3::{Digest, Keccak256},
};

pub enum TransactionAction {
  Call(H160),
  Create,
}

pub struct AccessListItem {
  pub address: H160,
  pub slots: Vec<H256>,
}

pub enum Transaction {
  /// rlp([nonce, gasPrice, gasLimit, to, value, data, init, vrs])
  Legacy {
    chain_id: Option<u64>,
    nonce: u64,
    gas_price: U256,
    gas_limit: u64,
    action: TransactionAction,
    value: U256,
    input: Bytes,
  },
  /// 0x01 || rlp([chainId, nonce, gasPrice, gasLimit, to, value, data,
  /// accessList, signatureYParity, signatureR, signatureS])
  EIP2930 {
    chain_id: u64,
    nonce: u64,
    gas_price: U256,
    gas_limit: u64,
    action: TransactionAction,
    value: U256,
    input: Bytes,
    access_list: Vec<AccessListItem>,
  },
  /// 0x02 || rlp([chain_id, nonce, max_priority_fee_per_gas, max_fee_per_gas,
  /// gas_limit, destination, amount, data, access_list, signature_y_parity,
  /// signature_r, signature_s])
  EIP1559 {
    chain_id: u64,
    nonce: u64,
    max_priority_fee_per_gas: U256,
    max_fee_per_gas: U256,
    gas_limit: u64,
    action: TransactionAction,
    value: U256,
    input: Bytes,
    access_list: Vec<AccessListItem>,
  },
}

impl Transaction {
  pub fn hash(&self) -> H256 {
    H256::zero()
  }
}

pub struct TransactionSignature {
  odd_y_parity: bool,
  r: H256,
  s: H256,
}

pub struct SignedTransaction {
  transaction: Transaction,
  signature: TransactionSignature,
}

#[derive(Debug)]
pub enum RLPError {
  EmptyRlp,
  InvalidLength,
}

impl TryFrom<&[u8]> for SignedTransaction {
  type Error = RLPError;

  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    if value.is_empty() {
      return Err(RLPError::EmptyRlp);
    }

    match value[0] {
      0x01 => parse_eip2930_transaction(value),
      0x02 => parse_eip1559_transaction(value),
      _ => parse_legacy_transaction(value),
    }
  }
}

impl SignedTransaction {
  /// The secp256k1 public key of the transaction sender.
  ///
  /// This public key can used to derive the equivalent Filecoin account
  pub fn sender_public_key(&self) -> Result<[u8; SECP_PUB_LEN], ActorError> {
    let mut sig = [0u8; 65];
    sig[..32].copy_from_slice(self.signature.r.as_bytes());
    sig[32..].copy_from_slice(self.signature.s.as_bytes());
    sig[64] = self.signature.odd_y_parity as u8;
    recover_public_key(&self.transaction.hash().to_fixed_bytes(), &sig).map_err(|e| {
      ActorError::illegal_argument(format!("failed to recover public key: {e:?}"))
    })
  }

  /// Ethereum sender address which is 20-bytes trimmed keccak256(pubkey)
  pub fn sender_address(&self) -> Result<H160, ActorError> {
    let pubkey = self.sender_public_key()?;
    let address_slice = &Keccak256::digest(&pubkey[1..])[12..];
    Ok(H160::from_slice(address_slice))
  }
}

fn parse_legacy_transaction(bytes: &[u8]) -> Result<SignedTransaction, RLPError> {
  Ok(SignedTransaction {
    signature: TransactionSignature {
      odd_y_parity: true,
      r: H256::zero(),
      s: H256::zero(),
    },
    transaction: Transaction::Legacy {
      chain_id: Some(1),
      nonce: 1,
      gas_price: U256::zero(),
      gas_limit: 1,
      action: TransactionAction::Create,
      value: U256::zero(),
      input: Bytes::new(),
    },
  })
}

fn parse_eip2930_transaction(_bytes: &[u8]) -> Result<SignedTransaction, RLPError> {
  Ok(SignedTransaction {
    signature: TransactionSignature {
      odd_y_parity: true,
      r: H256::zero(),
      s: H256::zero(),
    },
    transaction: Transaction::EIP2930 {
      chain_id: 1,
      nonce: 1,
      gas_price: U256::zero(),
      gas_limit: 1,
      action: TransactionAction::Create,
      value: U256::zero(),
      input: Bytes::new(),
      access_list: vec![],
    },
  })
}

fn parse_eip1559_transaction(_bytes: &[u8]) -> Result<SignedTransaction, RLPError> {
  Ok(SignedTransaction {
    signature: TransactionSignature {
      odd_y_parity: true,
      r: H256::zero(),
      s: H256::zero(),
    },
    transaction: Transaction::EIP1559 {
      chain_id: 1,
      nonce: 1,
      gas_limit: 1,
      action: TransactionAction::Create,
      value: U256::zero(),
      input: Bytes::new(),
      max_priority_fee_per_gas: U256::zero(),
      max_fee_per_gas: U256::zero(),
      access_list: vec![],
    },
  })
}

#[cfg(test)]
mod tests {

  #[test]
  fn decode_legacy_transaction() {
    todo!()
  }

  #[test]
  fn decode_eip2930_transaction() {
    todo!()
  }

  #[test]
  fn decode_eip1559_transaction() {
    todo!()
  }
}
