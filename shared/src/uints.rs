use {
  fixed_hash::construct_fixed_hash,
  impl_serde::{impl_fixed_hash_serde, impl_uint_serde},
  uint::construct_uint,
};

construct_uint! { pub struct U256(4); } // ethereum word size
construct_uint! { pub struct U512(8); } // used for addmod and mulmod opcodes

construct_fixed_hash! { pub struct H160(20); } // ethereum address
construct_fixed_hash! { pub struct H256(32); } // Keccak256

// make ETH uints serde serializable,
// so it can work with Hamt and other
// IPLD structures seamlessly
impl_uint_serde!(U256, 4);
impl_uint_serde!(U512, 8);
impl_fixed_hash_serde!(H160, 20);
impl_fixed_hash_serde!(H256, 32);

macro_rules! impl_hamt_hash {
  ($type:ident) => {
    impl fvm_ipld_hamt::Hash for $type {
      fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
      }
    }
  };
}

// Hamt support
impl_hamt_hash!(H160);
impl_hamt_hash!(H256);

impl_hamt_hash!(U256);
impl_hamt_hash!(U512);
