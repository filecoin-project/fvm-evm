use {
  fvm_evm::{abort, EthereumAccount, H160},
  fvm_ipld_encoding::{from_slice, to_vec, RawBytes, DAG_CBOR},
  fvm_sdk::{ipld, message, sself, NO_DATA_BLOCK_ID},
  store::Blockstore,
};

mod store;

// mapping of EVM addresses HASH(Pubkey) -> EthAccount metadata
type Hamt = fvm_ipld_hamt::Hamt<Blockstore, EthereumAccount, H160>;

#[no_mangle]
pub fn invoke(params_ptr: u32) -> u32 {
  let ret: Option<RawBytes> = match message::method_number() {
    1 => constructor(params_ptr),
    2 => retreive(params_ptr),
    3 => upsert(params_ptr),
    _ => abort!(USR_UNHANDLED_MESSAGE, "unrecognized method"),
  };

  match ret {
    None => NO_DATA_BLOCK_ID,
    Some(v) => match ipld::put_block(DAG_CBOR, v.bytes()) {
      Ok(id) => id,
      Err(err) => {
        abort!(USR_SERIALIZATION, "failed to store return value: {}", err)
      }
    },
  }
}

pub fn constructor(params_ptr: u32) -> Option<RawBytes> {
  const INIT_ACTOR_ADDR: ActorID = 1;
  if message::caller() != INIT_ACTOR_ADDR {
    abort!(USR_FORBIDDEN, "constructor invoked by non-init actor");
  }

  // initialize an empty dictionary
  let state_root = match Hamt::new(Blockstore).flush() {
    Ok(cid) => cid,
    Err(_) => abort!(
      USR_SERIALIZATION,
      "failed to initialize EVM contract state HAMT: {err}"
    ),
  };

  if let Err(err) = sself::set_root(&state_root) {
    abort!(USR_ILLEGAL_STATE, "failed to initialize state root: {err}");
  }
}

pub fn upsert(params_ptr: u32) -> Option<RawBytes> {
  todo!();
}

pub fn retreive(params_ptr: u32) -> Option<RawBytes> {
  let eth_address: EthereumAccount = {
    match message::params_raw(params_ptr) {
      Ok((codec, bytes)) => match codec {
        DAG_CBOR => from_slice(&bytes),
        _ => abort!(
          USR_ILLEGAL_STATE,
          "invalid parameter codec: {codec}. Expecting DAG-CBOR"
        ),
      },
      Err(err) => abort!(USR_ILLEGAL_ARGUMENT, "invalid parameter: {err:?}"),
    };
  };

  let root_cid = match sself::root() {
    Ok(cid) => cid,
    Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get state root: {err:?}"),
  };

  let dict = match Hamt::load(cid, Blockstore) {
    Ok(dict) => dict,
    Err(err) => abort!(USR_ILLEGAL_STATE, "failed to load hamt: {err}"),
  };

  let acc = match dict.get(&eth_address) {
    Ok(acc) => match acc {
      // account exists, returns its contents.
      Some(existing) => *existing,

      // account does not exist, ethereum then synthesizes an empty
      // account with zero balance, zero nonce, and everything else
      // zeroed out.
      None => EthereumAccount::default(),
    },
    Err(err) => abort!(USR_ILLEGAL_STATE),
  };

  Some(to_vec(&acc))
}
