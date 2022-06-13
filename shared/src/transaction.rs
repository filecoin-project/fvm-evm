#[cfg(not(test))]
use fvm_sdk::crypto::recover_public_key;
use {
  crate::{H160, H256, U256},
  bytes::Bytes,
  fil_actors_runtime::ActorError,
  fvm_shared::crypto::signature::SECP_PUB_LEN,
  rlp::{DecoderError, Rlp, RlpStream},
  sha3::{Digest, Keccak256},
  std::ops::Deref,
};

#[derive(Debug)]
pub enum TransactionAction {
  Call(H160),
  Create,
}

impl rlp::Encodable for TransactionAction {
  fn rlp_append(&self, s: &mut RlpStream) {
    match self {
      Self::Call(address) => {
        s.encoder().encode_value(&address[..]);
      }
      Self::Create => s.encoder().encode_value(&[]),
    }
  }
}

impl rlp::Decodable for TransactionAction {
  fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
    if rlp.is_empty() {
      if rlp.is_data() {
        Ok(TransactionAction::Create)
      } else {
        Err(DecoderError::RlpExpectedToBeData)
      }
    } else {
      Ok(TransactionAction::Call(rlp.as_val()?))
    }
  }
}

#[derive(Debug)]
pub struct AccessListItem {
  pub address: H160,
  pub slots: Vec<H256>,
}

#[derive(Debug)]
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
    match self {
      Transaction::Legacy {
        chain_id,
        nonce,
        gas_price,
        gas_limit,
        action,
        value,
        input,
      } => {
        let mut s = RlpStream::new();
        if let Some(chain_id) = chain_id {
          s.begin_list(9);
          s.append(nonce);
          s.append(gas_price);
          s.append(gas_limit);
          s.append(action);
          s.append(value);
          s.append(input);
          s.append(chain_id);
          s.append(&0_u8);
          s.append(&0_u8);
        } else {
          s.begin_list(6);
          s.append(nonce);
          s.append(gas_limit);
          s.append(gas_limit);
          s.append(action);
          s.append(value);
          s.append(input);
        }
        H256::from_slice(Keccak256::digest(s.as_raw()).as_slice())
      }
      Transaction::EIP2930 {
        chain_id,
        nonce,
        gas_price,
        gas_limit,
        action,
        value,
        input,
        access_list,
      } => todo!(),
      Transaction::EIP1559 {
        chain_id,
        nonce,
        max_priority_fee_per_gas,
        max_fee_per_gas,
        gas_limit,
        action,
        value,
        input,
        access_list,
      } => todo!(),
    }
  }
}

#[derive(Debug)]
pub struct TransactionRecoveryId(pub u64);

impl Deref for TransactionRecoveryId {
  type Target = u64;

  fn deref(&self) -> &u64 {
    &self.0
  }
}

impl TransactionRecoveryId {
  pub fn standard(&self) -> u8 {
    if self.0 == 27 || self.0 == 28 || self.0 > 36 {
      ((self.0 - 1) % 2) as u8
    } else {
      4
    }
  }

  pub fn chain_id(&self) -> Option<u64> {
    if self.0 > 36 {
      Some((self.0 - 35) / 2)
    } else {
      None
    }
  }
}

#[derive(Debug)]
pub struct TransactionSignature {
  v: TransactionRecoveryId,
  r: H256,
  s: H256,
}

impl rlp::Encodable for TransactionSignature {
  fn rlp_append(&self, s: &mut RlpStream) {
    s.append(&self.v.0);
    s.append(&self.r);
    s.append(&self.s);
  }
}

impl rlp::Decodable for TransactionSignature {
  fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
    Ok(Self {
      v: TransactionRecoveryId(rlp.val_at(0)?),
      r: rlp.val_at(1)?,
      s: rlp.val_at(2)?,
    })
  }
}

#[derive(Debug)]
pub struct SignedTransaction {
  transaction: Transaction,
  signature: TransactionSignature,
}

impl TryFrom<&[u8]> for SignedTransaction {
  type Error = DecoderError;

  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    if value.is_empty() {
      return Err(DecoderError::RlpIsTooShort);
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
    sig[32..64].copy_from_slice(self.signature.s.as_bytes());
    sig[64] = self.signature.v.standard();

    #[cfg(not(test))] // use a syscall to fvm
    return recover_public_key(&self.transaction.hash().to_fixed_bytes(), &sig).map_err(
      |e| ActorError::illegal_argument(format!("failed to recover public key: {e:?}")),
    );

    #[cfg(test)]
    // invoke the recovery impl directly as there is not FVM running this code
    return Ok(
      fvm_shared::crypto::signature::ops::recover_public_key(
        &self.transaction.hash().to_fixed_bytes(),
        &sig,
      )
      .unwrap()
      .serialize(),
    );
  }

  /// Ethereum sender address which is 20-bytes trimmed keccak256(pubkey)
  pub fn sender_address(&self) -> Result<H160, ActorError> {
    let pubkey = self.sender_public_key()?;
    let address_slice = &Keccak256::digest(&pubkey[1..])[12..];
    Ok(H160::from_slice(address_slice))
  }
}

