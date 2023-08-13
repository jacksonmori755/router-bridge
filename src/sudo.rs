use cosmwasm_std::{StdResult, Response, Binary, Coin, DepsMut, Env, StdError, to_binary};
use router_wasm_bindings::{RouterMsg, RouterQuery, ethabi::{ParamType, decode, encode, Token}};

pub fn handle_sudo_ack(
    deps: DepsMut<RouterQuery>,
    env: Env,   
    request_identifier: u64, 
    exec_flag: bool, 
    exec_data: Binary, 
    refund_amount: Coin 
) -> StdResult<Response<RouterMsg>> {
    let decoded = match decode(&[ParamType::String], &exec_data.0) {
        Ok(data) => data,
        Err(_) => {
            return Err(StdError::GenericErr {
                msg: String::from("can not decode exec_data"),
            });
        }
    };
    let exec_data_str = decoded[0].clone().into_string().unwrap();
    let result_txt = format!("handle_sudo_ack, exec_data: {}, request_identifier: {}, exec_flag: {}", 
        exec_data_str, request_identifier.to_string(), exec_flag.to_string());
    let result_vec = encode(&[Token::String(result_txt)]);
    let result = Binary::from(result_vec);
    let response = Response::new()
        .set_data(result);
    Ok(response)
}

pub fn handle_sudo_request(
    deps: DepsMut<RouterQuery>,
    env: Env,
    request_sender: String, 
    src_chain_id: String, 
    request_identifier: u64, 
    payload: Binary 
) -> StdResult<Response<RouterMsg>> {
    let decoded = match decode(&[ParamType::String], &payload.0) {
        Ok(data) => data,
        Err(_) => {
            return Err(StdError::GenericErr {
                msg: String::from("can not decode exec_data"),
            });
        }
    };
    let payload_str = decoded[0].clone().into_string().unwrap();
    let result_txt = format!("handle_sudo_request, request_identifier: {}, src_chain_id: {}, request_sender: {}, payload: {}",
        request_identifier.to_string(), src_chain_id, request_sender, payload_str);
    let result_vec = encode(&[Token::String(result_txt)]);
    let result = Binary::from(result_vec);
    let response = Response::new()
        .set_data(result);
    Ok(response)
}
