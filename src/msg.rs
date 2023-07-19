use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct InstantiateMsg {
    pub nonce: u64
}

#[cw_serde]
pub enum ExecuteMsg {
    ISend {
        version: u64,
        route_amount: u128,
        route_recipient: String,
        payload: Binary,
        dest_chain_id: String,
        dest_contract_address: String,
        request_metadata: Binary,
    },
}

