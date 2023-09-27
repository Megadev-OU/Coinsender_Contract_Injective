
## Fee calculation

Usage fee in smart contract is represend as percent number multiplied by 10  
1 % should be passed "10" as agrument, 2.3 % as 23, 25 % as 250  
fee argument as float is not supported


## running tests in contracts-coinsender/cosmwasm-contracts/ directory

`cargo test`

## build contract
`cargo wasm`
`docker run --rm -v "$(pwd)":/code   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry   cosmwasm/rust-optimizer:0.14.0`


## deploy contract to sei

``` 
export CHAIN_ID="pacific-1"
export RPC_ENDPOINT="https://sei-rpc.polkachu.com/"
export ACCOUNT_NAME="coinsender"
export ACCOUNT_ADDRESS="sei1v77p09duts5ke378wfgsz485d3ezfe87h9s95x"
export CONTRACT_WASM_BINARY="artifacts/cosmwasm_contracts.wasm"


seid tx wasm store $CONTRACT_WASM_BINARY -y --from=$ACCOUNT_NAME --chain-id=$CHAIN_ID --node $RPC_ENDPOINT --gas=2000000 --fees=200000usei --broadcast-mode=block

export CONTRACT_ID="356"
export LABEL="TokenCoinsender"

seid tx wasm instantiate $CONTRACT_ID '{"bank": "sei1u5hf6kg6kvwnsr259d3mpqyrxzc5g0l8x3ekpm"}' --chain-id $CHAIN_ID --node $RPC_ENDPOINT --from $ACCOUNT_NAME --gas=1000000 --fees=100000usei --broadcast-mode=block --label $LABEL --no-admin

```

### test call to transfer

```
seid tx wasm execute "sei10rdc0yqvlr57mjlxex8uez6xkpzh6f9s93tecsq6u0a9mv3k5pcss68hhr" '{"token_sender": {"recipient_amounts": [["sei1v77p09duts5ke378wfgsz485d3ezfe87h9s95x", "100000"], ["sei1v77p09duts5ke378wfgsz485d3ezfe87h9s95x", "500000"]], "fee": "10" }}' --amount=606000usei --from $ACCOUNT_NAME --broadcast-mode=block  --chain-id $CHAIN_ID --node $RPC_ENDPOINT --gas=150000 --fees=15000usei -y

```
