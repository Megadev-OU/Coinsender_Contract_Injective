use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub bank: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    TokenSender {
        // amount with a 18 precision
        recipient_amounts: Vec<(String, Uint128)>,
        fee: Uint128
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {

    #[returns(String)]
    GetBank {},
}