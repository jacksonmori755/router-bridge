use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, StdResult, Response};
use router_wasm_bindings::{RouterQuery, RouterMsg};

use crate::{msg::{InstantiateMsg, ExecuteMsg}, state::NONCE, execute::i_send};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    NONCE.save(deps.storage, &msg.nonce)?;
    let response = Response::new()
        .add_attribute("nonce", msg.nonce.to_string())
        .add_attribute("action", "bridge_contract_init");
    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<RouterMsg>> {
    match msg {
        ExecuteMsg::ISend { 
            version, 
            route_amount, 
            route_recipient, 
            payload, 
            dest_chain_id, 
            dest_contract_address, 
            request_metadata, 
        } => i_send(
            deps,
            version, 
            route_amount, 
            route_recipient, 
            payload, 
            dest_chain_id, 
            dest_contract_address, 
            request_metadata, 
        )
    }
}