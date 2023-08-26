#[cfg(test)]
mod tests {
    use crate::utils::instantiate_contract;

    use cosmwasm_std::{Addr};
    use cw_multi_test::Executor;
    use cosmwasm_contracts::msg::{ExecuteMsg, QueryMsg};

    #[test]
    fn test_change_bank_success() {
        let (mut app, addr) = instantiate_contract();

        app.execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &ExecuteMsg::ChangeBank {
                bank: "new_bank".to_string()
            },
            &[],
        )
            .unwrap();

        let bank: String = app
            .wrap()
            .query_wasm_smart(
                addr.clone(),
                &QueryMsg::GetBank {},
            )
            .unwrap();

        assert_eq!(
            bank,
            "new_bank".to_string()
        );
    }

    #[test]
    #[should_panic]
    fn test_change_bank_fail() {
        let (mut app, addr) = instantiate_contract();

        assert!(app.execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::ChangeBank {
                bank: "new_bank".to_string()
            },
            &[],
        ).is_err());

        let bank: String = app
            .wrap()
            .query_wasm_smart(
                addr.clone(),
                &QueryMsg::GetBank {},
            )
            .unwrap();

        assert_eq!(
            bank,
            "bank".to_string()
        );
    }
}