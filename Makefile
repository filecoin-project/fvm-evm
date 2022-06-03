RUSTFLAGS="-C target-feature=+crt-static"

all: copy
.PHONY: all

build:
	cargo build
.PHONY: build

copy: build
	@mkdir -p wasm && \
	 cp target/debug/wbuild/fvm-evm-registry/fvm_evm_registry.compact.wasm wasm/fvm_evm_registry.compact.wasm && \
	 cp target/debug/wbuild/fvm-evm-runtime/fvm_evm_runtime.compact.wasm wasm/fvm_evm_runtime.compact.wasm 

clean:
	@rm -rf target && cargo clean

test: copy
	cargo test -- --show-output

lint: clean
	cargo fmt --all
	cargo clippy --all -- -D warnings -A clippy::upper_case_acronyms
