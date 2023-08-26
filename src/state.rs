use {
    cosmwasm_std::Uint128,
    cw_storage_plus::Item,
};

pub const BANK_ADDRESS: Item<String> = Item::new("bank_address");

pub const OWNER_ADDRESS: Item<String> = Item::new("owner_address");

// with a precision of 10^3 f.e. 1 * 10^PERCENT_DECIMALS
// PERCENT_DECIMALS = 3
pub const FEE: Item<Uint128> = Item::new("fee");
