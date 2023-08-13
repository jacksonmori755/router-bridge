use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, StdResult, Response, Binary, to_binary, Deps};
use router_wasm_bindings::{RouterQuery, RouterMsg, SudoMsg};

use crate::{msg::{InstantiateMsg, ExecuteMsg, QueryMsg}, state::NONCE, execute::{i_send, set_nonce, new_send}, sudo::{handle_sudo_ack, handle_sudo_request}};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    NONCE.save(deps.storage, &msg.nonce.clone())?;
    let response = Response::new()
        // .add_attribute("nonce", msg.nonce.to_string())
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
        ),
        ExecuteMsg::MySend {} => {
            let version = 0u64;
            let route_amount = 0u128;
            let route_recipient = "".to_string();
            let payload = Binary::from_base64("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAIHsicmVnaXN0ZXIiOiB7Im5hbWUiOiAidGVzdDIwIn19")?;
            let dest_chain_id = "osmo-test-5".to_string();
            let dest_contract_address = "osmo1zsp9c3tnlxzhqgpuvfqngz4sh52tp3cvjcscquzhfufvkfkwwymq2lgp49".to_string();
            let request_metadata = Binary::from_base64("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAwA=")?;
            i_send(
                deps,
                version, 
                route_amount, 
                route_recipient, 
                payload, 
                dest_chain_id, 
                dest_contract_address, 
                request_metadata, 
            )
        },
        ExecuteMsg::SetNonce { nonce } => set_nonce(deps, nonce),
        ExecuteMsg::NewSend { 
            version, 
            route_amount, 
            // route_recipient, 
            // dest_chain_id, 
            // dest_contract_address
         } => {
            new_send(deps, version, route_amount)//, route_recipient, dest_chain_id, dest_contract_address)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut<RouterQuery>, env: Env, msg: SudoMsg) -> StdResult<Response<RouterMsg>> {
    match msg {
        SudoMsg::HandleIAck { 
            request_identifier, 
            exec_flag, 
            exec_data, 
            refund_amount 
        } => handle_sudo_ack(
            deps,
            env,
            request_identifier, 
            exec_flag, 
            exec_data, 
            refund_amount 
        ),
        SudoMsg::HandleIReceive { 
            request_sender, 
            src_chain_id, 
            request_identifier, 
            payload 
        } => handle_sudo_request(
            deps,
            env,
            request_sender, 
            src_chain_id, 
            request_identifier, 
            payload 
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetNonce {} => Ok(to_binary(&to_binary(&NONCE.load(deps.storage)?)?)?)
    }
}
