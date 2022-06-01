RUSTFLAGS="-C target-feature=+crt-static"

all: build
.PHONY: all

build: copy
	cargo build
.PHONY: build

copy: 
	@mkdir -p wasm && \
	 cp target/debug/wbuild/fvm-evm-registry/fvm_evm_registry.compact.wasm wasm/fvm_evm_registry.compact.wasm && \
	 cp target/debug/wbuild/fvm-evm-runtime/fvm_evm_runtime.compact.wasm wasm/fvm_evm_runtime.compact.wasm 

clean:
	@rm -rf target && cargo clean

test:
	cargo test

lint: clean
	cargo fmt --all
	cargo clippy --all -- -D warnings -A clippy::upper_case_acronyms
