LOTUS_PATH=~/.lotus-local-net
LOTUS_MINER_PATH=~/.lotus-miner-local-net
LOTUS_SKIP_GENESIS_CHECK=_yes_
CGO_CFLAGS_ALLOW="-D__BLST_PORTABLE__"
CGO_CFLAGS="-D__BLST_PORTABLE__"

echo "Building actor..."
cargo build

CID=$(lotus chain install-actor target/debug/wbuild/fil_actor_evm/fil_actor_evm.compact.wasm | sed -n 's,^Actor Code CID: ,,p')
echo "CodeID: $CID"

CREATE_PARAMS=$(echo "EVM Bytecode - 123\c" | base64)
ADDRESS=$(lotus chain create-actor $CID $CREATE_PARAMS | sed -n 's,^Robust Address: ,,p')
echo "Actor Address: $ADDRESS"

echo "invoking method get_bytecode.."
RETURN=$(lotus chain invoke $ADDRESS 2 | tail -1 | base64 --decode)
echo "Result: $RETURN"