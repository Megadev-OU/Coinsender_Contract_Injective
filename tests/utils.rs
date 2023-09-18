use cosmwasm_std::{coins, Addr};
use cw_multi_test::{App, BasicApp, ContractWrapper, Executor};

use cosmwasm_contracts::msg::{InstantiateMsg, QueryMsg};
use cosmwasm_contracts::{execute, instantiate, query};


pub fn instantiate_contract() -> (BasicApp, Addr) {
    const TOKENS_DECIMALS: u32 = 18;

    const INIT_BALANCE: u128 = 10000 * 10u128.pow(TOKENS_DECIMALS);

    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked("user"),
                coins(10 * INIT_BALANCE, "eth"),
            )
            .unwrap();

        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked("owner"),
                coins(INIT_BALANCE, "eth"),
            )
            .unwrap();

        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked("bank"),
                coins(INIT_BALANCE, "eth"),
            )
            .unwrap();

        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked("recipient_1"),
                coins(INIT_BALANCE, "eth"),
            )
            .unwrap();

        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked("recipient_2"),
                coins(INIT_BALANCE, "eth"),
            )
            .unwrap();
    });

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                bank: "bank".to_string(),
            },
            &[],
            "Contract",
            Some("owner".to_string()), // contract that can execute migrations
        )
        .unwrap();

    let bank_account: String = app
        .wrap()
        .query_wasm_smart(
            addr.clone(),
            &QueryMsg::GetBank {},
        )
        .unwrap();

    assert_eq!(
        bank_account,
        "bank".to_string()
    );


    (app, addr)
}
