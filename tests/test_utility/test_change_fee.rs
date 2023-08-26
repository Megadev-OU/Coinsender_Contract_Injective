#[cfg(test)]
mod tests {
    use crate::utils::instantiate_contract;

    use cosmwasm_std::{Addr, Uint128};
    use cw_multi_test::Executor;
    use cosmwasm_contracts::msg::{ExecuteMsg, QueryMsg};

    const PERCENT_DECIMALS: u128 = 3;

    #[test]
    fn test_change_fee_success() {
        let (mut app, addr) = instantiate_contract();

        app.execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &ExecuteMsg::ChangeFee {
                fee: Uint128::from(10 * PERCENT_DECIMALS), // 10%
            },
            &[],
        )
            .unwrap();

        let fee: Uint128 = app
            .wrap()
            .query_wasm_smart(
                addr.clone(),
                &QueryMsg::GetFee {},
            )
            .unwrap();

        assert_eq!(
            fee,
            Uint128::from(10 * PERCENT_DECIMALS) // 10%
        );
    }

    #[test]
    #[should_panic]
    fn test_change_fee_fail() {
        let (mut app, addr) = instantiate_contract();

        assert!(app.execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::ChangeFee {
                fee: Uint128::from(1u128)
            },
            &[],
        ).is_err());

        let fee: Uint128 = app
            .wrap()
            .query_wasm_smart(
                addr.clone(),
                &QueryMsg::GetFee {},
            )
            .unwrap();

        assert_eq!(
            fee,
            Uint128::from(1 * PERCENT_DECIMALS)
        );
    }
}