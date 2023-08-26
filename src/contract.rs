use cosmwasm_std::CosmosMsg;
use {
    crate::{
        error::ContractError,
        msg::InstantiateMsg,
        msg::{ExecuteMsg, QueryMsg},
    },
    cosmwasm_std::{
        coins, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
        Uint128,
    },
};

use crate::contract::query::{get_bank, get_fee, get_owner};
use crate::state::{BANK_ADDRESS, FEE, OWNER_ADDRESS};

const CONTRACT_NAME: &str = "crates.io:cosmwasm-contracts";
const PERCENT_DECIMALS: u32 = 3;
const PERCENT_PRECISION: u128 = 10u128.pow(PERCENT_DECIMALS);


pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    OWNER_ADDRESS.save(deps.storage, &msg.owner)?;
    BANK_ADDRESS.save(deps.storage, &msg.bank)?;
    FEE.save(deps.storage, &msg.fee)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}


pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::TokenSender { recipient_amounts } => {
            if info.funds.is_empty() {
                return Err(ContractError::CustomError {
                    val: "No funds deposited!".to_string(),
                });
            }

            if info.funds.len() != 1 {
                return Err(ContractError::CustomError {
                    val: "You have to deposit one asset per time!".to_string(),
                });
            }

            let mut messages: Vec<CosmosMsg> = vec![];

            let deposited_token = info.funds.first().unwrap();
            let deposited_token_amount = deposited_token.amount.u128();

            let mut total_amount: u128 = 0;
            for (_, amount) in &recipient_amounts {
                total_amount += amount.u128()
            }

            let fee_amount = total_amount * get_fee(deps.as_ref(), env.clone()).unwrap().u128() / PERCENT_PRECISION;
            total_amount += fee_amount;


            if !(deposited_token_amount > 0 && total_amount <= deposited_token_amount) {
                return Err(ContractError::CustomError {
                    val: "Total Deposited Amount is Different From Total Multisend AMount!".to_string(),
                });
            }

            for (recipient, amount) in recipient_amounts {
                let recipient_checked
                    = deps.api.addr_validate(&recipient)?;

                messages.push(CosmosMsg::Bank(BankMsg::Send {
                    to_address: recipient_checked.to_string(),
                    amount: coins(amount.u128(), deposited_token.denom.clone()),
                }));
            }

            let bank_address = deps.api.addr_validate(&*get_bank(deps.as_ref(), env).unwrap())?;

            messages.push(CosmosMsg::Bank(BankMsg::Send {
                to_address: bank_address.to_string(),
                amount: coins(fee_amount.clone(), deposited_token.denom.clone()),

            }));

            Ok(Response::new().add_messages(messages))
        }
        ExecuteMsg::ChangeOwner { owner } => {
            assert_eq!(
                info.sender.to_string(),
                OWNER_ADDRESS.load(deps.storage).unwrap(),
                "This functionality is allowed for owner only"
            );

            OWNER_ADDRESS.save(
                deps.storage,
                &owner,
            )?;

            Ok(Response::default())
        }
        ExecuteMsg::ChangeBank { bank } => {
            assert_eq!(
                info.sender.to_string(),
                OWNER_ADDRESS.load(deps.storage).unwrap(),
                "This functionality is allowed for owner only"
            );

            BANK_ADDRESS.save(
                deps.storage,
                &bank,
            )?;

            Ok(Response::default())
        }
        ExecuteMsg::ChangeFee { fee } => {
            assert_eq!(
                info.sender.to_string(),
                OWNER_ADDRESS.load(deps.storage).unwrap(),
                "This functionality is allowed for owner only"
            );

            FEE.save(
                deps.storage,
                &fee,
            )?;

            Ok(Response::default())
        }
    }
}


pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => {
            to_binary(&get_owner(deps, env)?)
        }
        QueryMsg::GetBank {} => {
            to_binary(&get_bank(deps, env)?)
        }
        QueryMsg::GetFee {} => {
            to_binary(&get_fee(deps, env)?)
        }
    }
}


pub mod query {
    use crate::state::{BANK_ADDRESS, FEE, OWNER_ADDRESS};
    use super::*;


    pub fn get_owner(
        deps: Deps,
        _env: Env,
    ) -> StdResult<String> {
        OWNER_ADDRESS
            .load(deps.storage)
    }

    pub fn get_bank(
        deps: Deps,
        _env: Env,
    ) -> StdResult<String> {
        BANK_ADDRESS
            .load(deps.storage)
    }

    pub fn get_fee(
        deps: Deps,
        _env: Env,
    ) -> StdResult<Uint128> {
        FEE
            .load(deps.storage)
    }
}
