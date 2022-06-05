mod actor;
mod execution;
mod instructions;
mod memory;
mod message;
mod opcode;
mod stack;
mod state;

#[cfg(feature = "fil-actor")]
fil_actors_runtime::wasm_trampoline!(actor::EvmRuntimeActor);
