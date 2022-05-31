export LOTUS_PATH=~/.lotus-local-net
export LOTUS_MINER_PATH=~/.lotus-miner-local-net
export LOTUS_SKIP_GENESIS_CHECK=_yes_
export CGO_CFLAGS_ALLOW="-D__BLST_PORTABLE__"
export CGO_CFLAGS="-D__BLST_PORTABLE__"

rm -rf ~/.lotus
rm -rf ~/.genesis-sectors
rm -rf $LOTUS_PATH
rm -rf $LOTUS_MINER_PATH

lotus fetch-params 2048
lotus-seed pre-seal --sector-size 2KiB --num-sectors 2
lotus-seed genesis new ~/lotus-localnet.json
lotus-seed genesis add-miner ~/lotus-localnet.json ~/.genesis-sectors/pre-seal-t01000.json

lotus daemon --lotus-make-genesis=devgen.car --genesis-template=~/lotus-localnet.json --bootstrap=false &
sleep 10 &&
lotus wallet import --as-default ~/.genesis-sectors/pre-seal-t01000.key &&
lotus-miner init --genesis-miner --actor=t01000 --sector-size=2KiB --pre-sealed-sectors=~/.genesis-sectors --pre-sealed-metadata=~/.genesis-sectors/pre-seal-t01000.json --nosync &&
lotus-miner run --nosync &&
wait
