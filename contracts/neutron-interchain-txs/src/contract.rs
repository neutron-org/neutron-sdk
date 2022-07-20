// Copyright 2022 Neutron
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use cosmos_sdk_proto::cosmos::staking::v1beta1::{MsgDelegate, MsgUndelegateResponse};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, to_vec, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply,
    Response, StdError, StdResult, Storage, SubMsg,
};
use interchain_txs::helpers::{parse_item, parse_response};
use interchain_txs::msg::SudoMsg;
use interchain_txs::storage::RequestPacket;
use prost::Message;
use prost_types::Any;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractResult;
use crate::msg::{ExecuteMsg, MsgSubmitTx};
use crate::msg::{InstantiateMsg, MigrateMsg};
use crate::storage::{
    IBC_SUDO_ID_RANGE_END, IBC_SUDO_ID_RANGE_START, REPLY_QUEUE_ID, SUDO_PAYLOAD,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SudoPayload {
    pub message: String,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> ContractResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<MsgSubmitTx>> {
    deps.api
        .debug(format!("WASMDEBUG: execute: received msg: {:?}", msg).as_str());
    match msg {
        ExecuteMsg::Delegate { from, to, amount } => execute_delegate(deps, env, from, to, amount),
    }
}

pub fn get_next_id(store: &mut dyn Storage) -> StdResult<u64> {
    REPLY_QUEUE_ID
        .keys(store, None, None, cosmwasm_std::Order::Descending)
        .next()
        .unwrap_or(Ok(IBC_SUDO_ID_RANGE_START))
        .map(|id| id + 1)
}

pub fn save_reply_payload(store: &mut dyn Storage, payload: SudoPayload) -> StdResult<u64> {
    let id = get_next_id(store)?;
    REPLY_QUEUE_ID.save(store, id, &to_vec(&payload)?)?;
    Ok(id)
}

pub fn read_reply_payload(store: &mut dyn Storage, id: u64) -> StdResult<SudoPayload> {
    let data = REPLY_QUEUE_ID.load(store, id)?;
    from_binary(&Binary(data))
}

pub fn read_sudo_payload(store: &mut dyn Storage, seq_id: u64) -> StdResult<SudoPayload> {
    let data = SUDO_PAYLOAD.load(store, seq_id)?;
    from_binary(&Binary(data))
}

fn msg_with_sudo_callback<C: Into<CosmosMsg<T>>, T>(
    deps: DepsMut,
    msg: C,
    payload: SudoPayload,
) -> StdResult<SubMsg<T>> {
    let id = save_reply_payload(deps.storage, payload)?;
    Ok(SubMsg::reply_on_success(msg, id))
}

fn execute_delegate(
    mut deps: DepsMut,
    env: Env,
    from: String,
    to: String,
    amount: u128,
) -> StdResult<Response<MsgSubmitTx>> {
    let delegate_msg = MsgDelegate {
        delegator_address: from,
        validator_address: to,
        amount: Some(Coin {
            denom: "stake".to_string(),
            amount: amount.to_string(),
        }),
    };
    let mut buf = Vec::new();
    buf.reserve(delegate_msg.encoded_len());

    if let Err(e) = delegate_msg.encode(&mut buf) {
        return Err(StdError::generic_err(format!("Encode error: {}", e)));
    }

    let any_msg = Any {
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(),
        value: buf,
    };

    let mut buf = Vec::new();
    buf.reserve(any_msg.encoded_len());
    if let Err(e) = delegate_msg.encode(&mut buf) {
        return Err(StdError::generic_err(format!("Encode error: {}", e)));
    }

    let cosmos_msg = CosmosMsg::Custom(MsgSubmitTx {
        from_address: env.contract.address.to_string(),
        connection_id: "connection-0".to_string(),
        msgs: vec![to_binary(&buf)?],
        owner: env.contract.address.to_string(),
        memo: "".to_string(),
    });

    let submsg = msg_with_sudo_callback(
        deps.branch(),
        cosmos_msg,
        SudoPayload {
            message: "message".to_string(),
        },
    )?;

    Ok(Response::default().add_submessages(vec![submsg]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> StdResult<Response> {
    match msg {
        SudoMsg::Response { request, data } => sudo_response(deps, request, data),
        SudoMsg::Error { request, details } => sudo_error(deps.as_ref(), request, details),
        SudoMsg::Timeout { request } => sudo_timeout(deps, env, request),
        _ => Ok(Response::default()),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

fn sudo_response(deps: DepsMut, request: RequestPacket, data: String) -> StdResult<Response> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_response: sudo received: {:?} {:?}",
            request, data
        )
        .as_str(),
    );
    let seq_id = request
        .sequence
        .ok_or_else(|| StdError::generic_err("sequence not found"))?;

    let payload = read_sudo_payload(deps.storage, seq_id)?;
    deps.api
        .debug(format!("WASMDEBUG: sudo_response: sudo payload: {:?}", payload).as_str());
    // handle response
    Ok(Response::default())
}

fn _sudo_response(_request: RequestPacket, data: String) -> StdResult<Response> {
    let mut res = Response::new();
    let parsed_data = parse_response(data)?;
    for item in parsed_data {
        let item_type = item.msg_type.as_str();
        // //TODO: cover all possible msg_types and handle them properly
        match item_type {
            "/cosmos.staking.v1beta1.MsgUndelegate" => {
                let out: MsgUndelegateResponse = parse_item(&item.data)?;
                let completion_time = out.completion_time;
                match completion_time {
                    Some(c) => {
                        res.data = Some(Binary::from(
                            (item.msg_type + " " + &c.seconds.to_string()).as_bytes(),
                        ));
                    }
                    None => {
                        res.data = Some(Binary::from(
                            (item.msg_type + " no completion time").as_bytes(),
                        ));
                    }
                }
            }
            _ => {
                res.data = Some(Binary::from("unknown request".as_bytes()));
            }
        }
    }
    Ok(res)
}

fn sudo_timeout(deps: DepsMut, _env: Env, request: RequestPacket) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: sudo timeout request: {:?}", request).as_str());
    Ok(Response::default())
}

/// Here you can handle error
fn sudo_error(deps: Deps, _request: RequestPacket, details: String) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: sudo error: {}", details).as_str());
    Ok(Response::default())
}

pub fn save_sudo_payload(
    store: &mut dyn Storage,
    seq_id: u64,
    payload: SudoPayload,
) -> StdResult<()> {
    SUDO_PAYLOAD.save(store, seq_id, &to_vec(&payload)?)
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
