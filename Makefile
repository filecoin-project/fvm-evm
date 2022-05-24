RUSTFLAGS="-C target-feature=+crt-static"

all: build
.PHONY: all

build:
	cargo build --target wasm32-unknown-unknown --release
.PHONY: build

clean:
	cargo clean

test:
	cargo test

lint: clean
	cargo fmt --all
	cargo clippy --all -- -D warnings -A clippy::upper_case_acronyms
