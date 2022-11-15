# LINE BLOCKCHAIN AUCTION

## clone auction contract
```
❯ git clone https://github.com/shiki-tak/line-blockchain-auction.git && cd line-blockchain-auction
```

## prepare chain
- Install simapp binaries with dynamic_link available in the following repository: https://github.com/line/lbm-sdk
```
❯ cd chain_tools
❯ cp simd $GOPATH/bin/ 
❯ ./init_node.sh sim
```

## send coin to alice and bob
```
❯ simd tx bank send $(simd keys show validator0 -a --keyring-backend=test --home ~/.simapp/simapp0) $(simd keys show alice -a --keyring-backend=test --home ~/.simapp/simapp0) 20000000000stake --keyring-backend=test --home ~/.simapp/simapp0 --chain-id sim -y
❯ simd tx bank send $(simd keys show validator0 -a --keyring-backend=test --home ~/.simapp/simapp0) $(simd keys show bob -a --keyring-backend=test --home ~/.simapp/simapp0) 20000000000stake --keyring-backend=test --home ~/.simapp/simapp0 --chain-id sim -y
```

## check accounts
```
❯ simd keys list --keyring-backend=test --home ~/.simapp/simapp0
```

## compile contract
```
❯ docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.5 && cd artifacts
```

## store nft contract
```
❯ simd tx wasm store nft.wasm --from alice --gas-prices="0.025stake" --gas="auto" --gas-adjustment="1.2" -y --keyring-backend=test --chain-id=sim --home ~/.simapp/simapp0
❯ NFT_CODE_ID=1
```

## instantiate nft contract
```
❯ NFT_INIT='{"name":"line-nft", "symbol":"NFT"}'
simd tx wasm instantiate $NFT_CODE_ID "$NFT_INIT" --admin $(simd keys show alice -a --keyring-backend=test --home ~/.simapp/simapp0) --from alice --label "test" --gas-prices="0.025stake" --gas="auto" --gas-adjustment="1.2" -y --keyring-backend=test --chain-id=sim --home ~/.simapp/simapp0
❯ NFT_CONTRACT=link18vd8fpwxzck93qlwghaj6arh4p7c5n89fvcmzu
```

## mint nft
```
❯ MINT='{"mint":{"name":"nft-1", "uri":"nft.1.example.com"}}'
simd tx wasm execute $NFT_CONTRACT "$MINT" --from alice --gas-prices="0.025stake" --gas="auto" --gas-adjustment="1.2" -y --keyring-backend=test --chain-id=sim --home ~/.simapp/simapp0
```
## query nft owner
```
❯ OWNER='{"owner":{"token_id":"0"}}' && simd query wasm contract-state smart $NFT_CONTRACT "$OWNER" --output json
```

## store auction contract
```
❯ simd tx wasm store auction.wasm --from alice --gas-prices="0.025stake" --gas="auto" --gas-adjustment="1.2" -y --keyring-backend=test --chain-id=sim --home ~/.simapp/simapp0
❯ AUCTION_CODE_ID=2
```

## instantiate auction contract
```
❯ AUCTION_INIT='{"auction_limit_block_height":100}'
❯ simd tx wasm instantiate $AUCTION_CODE_ID "$AUCTION_INIT" --admin $(simd keys show alice -a --keyring-backend=test --home ~/.simapp/simapp0) --from alice --label "test" --gas-prices="0.025stake" --gas="auto" --gas-adjustment="1.2" -y --keyring-backend=test --chain-id=sim --home ~/.simapp/simapp0
❯ AUCTION_CONTRACT=link10pyejy66429refv3g35g2t7am0was7yaducgya
```

## approve for all 
```
❯ APPROVEFORALL=$(jq -n --arg opeartor $AUCTION_CONTRACT '{"approve_for_all":{"opeartor": $opeartor, "approved": true}}')
❯ simd tx wasm execute $NFT_CONTRACT "$APPROVEFORALL" --from alice --gas-prices="0.025stake" --gas="auto" --gas-adjustment="1.2" -y --keyring-backend=test --chain-id=sim --home ~/.simapp/simapp0
```

# listing nft
```
❯ LISTING=$(jq -n --arg nft_contract_address $NFT_CONTRACT '{"listing":{"nft_contract_address":$nft_contract_address, "id": "0", "minimum_bid":{"denom":"stake", "amount": "100"}}}')
❯ simd tx wasm execute $AUCTION_CONTRACT "$LISTING" --from alice --gas-prices="0.025stake" --gas="auto" --gas-adjustment="1.2" -y --keyring-backend=test --chain-id=sim --home ~/.simapp/simapp0
```

## query listing token
```
❯ LISTINGTOKEN='{"listing_token":{"listing_id":"pwxzck93qlwghaj6arh4p7c5n89fvcmzu0"}}'
❯ simd query wasm contract-state smart $AUCTION_CONTRACT "$LISTINGTOKEN" --output json
```

## query nft owner
```
❯ simd query wasm contract-state smart $NFT_CONTRACT "$OWNER" --output json
```

## bid
```
❯ BID='{"bid":{"listing_id":"pwxzck93qlwghaj6arh4p7c5n89fvcmzu0"}}'
❯ simd tx wasm execute $AUCTION_CONTRACT "$BID" --from bob --amount 101stake --gas-prices="0.025stake" --gas="auto" --gas-adjustment="1.2" -y --keyring-backend=test --chain-id=sim --home ~/.simapp/simapp0
```

## withdraw
```
❯ WITHDRAW='{"withdraw":{"listing_id":"pwxzck93qlwghaj6arh4p7c5n89fvcmzu0"}}'
❯ simd tx wasm execute $AUCTION_CONTRACT "$WITHDRAW" --from alice --gas-prices="0.025stake" --gas="auto" --gas-adjustment="1.2" -y --keyring-backend=test --chain-id=sim --home ~/.simapp/simapp0
```

## auction result
```
❯ simd query wasm contract-state smart $NFT_CONTRACT "$OWNER" --output json
```