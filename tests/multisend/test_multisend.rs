#[cfg(test)]
mod tests {    
    use crate::utils::instantiate_contract;

    use cosmwasm_std::{Addr, coins, Uint128};
    use cw_multi_test::Executor;
    use cosmwasm_contracts::msg::ExecuteMsg;

    const TOKENS_DECIMALS: u32 = 18;
    const PERCENT_PRECISION: u128 = 10u128.pow(3);

    const AMOUNT_TO_RECIPIENT_1: u128 = 10 * 10u128.pow(TOKENS_DECIMALS);
    const AMOUNT_TO_RECIPIENT_2: u128 = 10 * 10u128.pow(TOKENS_DECIMALS);

    const TOTAL_AMOUNT_FOR_RECIPIENTS: u128 = AMOUNT_TO_RECIPIENT_1 + AMOUNT_TO_RECIPIENT_2;

    #[test]
    fn test_success() {
        let (mut app, addr) = instantiate_contract();

        let user_balance_before =
            app.wrap()
                .query_balance("user", "eth")
                .unwrap()
                .amount
                .u128();

        let bank_balance_before =
            app.wrap()
                .query_balance("bank", "eth")
                .unwrap()
                .amount
                .u128();

        let recipient_1_balance_before =
            app.wrap()
                .query_balance("recipient_1", "eth")
                .unwrap()
                .amount
                .u128();

        let recipient_2_balance_before =
            app.wrap()
                .query_balance("recipient_2", "eth")
                .unwrap()
                .amount
                .u128();


        let fee = Uint128::new(10); // 1%
        let total_funds = TOTAL_AMOUNT_FOR_RECIPIENTS + (TOTAL_AMOUNT_FOR_RECIPIENTS * fee.u128() / PERCENT_PRECISION);

        app.execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::TokenSender {
                recipient_amounts:
                vec![("recipient_1".to_string(), Uint128::from(AMOUNT_TO_RECIPIENT_1)),
                     ("recipient_2".to_string(), Uint128::from(AMOUNT_TO_RECIPIENT_2)),
                ],
                fee
            },
            &coins(total_funds, "eth"),
        )
            .unwrap();


        let user_balance_after =
            app.wrap()
                .query_balance("user", "eth")
                .unwrap()
                .amount
                .u128();

        let bank_balance_after =
            app.wrap()
                .query_balance("bank", "eth")
                .unwrap()
                .amount
                .u128();

        let recipient_1_balance_after =
            app.wrap()
                .query_balance("recipient_1", "eth")
                .unwrap()
                .amount
                .u128();

        let recipient_2_balance_after =
            app.wrap()
                .query_balance("recipient_2", "eth")
                .unwrap()
                .amount
                .u128();

        assert_eq!(user_balance_before - TOTAL_AMOUNT_FOR_RECIPIENTS - TOTAL_AMOUNT_FOR_RECIPIENTS * fee.u128() / PERCENT_PRECISION, user_balance_after);

        assert_eq!(bank_balance_before + TOTAL_AMOUNT_FOR_RECIPIENTS * fee.u128() / PERCENT_PRECISION, bank_balance_after);

        assert_eq!(recipient_1_balance_before + AMOUNT_TO_RECIPIENT_1, recipient_1_balance_after);

        assert_eq!(recipient_2_balance_before + AMOUNT_TO_RECIPIENT_2, recipient_2_balance_after);
    }


    #[test]
    #[should_panic]
    fn test_fail_not_enough_deposited() {

        let (mut app, addr) = instantiate_contract();

        app.execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &ExecuteMsg::TokenSender {
                recipient_amounts:
                vec![("recipient_1".to_string(), Uint128::from(AMOUNT_TO_RECIPIENT_1)),
                     ("recipient_2".to_string(), Uint128::from(AMOUNT_TO_RECIPIENT_2)),
                ],
                fee: Uint128::new(1)
            },
            &coins(TOTAL_AMOUNT_FOR_RECIPIENTS, "eth"),
        )
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_fail_fee_too_big() {

        let (mut app, addr) = instantiate_contract();

        let fee = Uint128::new(51); // 5.1%
        let total_funds = TOTAL_AMOUNT_FOR_RECIPIENTS + (TOTAL_AMOUNT_FOR_RECIPIENTS * fee.u128() / PERCENT_PRECISION);

        app.execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &ExecuteMsg::TokenSender {
                recipient_amounts:
                vec![("recipient_1".to_string(), Uint128::from(AMOUNT_TO_RECIPIENT_1)),
                     ("recipient_2".to_string(), Uint128::from(AMOUNT_TO_RECIPIENT_2)),
                ],
                fee
            },
            &coins(total_funds, "eth"),
        )
            .unwrap();
    }

}


