use cosmwasm_std::{DepsMut, Binary, Response, StdResult, Uint128, SubMsg, ReplyOn};
use router_wasm_bindings::{RouterMsg, RouterQuery, ethabi::{encode, Token}};

use crate::consts::I_SEND_REQUEST;

pub fn i_send(
    deps: DepsMut<RouterQuery>,
    version: u64, 
    route_amount: u128, 
    route_recipient: String, 
    payload: Binary, 
    dest_chain_id: String, 
    dest_contract_address: String, 
    request_metadata: Binary, 
) -> StdResult<Response<RouterMsg>> {
    let request_packet = encode(&[
        Token::String(dest_contract_address.clone()),
        Token::Bytes(payload.0)
    ]);
    let send_request = RouterMsg::CrosschainCall {
        version, 
        route_amount: Uint128::from(route_amount), 
        route_recipient, 
        dest_chain_id, 
        request_metadata: request_metadata.0, 
        request_packet 
    };
    let cross_chain_sub_msg = SubMsg {
        id: I_SEND_REQUEST,
        msg: send_request.into(),
        gas_limit: None,
        reply_on: ReplyOn::Never,
    };
    let response = Response::new()
        .add_submessage(cross_chain_sub_msg);
    Ok(response)
}