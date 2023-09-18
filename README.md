## running tests in contracts-coinsender/cosmwasm-contracts/ directory

`cargo test`

## build contract
`cargo wasm && cp target/wasm32-unknown-unknown/release/cosmwasm_contracts.wasm multisend.wasm`

## Fee calculation

Usage fee in smart contract is represend as percent number multiplied by 1000
1 % should be passed "1000" as agrument, 2.35 % as 2350

