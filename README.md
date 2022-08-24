# [ARCHIVED] Filecoin Virtual Machine EVM Actor prototype

---
>
> ### This is a deprecated repo with historical relevance
>
> This repo is where the EVM actor for the Filecoin Virtual Machine was incubated.
>
> This actor was merged into builtin-actors in the following PR: https://github.com/filecoin-project/builtin-actors/pull/517
>
---

This is a very early stage of the project and this readme will evolve over time to include build and deployment instrustions.

## Build to WASM

In the root of the project execute the following command:

```sh
$ make
```

It should produce a wasm binary in `./wasm/fil_actor_evm.compact.wasm` that containst the EVM runtime actor in optimized release mode.

## Running tests

In the root of the project execute the following command: 

```sh
$ make test
```

it will compile all actors to wasm and then run them inside a simulated FVM environment and excercise all test cases.

## Design Overview

### Opcodes

The EVM runtime in this actor implement opcodes and their semantics as of the London hard fork.

### Memory

Handling of EVM `MSTORE`, `MLOAD` and `MSIZE` opcodes is implemented by the `Memory` module. This module is responsible for the volatile memory that is available only during contract execution and reclaimed afterwards by the system. 

The basic unit of allocation is a _Page_ which is 64KB, to map 1:1 to WASM memory model of allocating memory in pages of 64KB. 

Every contract execution context begins with one memory page allocated and it grows to more pages if the contract requests more memory than the current reserved amount.

Currently there is no limit on the EVM side on how much memory a contract may reserve. This is limited by the wasmtime configuration in FVM.

### Persistance

Handling of EVM `SSTORE` and `SLOAD` opcodes is implemented in terms of reads and writes to _IPLD Hamt_. EVM defines the concept of cold and warm memory access, where first access to a given address is considered cold that is more expensive and subsequent reads or writes to that memory address are considered warm and incur lower gas cost. This comes from [EIP-2930](https://eips.ethereum.org/EIPS/eip-2930). Filecoin does not have a notion of warm and cold storage access so this is kind of meaningless to us in general and is only kept there for now to keep EVM gas accounting accurate. This will likely go in future iterations and all state access will be treated equally.

All contract runtime state is persisted in a `Hamt::<_, U256, U256>` mapping and its root `Cid` is stored in the `state` field of the contract state. This Cid is conceptially equivalent to Ethereum's state root field of a contract account. This data structure may mutate throught contract's lifetime and the root Cid of the mapping gets updated after every successfull transaction that performed writes.

Bytecode is an immutable part of the state that is created in EVM runtime's constructor as a result of executing the init EVM bytecode in the creation transaction.

Any change to the contract state will invoke `sself::set_root` and update the state tree roof of the contract actor. This syscall is called only once at the end of a successful non-reverted transaction.

The entire contract state is represented using the following structure:

```rust
pub struct ContractState {
  pub bridge: FilecoinAddress,
  pub bytecode: Cid,
  pub state: Cid,
  pub self_address: H160,
}
```

A reference to the bridge actor is stored in every EVM contract for resolving FIL addresses of other actors in cross-contract calls.


### Platform Interface



### Transactions

### Addressing and Registry

Currently there is a component called _Registry_ that is responsible for translating EVM addresses to FVM addresses. EVM transactions and inter-contract calls must use EVM addresses which are the first 20 bytes of keccak hash of their secp256k1 public key. This is incompatible with Filecoin addresses that are first 20 bytes of Blake2B hash of the account public key.

The registry contains a _Hamt_ IPLD structure keyed by `H160` values (Eth address). Each key maps to the following structure:

```rust
pub struct EthereumAccount {
  pub nonce: u64,
  pub balance: U256,
  pub kind: AccountKind,
}

pub enum AccountKind {
  ExternallyOwned {
    fil_account: Option<FileCoinAddress>,
  },
  Contract {
    fil_account: FileCoinAddress
  },
}
```

Externally owned addresses don't always have a known mapping to their Filecoin equivalent. This mapping is discovered only once a transaction is issued from that account. This mapping discovery process relies on `ecrecover` to recover the public key used for sigining a transaction and then hasing it using _Blake2B_ to get a FIL address that is controlled by the same private key as the EVM account.

Contract accounts _always_ maps to a known FIL address because contract creation occurs on the registry and the robust address is returned by the EVM Runtime actor constructor.

In later iterations we are planning on removing the registry commonent and replace it with univeral addresses, but this is still under design and discussion.

## Common Scenatios

### Contract Deployment

### Contract Invocation

### Delegate Call

### Cross Contract Calls
