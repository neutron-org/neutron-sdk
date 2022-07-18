use cosmwasm_std::{
    coin, entry_point, CosmosMsg, Deps, DepsMut, Env, IbcMsg, IbcTimeout, IbcTimeoutBlock,
    MessageInfo, Reply, Response, StdError, StdResult, SubMsg,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{
    read_reply_payload, read_sudo_payload, save_reply_payload, save_sudo_payload,
    IBC_SUDO_ID_RANGE_END, IBC_SUDO_ID_RANGE_START,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    deps.api.debug("WASMDEBUG: Instatntiate");
    Ok(Response::default())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Send { to: String, amount: u128 },
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, _: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: execute: received msg: {:?}", msg).as_str());
    match msg {
        ExecuteMsg::Send { to, amount } => execute_send(deps, to, amount),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Type1 {
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Type2 {
    pub data: String,
}

fn sudo_callback1(deps: Deps, payload: Type1) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: callback1: sudo payload: {:?}", payload).as_str());
    Ok(Response::new())
}

fn sudo_callback2(deps: Deps, payload: Type2) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: callback2: sudo payload: {:?}", payload).as_str());
    Ok(Response::new())
}

#[derive(Serialize, Deserialize)]
pub enum SudoPayload {
    HandlerPayload1(Type1),
    HandlerPayload2(Type2),
}

fn msg_with_sudo_callback<C: Into<CosmosMsg>>(
    deps: DepsMut,
    msg: C,
    payload: SudoPayload,
) -> StdResult<SubMsg> {
    let id = save_reply_payload(deps.storage, payload)?;
    Ok(SubMsg::reply_on_success(msg, id))
}

fn parse_sequence(deps: Deps, msg: Reply) -> StdResult<u64> {
    let seq_id = str::parse(
        &msg.result
            .into_result()
            .map_err(StdError::generic_err)?
            .events
            .iter()
            .find(|e| e.ty == "send_packet")
            .and_then(|e| e.attributes.iter().find(|a| a.key == "packet_sequence"))
            .ok_or_else(|| StdError::generic_err("failed to find packet_sequence atribute"))?
            .value
            .clone(),
    )
    .map_err(|_e| StdError::generic_err("parse int error"))?;
    deps.api
        .debug(format!("WASMDEBUG: parse_sequence: reply result: {:?}", seq_id).as_str());
    Ok(seq_id)
}

fn prepare_sudo_payload(mut deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    let payload = read_reply_payload(deps.storage, msg.id)?;
    let seq_id = parse_sequence(deps.as_ref(), msg)?;
    save_sudo_payload(deps.branch().storage, seq_id, payload)?;
    Ok(Response::new())
}

#[entry_point]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        IBC_SUDO_ID_RANGE_START..=IBC_SUDO_ID_RANGE_END => prepare_sudo_payload(deps, env, msg),
        _ => Err(StdError::generic_err(format!(
            "unsupported reply message id {}",
            msg.id
        ))),
    }
}

fn execute_send(mut deps: DepsMut, to: String, amount: u128) -> StdResult<Response> {
    let coin1 = coin(amount, "stake");
    let msg1: CosmosMsg = CosmosMsg::Ibc(IbcMsg::Transfer {
        // transfer channel
        channel_id: "channel-0".to_string(),
        // "to" is an address on the counterpart chain
        to_address: to.clone(),
        amount: coin1,
        timeout: IbcTimeout::with_block(IbcTimeoutBlock {
            revision: 2,
            height: 10000000,
        }),
    });
    let coin2 = coin(2 * amount, "stake");
    let msg2: CosmosMsg = CosmosMsg::Ibc(IbcMsg::Transfer {
        // transfer channel
        channel_id: "channel-0".to_string(),
        // "to" is an address on the counterpart chain
        to_address: to,
        amount: coin2,
        timeout: IbcTimeout::with_block(IbcTimeoutBlock {
            revision: 2,
            height: 10000000,
        }),
    });
    let submsg1 = msg_with_sudo_callback(
        deps.branch(),
        msg1,
        SudoPayload::HandlerPayload1(Type1 {
            message: "message".to_string(),
        }),
    )?;
    let submsg2 = msg_with_sudo_callback(
        deps.branch(),
        msg2,
        SudoPayload::HandlerPayload2(Type2 {
            data: "data".to_string(),
        }),
    )?;
    deps.as_ref()
        .api
        .debug(format!("WASMDEBUG: execute_send: sent submsg1: {:?}", submsg1).as_str());
    deps.api
        .debug(format!("WASMDEBUG: execute_send: sent submsg2: {:?}", submsg2).as_str());
    Ok(Response::default().add_submessages(vec![submsg1, submsg2]))
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RequestPacket {
    sequence: Option<u64>,
    source_port: Option<String>,
    source_channel: Option<String>,
    destination_port: Option<String>,
    destination_channel: Option<String>,
    data: Option<String>,
    timeout_height: Option<RequestPacketTimeoutHeight>,
    timeout_timestamp: Option<u64>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RequestPacketTimeoutHeight {
    revision_number: Option<u64>,
    revision_height: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    Response {
        request: RequestPacket,
        data: String,
    },
}

#[entry_point]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> StdResult<Response> {
    match msg {
        SudoMsg::Response { request, data } => sudo_response(deps, request, data),
    }
}

fn sudo_response(deps: DepsMut, req: RequestPacket, data: String) -> StdResult<Response> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_response: sudo received: {:?} {:?}",
            req, data
        )
        .as_str(),
    );
    let seq_id = req
        .sequence
        .ok_or_else(|| StdError::generic_err("sequence not found"))?;
    match read_sudo_payload(deps.storage, seq_id)? {
        SudoPayload::HandlerPayload1(t1) => sudo_callback1(deps.as_ref(), t1),
        SudoPayload::HandlerPayload2(t2) => sudo_callback2(deps.as_ref(), t2),
    }
}
