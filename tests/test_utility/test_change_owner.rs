#[cfg(test)]
mod tests {
    use crate::utils::instantiate_contract;

    use cosmwasm_std::{Addr};
    use cw_multi_test::Executor;
    use cosmwasm_contracts::msg::{ExecuteMsg, QueryMsg};

    #[test]
    fn test_change_owner_success() {
        let (mut app, addr) = instantiate_contract();

        app.execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &ExecuteMsg::ChangeOwner {
                owner: "new_owner".to_string()
            },
            &[],
        )
            .unwrap();


        let owner_account: String = app
            .wrap()
            .query_wasm_smart(
                addr.clone(),
                &QueryMsg::GetOwner {},
            )
            .unwrap();

        assert_eq!(
            owner_account,
            "new_owner".to_string()
        );
    }

    #[test]
    #[should_panic]
    fn test_change_owner_fail() {
        let (mut app, addr) = instantiate_contract();

        assert!(app.execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::ChangeOwner {
                owner: "new_owner".to_string()
            },
            &[],
        ).is_err());

        let owner_account: String = app
            .wrap()
            .query_wasm_smart(
                addr.clone(),
                &QueryMsg::GetOwner {},
            )
            .unwrap();

        assert_eq!(
            owner_account,
            "owner".to_string()
        );
    }
}