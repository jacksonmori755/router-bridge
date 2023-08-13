use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Coin};

#[cw_serde]
pub struct InstantiateMsg {
    pub nonce: u64
}

// pub enum SudoMsg {
//     // HandleIReceive {
//     //     request_sender: String,
//     //     src_chain_id: String,
//     //     request_identifier: u64,
//     //     payload: Binary,
//     // },
//     HandleIAck {
//         request_identifier: u64,
//         exec_flag: bool,
//         exec_data: Binary,
//         refund_amount: Coin,
//     },
// }


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
    MySend {
    },
    NewSend{
        version: u64,
        route_amount: u128,
        // route_recipient: String,
        // dest_chain_id: String,
        // dest_contract_address: String,
    },
    SetNonce {
        nonce: Binary
    }
}

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    #[returns(Binary)]
    GetNonce {
    }
}