/// rlp([nonce, gasPrice, gasLimit, to, value, data, init, v, r, s])
fn parse_legacy_transaction(bytes: &[u8]) -> Result<SignedTransaction, DecoderError> {
  let rlp = Rlp::new(bytes);

  if rlp.item_count()? != 9 {
    return Err(DecoderError::RlpIncorrectListLen);
  }

  let signature = TransactionSignature {
    v: TransactionRecoveryId(rlp.val_at(6)?),
    r: rlp.val_at(7)?,
    s: rlp.val_at(8)?,
  };

  Ok(SignedTransaction {
    transaction: Transaction::Legacy {
      chain_id: signature.v.chain_id(),
      nonce: rlp.val_at(0)?,
      gas_price: rlp.val_at(1)?,
      gas_limit: rlp.val_at(2)?,
      action: rlp.val_at(3)?,
      value: rlp.val_at(4)?,
      input: rlp.val_at(5)?,
    },
    signature,
  })
}

/// 0x01 || rlp([chainId, nonce, gasPrice, gasLimit, to, value, data,
/// accessList, signatureYParity, signatureR, signatureS])
fn parse_eip2930_transaction(bytes: &[u8]) -> Result<SignedTransaction, DecoderError> {
  let _rlp = Rlp::new(&bytes[1..]);
  Ok(SignedTransaction {
    signature: TransactionSignature {
      v: TransactionRecoveryId(27),
      r: H256::zero(),
      s: H256::zero(),
    },
    transaction: Transaction::EIP2930 {
      chain_id: 1,
      nonce: 1,
      gas_price: U256::zero(),
      gas_limit: 1,
      action: TransactionAction::Call(H160::zero()),
      value: U256::zero(),
      input: Bytes::new(),
      access_list: vec![],
    },
  })
}

/// 0x02 || rlp([chain_id, nonce, max_priority_fee_per_gas, max_fee_per_gas,
/// gas_limit, destination, amount, data, access_list, signature_y_parity,
/// signature_r, signature_s])
fn parse_eip1559_transaction(bytes: &[u8]) -> Result<SignedTransaction, DecoderError> {
  let _rlp = Rlp::new(&bytes[1..]);
  Ok(SignedTransaction {
    signature: TransactionSignature {
      v: TransactionRecoveryId(28),
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
  use {crate::SignedTransaction, hex_literal::hex};

  #[test]
  fn decode_legacy_transaction() {
    // https://etherscan.io/tx/0x3741aea434dc6e9e740be0113af4bac372fcdd2fa2188409c93c9405cbdcaaf0
    let raw = hex!(
      "f9016b0885113abe69b38302895c947a250d5630b4cf539739df2c5dacb4c659f2488d80b90
       1044a25d94a00000000000000000000000000000000000000000000000022b1c8c1227a0000
       000000000000000000000000000000000000000000000003f0a59430f92a924400000000000
       000000000000000000000000000000000000000000000000000a00000000000000000000000
       0012021043bbaab3b71b2217655787a13d24cf618b000000000000000000000000000000000
       00000000000000000000000603c6a1e00000000000000000000000000000000000000000000
       00000000000000000002000000000000000000000000fe9a29ab92522d14fc65880d8172142
       61d8479ae000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc225
       a01df6c364ee7d2b684bbb6e3892fee69a1bc4fc487222b003ea57ec1596884916a01e1643f
       de193fde5e6be4ae0b2d4c4669560132a6dc87b6404d5c0cdc743fee6
    "
    );

    let transaction = SignedTransaction::try_from(&raw[..]).unwrap();
    let address = transaction.sender_address().unwrap();
    let hash = transaction.transaction.hash();
    println!("legacy transaction: {transaction:?}");
    println!("sender address: {address:?}");
    println!("tx hash: {hash:?}");
  }

  #[test]
  fn decode_eip2930_transaction() {
    let _raw = hex!(
      "b8f501f8f205078506fc23ac008357b58494811a752c8cd697e3cb27279c330ed1ada745
      a8d7881bc16d674ec80000906ebaf477f83e051589c1188bcc6ddccdf872f85994de0b295
      669a9fd93d5f28d9ec85e40f4cb697baef842a00000000000000000000000000000000000
      000000000000000000000000000003a000000000000000000000000000000000000000000
      00000000000000000000007d694bb9bc244d798123fde783fcc1c72d3bb8c189413c080a0
      36b241b061a36a32ab7fe86c7aa9eb592dd59018cd0443adc0903590c16b02b0a05edcc54
      1b4741c5cc6dd347c5ed9577ef293a62787b4510465fadbfe39ee4094"
    );
    todo!()
  }

  #[test]
  fn decode_eip1559_transaction() {
    // https://etherscan.io/tx/0x734678f719001015c5b5f5cbac6a9210ede7ee6ce63e746ff2e9eecda3ab68c7
    let _raw = hex!(
      "02f8720104843b9aca008504eb6480bc82520894f76c5b19e86c256
       482f4aad1dae620a0c3ac0cd68717699d954d540080c080a05a5206a8e0486b8e101bcf
       4ed5b290df24a4d54f1ca752c859fa19c291244b98a0177166d96fd69db70628d99855b
       400c8a149b2254c211a0a00645830f5338218"
    );
    todo!()
  }
}
