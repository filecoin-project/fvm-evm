# Filecoin Virtual Machine EVM Actor

This is a very early stage of the project and this readme will evolve over time to include build and deployment instrustions.

## Build to WASM

In the root of the project execute the following command:

```sh
$ make
```

It should produce a wasm binary in `./target/debug/wbuild/fil_actor_evm/fil_actor_evm.compact.wasm` that containst the EVM runtime actor in optimized release mode.

## Running tests

In the root of the project execute the following command: 

```sh
$ make test
```

it will compile all actors to wasm and then run them inside a simulated FVM environment and excercise all test cases.

## Design Choices

So far I have made the following design choices:
  - The EVM implementation of choice will be [evmodin](https://github.com/vorot93/evmodin). The two primary reasons for that are:
    - It has a very high code quality and it is used in a production-grade ethereum client ([akula](https://github.com/akula-bft/akula)) that is passing 100% of the conformance tests of Ethereum, as oposed to ~98% in other implementations like revm, sputnik, etc.
    - The second very important reason is that it has the resumable execution model, meaning that this evm interpretter can be run in a way where it traps and invokes user-provided code, as opposed to doing everything on its own and then returning the result of the operation. This will allow for greater control over the execution of smart contracts and opens the possibility for many optimizations, like caching, on-the fly translation between filecoin IPLD types and EVM types rather than having to prepare a copy of the entire execution state ahead of time, etc.


