# COSMWASM-NFT
[![CircleCI](https://circleci.com/gh/shiki-tak/cosmwasm-nft.svg?style=svg)](https://circleci.com/gh/shiki-tak/cosmwasm-nft)
[![codecov](https://codecov.io/gh/shiki-tak/cosmwasm-nft/branch/master/graph/badge.svg)](https://codecov.io/gh/shiki-tak/cosmwasm-nft)

## 1. Checkout code and compile
```
git clone https://github.com/CosmWasm/wasmd.git
cd wasmd
make install
```

## 2. Set up a single-node local testnet
```
# if you've done this before, wipe out all data from last run
# this may wipe out keys, make sure you know you want to do this
rm -rf ~/.wasmd

cd $HOME
wasmd init --chain-id=testing testing

# if you've done this before, check which keys are created locally first
# wasmcli keys list
# you can skip any "add" steps if they already exist
wasmcli keys add validator

wasmd add-genesis-account $(wasmcli keys show validator -a) 1000000000stake,1000000000validatortoken
# You can add a few more accounts here if you wish (for experiments beyond the tutorial)

wasmd gentx --name validator
wasmd collect-gentxs
wasmd start
```

## 3. Connecting with a Client
```
wasmcli config chain-id testing
wasmcli config trust-node true
wasmcli config node tcp://localhost:26657
wasmcli config output json
wasmcli config indent true
# this is important, so the cli returns after the tx is in a block,
# and subsequent queries return the proper results
wasmcli config broadcast-mode block

wasmcli keys add fred
wasmcli keys add bob
wasmcli keys list

# verify initial setup
wasmcli query account $(wasmcli keys show validator -a)
```

## 4. Uploading the Code
```
# for the rest of this section, we assume you are in the same path as the rust contract (Cargo.toml)
cd <path/to/rust/code>

# and recompile wasm
docker run --rm -v $(pwd):/code \
  --mount type=volume,source=$(basename $(pwd))_cache,target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  confio/cosmwasm-opt:0.7.2

# ensure the hash changed
cat hash.txt

# both should be empty
wasmcli query wasm list-code

# upload and see we create code 1
wasmcli tx wasm store contract.wasm --from validator --gas 42000000 -y
wasmcli query wasm list-code
wasmcli query wasm list-contract-by-code 1
```

## 5. Instantiating the Contract
```
# instantiate contract and verify
INIT="{\"name\":\"wasm-cosmwasm_nft\", \"symbol\":\"WSM\"}"
wasmcli tx wasm instantiate 1 "$INIT" --from validator --amount=50000stake  --label "nft-contract" -y

# check the contract state (and account balance)
wasmcli query wasm list-contract-by-code 1
# contracts ids (like code ids) are based on an auto-gen sequence
# if this is the first contract in the devnet, it will have this address (otherwise, use the result from list-contract-by-code)
CONTRACT=cosmos18vd8fpwxzck93qlwghaj6arh4p7c5n89uzcee5
wasmcli query wasm contract $CONTRACT
wasmcli query account $CONTRACT
```