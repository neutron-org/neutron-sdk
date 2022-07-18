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

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{Binary, DepsMut, Env, MessageInfo, Response, StdResult};

use cosmos_sdk_proto::cosmos::staking::v1beta1::MsgUndelegateResponse;
use interchain_txs::helpers::{parse_item, parse_response};
use interchain_txs::msg::SudoMsg;
use interchain_txs::storage::RequestPacket;

use crate::error::ContractResult;
use crate::msg::{InstantiateMsg, MigrateMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> ContractResult<Response> {
    //TODO
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> StdResult<Response> {
    match msg {
        SudoMsg::Response { request, data } => sudo_response(request, data),
        SudoMsg::Error { request, details } => sudo_error(request, details),
        SudoMsg::Timeout { request } => sudo_timeout(deps, env, request),
        SudoMsg::OpenAck {
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_version,
        } => sudo_open_ack(
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_version,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

fn sudo_response(_request: RequestPacket, data: String) -> StdResult<Response> {
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

fn sudo_timeout(_deps: DepsMut, _env: Env, _request: RequestPacket) -> StdResult<Response> {
    // return just data for debugging purposes for now
    let debug_response = "sudo_timeout zone_id=".to_string();
    Ok(Response::new().set_data(Binary::from(debug_response.as_bytes())))
}

//TODO: replace it with actual error handler
fn sudo_error(_request: RequestPacket, details: String) -> StdResult<Response> {
    let mut res = Response::new();
    res.data = Some(Binary::from(
        ("Handled error: ".to_string() + &details).as_bytes(),
    ));
    Ok(res)
}

//TODO: replace it with actual OpenAck handler
fn sudo_open_ack(
    port_id: String,
    channel_id: String,
    counterparty_channel_id: String,
    counterparty_version: String,
) -> StdResult<Response> {
    let mut res = Response::new();
    res.data = Some(Binary::from(
        ("Handled open ack: ".to_string()
            + &port_id
            + " "
            + &channel_id
            + " "
            + &counterparty_channel_id
            + " "
            + &counterparty_version)
            .as_bytes(),
    ));
    Ok(res)
}
