## running tests in contracts-coinsender/cosmwasm-contracts/ directory

`cargo test`

## build contract
`cargo wasm && cp target/wasm32-unknown-unknown/release/cosmwasm_contracts.wasm multisend.wasm`

## Fee calculation

Usage fee in smart contract is represend as percent number multiplied by 10  
1 % should be passed "10" as agrument, 2.3 % as 23, 25 % as 250  
fee argument as float is not supported
