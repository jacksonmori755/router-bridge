use cosmwasm_std::{DepsMut, Binary, Response, StdResult, Uint128, SubMsg, ReplyOn, to_binary, Event, from_binary};
use router_wasm_bindings::{RouterMsg, RouterQuery, ethabi::{encode, Token}};

use crate::{consts::I_SEND_REQUEST, state::NONCE};

pub fn i_send(
    _deps: DepsMut<RouterQuery>,
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

pub fn set_nonce(deps: DepsMut<RouterQuery>, nonce_bin: Binary) -> StdResult<Response<RouterMsg>> {
    let nonce: u64 = from_binary(&nonce_bin)?;
    NONCE.save(deps.storage, &nonce)?;
    let result_str = format!("set_nonce, nonce: {}", nonce.clone());
    let result = to_binary(&result_str)?;
    let response = Response::new()
        .set_data(result)
        .add_event(
            Event::new("set_nonce")
                .add_attribute("nonce", nonce.to_string())   
        );
    Ok(response)
}

pub fn new_send(
    _deps: DepsMut<RouterQuery>,
    version: u64, 
    route_amount: u128, 
    // route_recipient: String, 
    // dest_chain_id: String, 
    // dest_contract_address: String, 
) -> StdResult<Response<RouterMsg>> {
    let route_recipient = "".to_string();
    let payload = Binary::from_base64("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAIHsicmVnaXN0ZXIiOiB7Im5hbWUiOiAidGVzdDIwIn19")?;
    let dest_chain_id = "osmo-test-5".to_string();
    let dest_contract_address = "osmo1zsp9c3tnlxzhqgpuvfqngz4sh52tp3cvjcscquzhfufvkfkwwymq2lgp49".to_string();
    let request_metadata = Binary::from_base64("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAwA=")?;
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