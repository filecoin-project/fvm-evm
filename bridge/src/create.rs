use {
  crate::state,
  anyhow::anyhow,
  cid::Cid,
  fil_actors_runtime::{runtime::Runtime, ActorError},
  fvm_evm::{
    execute,
    AccountKind,
    Bytecode,
    EthereumAccount,
    EvmContractRuntimeConstructor,
    ExecutionState,
    Message,
    SignedTransaction,
    StatusCode,
    System,
    H160,
    U256,
  },
  fvm_ipld_blockstore::Blockstore,
  fvm_ipld_encoding::{from_slice, RawBytes},
  fvm_ipld_hamt::Hamt,
  fvm_shared::{address::Address, bigint::BigInt},
  rlp::RlpStream,
  serde_tuple::{Deserialize_tuple, Serialize_tuple},
  sha3::{Digest, Keccak256},
};

const INIT_ACTOR_EXEC_METHOD_NUM: u64 = 2;

/// Init actor Exec Params
#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct ExecParams {
  pub code_cid: Cid,
  pub constructor_params: RawBytes,
}

/// Init actor Exec Return value
#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct ExecReturn {
  /// ID based address for created actor
  pub id_address: Address,
  /// Reorg safe address for actor
  pub robust_address: Address,
}

/// Determine the address of the newly created contract
fn compute_contract_address(tx: &SignedTransaction) -> Result<H160, ActorError> {
  let mut rlp = RlpStream::new();
  rlp.append(&tx.sender_address()?);
  rlp.append(&tx.nonce());
  Ok(H160::from_slice(&Keccak256::digest(rlp.as_raw())[12..]))
}

/// This is invoked when a transaction is sent to the ZERO address
/// and has an input bytes. If there is no input bytes then it means
/// that it is a simple burn.
pub fn create_contract<BS, RT>(
  rt: &mut RT,
  tx: SignedTransaction,
) -> anyhow::Result<RawBytes>
where
  BS: Blockstore,
  RT: Runtime<BS>,
{
  // Create a temporary contract state that will be used to store
  // results of constructor execution, then assigned as the state
  // root of a new EVM actor
  let state_cid = Hamt::<_, U256, U256>::new(rt.store()).flush()?;

  // The address of the bridge is always passed as a constructor
  // parameter to newly created EVM actors, so it knows where to
  // query for other EVM accounts. This cannot be hardcoded because
  // the deployment code cid of the bridge is not known at compile time.
  let bridge_addr = Address::new_id(fvm_sdk::message::receiver());

  // Create an instance of the platform abstraction layer with it's state
  // rooted at the temporary contract state.
  let system = System::new(state_cid, rt, bridge_addr, H160::zero(), &tx)?;

  // compute the potential contract address if the
  // constructor runs successfully to completion.
  let contract_address = compute_contract_address(&tx)?;

  // the initial balance of the newly created contract
  let endowment = tx.value();

  let message: Message = tx.try_into()?;

  // create new execution context around this transaction
  let mut exec_state = ExecutionState::new(&message);

  // identify bytecode valid jump destinations
  let bytecode = Bytecode::new(&message.input_data).map_err(|e| anyhow!(e))?;

  // invoke the contract constructor
  let exec_status = execute(&bytecode, &mut exec_state, &system)
    .map_err(|e| ActorError::unspecified(format!("EVM execution error: {e:?}")))?;

  if !exec_status.reverted
    && exec_status.status_code == StatusCode::Success
    && !exec_status.output_data.is_empty()
  {
    // load global bridge HAMT
    let mut bridge_state = state::BridgeState::load(rt)?;
    let mut bridge_accounts_map = bridge_state.accounts(rt)?;

    // todo: support counterfactual deployments.
    if !bridge_accounts_map.contains_key(&contract_address)? {
      // constructor ran to completion successfully and returned
      // the resulting bytecode.
      let bytecode = exec_status.output_data.clone();

      // this data will be used to intantiate a new EVM actor
      // instance. Use the state populated by the EVM constructor
      // code and the returned resulting bytecode. Also keep
      // a reference to the bridge address on every EVM actor.
      let runtime_params = EvmContractRuntimeConstructor {
        bytecode,
        initial_state: system.flush_state()?,
        registry: bridge_addr,
        address: contract_address,
      };

      fvm_sdk::debug::log(format!(
        "Bridge thinks that EVM runtime CodeCid is {:?}",
        bridge_state.runtime_cid()
      ));

      // Params to the builtin InitActor#Exec method
      let init_actor_params = ExecParams {
        code_cid: *bridge_state.runtime_cid(),
        constructor_params: RawBytes::serialize(runtime_params)?,
      };

      let init_actor_params = RawBytes::serialize(init_actor_params)?;

      // let the Init Actor create a new address
      let init_output = rt.send(
        *fil_actors_runtime::INIT_ACTOR_ADDR,
        INIT_ACTOR_EXEC_METHOD_NUM,
        init_actor_params,
        BigInt::default(),
      )?;

      // the init actor should return the address of the new contract
      let init_output: ExecReturn = from_slice(&init_output)?;

      // store the EVM to FVM account mapping
      bridge_accounts_map.set(contract_address, EthereumAccount {
        nonce: 0,
        balance: endowment,
        kind: AccountKind::Contract {
          fil_account: init_output.robust_address,
        },
      })?;

      // save accoutns state updates
      bridge_state.update_accounts(&mut bridge_accounts_map)?;

      // return newly created contract address
      Ok(RawBytes::serialize(contract_address)?)
    } else {
      unimplemented!("Not implemented yet");
    }
  } else {
    // todo: more precise error handling
    Err(anyhow!(ActorError::illegal_argument(format!(
      "EVM constructor failed: {exec_status:?}"
    ))))
  }
}
