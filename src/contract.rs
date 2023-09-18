use cosmwasm_std::CosmosMsg;
use {
    crate::{
        error::ContractError,
        msg::InstantiateMsg,
        msg::{ExecuteMsg, QueryMsg},
    },
    cosmwasm_std::{
        coins, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    },
    cw_storage_plus::Item,
};

use crate::contract::query::get_bank;
pub const BANK_ADDRESS: Item<String> = Item::new("bank_address");

// const CONTRACT_NAME: &str = "crates.io:cosmwasm-contracts";
const PERCENT_DECIMALS: u32 = 3;
pub const PERCENT_PRECISION: u128 = 10u128.pow(PERCENT_DECIMALS);


pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    BANK_ADDRESS.save(deps.storage, &msg.bank)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}


pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::TokenSender { recipient_amounts , fee} => {
            
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

            let fee_amount =  total_amount * fee.u128() / PERCENT_PRECISION;
            total_amount += fee_amount;

            if !(deposited_token_amount > 0 && total_amount == deposited_token_amount) {
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
    }
}


pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBank {} => {
            to_binary(&get_bank(deps, env)?)
        }
    }
}


pub mod query {
    use super::*;

    pub fn get_bank(
        deps: Deps,
        _env: Env,
    ) -> StdResult<String> {
        BANK_ADDRESS
            .load(deps.storage)
    }
}
