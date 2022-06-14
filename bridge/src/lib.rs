use {
  fil_actors_runtime::{
    actor_error,
    runtime::{ActorCode, Runtime},
    ActorError,
    INIT_ACTOR_ADDR,
  },
  fvm_evm::{
    abort,
    execute,
    Bytecode,
    EthereumAccount,
    ExecutionState,
    Message,
    Output,
    StatusCode,
    System,
    H160,
    U256,
  },
  fvm_ipld_blockstore::Blockstore,
  fvm_ipld_encoding::{from_slice, RawBytes},
  fvm_ipld_hamt::Hamt,
  fvm_sdk::{debug, sself},
  fvm_shared::{address::Address, MethodNum, METHOD_CONSTRUCTOR},
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
  ProcessTransaction = 4,
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

  pub fn process_transaction<BS, RT>(
    rt: &mut RT,
    rlp: &[u8],
  ) -> Result<Output, ActorError>
  where
    BS: Blockstore,
    RT: Runtime<BS>,
  {
    rt.validate_immediate_caller_accept_any()?;

    let transaction = fvm_evm::SignedTransaction::try_from(rlp)
      .map_err(|e| ActorError::illegal_argument(format!("{e:?}")))?;

    let message: Message = transaction
      .try_into()
      .map_err(|e| ActorError::illegal_argument(format!("{e:?}")))?;

    debug::log(format!("FVM message: {message:#?}"));

    let mut exec_state = ExecutionState::new(&message);
    let bytecode = Bytecode::new(&message.input_data)
      .map_err(|e| ActorError::illegal_argument(format!("{e:?}")))?;

    let bridge_addr = Address::new_id(fvm_sdk::message::receiver());
    let state_cid = Hamt::<_, U256, U256>::new(rt.store())
      .flush()
      .map_err(|e| ActorError::illegal_argument(format!("{e:?}")))?;

    debug::log(format!("bridge address: {bridge_addr:?}"));
    let mut system = System::new(state_cid, rt, bridge_addr, H160::zero())
      .map_err(|e| ActorError::illegal_argument(format!("{e:?}")))?;

    let exec_status = execute(&bytecode, &mut exec_state, &mut system)
      .map_err(|e| ActorError::illegal_argument(format!("{e:?}")))?;

    debug::log(format!("evm exec status: {exec_status:?}"));

    Ok(Output {
      logs: vec![message.sender.to_string()],
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
      Some(Method::ProcessTransaction) => {
        let rlp: Vec<u8> = from_slice(&params)?;
        let output = Self::process_transaction(rt, &rlp)?;
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
