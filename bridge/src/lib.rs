use {
  fil_actors_runtime::{
    actor_error,
    runtime::{ActorCode, Runtime},
    ActorError,
    INIT_ACTOR_ADDR,
  },
  fvm_evm::{abort, EthereumAccount, Output, StatusCode, H160},
  fvm_ipld_blockstore::Blockstore,
  fvm_ipld_encoding::{from_slice, RawBytes},
  fvm_ipld_hamt::Hamt,
  fvm_sdk::sself,
  fvm_shared::{MethodNum, METHOD_CONSTRUCTOR},
  num_derive::FromPrimitive,
  num_traits::FromPrimitive,
};

#[cfg(feature = "fil-actor")]
fil_actors_runtime::wasm_trampoline!(RegistryActor);

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
  Constructor = METHOD_CONSTRUCTOR,
  RetreiveAccount = 2,
  UpsertAccount = 3,
  InvokeMessage = 4,
}

pub struct RegistryActor;
impl RegistryActor {
  pub fn constructor<BS, RT>(rt: &mut RT) -> Result<(), ActorError>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    rt.validate_immediate_caller_is(std::iter::once(&*INIT_ACTOR_ADDR))?;

    let empty_map_cid = Hamt::<_, EthereumAccount, H160>::new(rt.store())
      .flush()
      .map_err(|e| {
        ActorError::illegal_state(format!("Failed to create empty map: {e}"))
      })?;

    if let Err(err) = sself::set_root(&empty_map_cid) {
      abort!(USR_ILLEGAL_STATE, "failed to initialize state root: {err}");
    }

    Ok(())
  }

  pub fn upsert<BS, RT>(
    rt: &mut RT,
    address: H160,
    account: EthereumAccount,
  ) -> Result<(), ActorError>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    rt.validate_immediate_caller_accept_any()?;

    let mut dict = load_global_map(rt)?;

    dict
      .set(address, account)
      .map_err(|e| ActorError::illegal_argument(e.to_string()))?;

    let new_root = dict
      .flush()
      .map_err(|e| ActorError::illegal_state(e.to_string()))?;

    sself::set_root(&new_root).map_err(|e| ActorError::illegal_state(e.to_string()))?;

    Ok(())
  }

  pub fn retreive<BS, RT>(
    rt: &mut RT,
    address: H160,
  ) -> Result<EthereumAccount, ActorError>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    rt.validate_immediate_caller_accept_any()?;

    let dict = load_global_map(rt)?;

    let account = dict
      .get(&address)
      .map_err(|e| ActorError::illegal_argument(e.to_string()))?;

    Ok(match account {
      // account exists, returns its contents.
      Some(acc) => *acc,

      // account does not exist, ethereum then synthesizes an empty
      // account with zero balance, zero nonce, and everything else
      // zeroed out.
      None => EthereumAccount::default(),
    })
  }

  pub fn invoke<BS, RT>(rt: &mut RT, rlp: &[u8]) -> Result<Output, ActorError>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    let transaction = fvm_evm::SignedTransaction::try_from(rlp)
      .map_err(|e| ActorError::illegal_argument(format!("{e:?}")))?;

    let sender = transaction
      .sender_address()
      .map_err(|e| ActorError::illegal_argument(format!("{e:?}")))?;

    Ok(Output {
      logs: vec![sender.to_string()],
      gas_left: 0,
      status_code: StatusCode::Success,
    })
    
  }
}

impl ActorCode for RegistryActor {
  fn invoke_method<BS, RT>(
    rt: &mut RT,
    method: MethodNum,
    params: &RawBytes,
  ) -> Result<RawBytes, ActorError>
  where
    BS: Blockstore + Clone,
    RT: Runtime<BS>,
  {
    match FromPrimitive::from_u64(method) {
      Some(Method::Constructor) => {
        Self::constructor(rt)?;
        Ok(RawBytes::default())
      }
      Some(Method::RetreiveAccount) => {
        let address = from_slice(&params)?;
        let account = Self::retreive(rt, address)?;
        Ok(RawBytes::serialize(account)?)
      }
      Some(Method::UpsertAccount) => {
        let (address, account) = from_slice(&params)?;
        Self::upsert(rt, address, account)?;
        Ok(RawBytes::default())
      }
      Some(Method::InvokeMessage) => {
        let rlp: Vec<u8> = from_slice(&params)?;
        let output = Self::invoke(rt, &rlp)?;
        Ok(RawBytes::serialize(output)?)
      }
      None => Err(actor_error!(unhandled_message; "Invalid method")),
    }
  }
}

fn load_global_map<BS, RT>(
  rt: &mut RT,
) -> Result<Hamt<&BS, EthereumAccount, H160>, ActorError>
where
  BS: Blockstore,
  RT: Runtime<BS>,
{
  Hamt::<_, EthereumAccount, H160>::load(
    &sself::root().map_err(|e| ActorError::not_found(e.to_string()))?,
    rt.store(),
  )
  .map_err(|e| ActorError::illegal_state(e.to_string()))
}
