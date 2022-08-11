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
use cosmos_sdk_proto::cosmos::staking::v1beta1::{
    MsgDelegate, MsgUndelegate, MsgUndelegateResponse,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, to_vec, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply,
    Response, StdError, StdResult, Storage, SubMsg,
};
use interchain_txs::helpers::{parse_item, parse_response};
use interchain_txs::msg::SudoMsg;
use interchain_txs::storage::RequestPacket;
use neutron_bindings::msg::NeutronMsg;
use neutron_bindings::query::InterchainQueries;

use neutron_bindings::query::QueryInterchainAccountAddressResponse;
use neutron_bindings::ProtobufAny;
use prost::Message;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractResult;
use crate::msg::{InstantiateMsg, MigrateMsg};
use crate::storage::{
    Config, CONFIG, IBC_SUDO_ID_RANGE_END, IBC_SUDO_ID_RANGE_START, REPLY_QUEUE_ID, SUDO_PAYLOAD,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Ica {},
    Config {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
struct OpenAckVersion {
    version: String,
    controller_connection_id: String,
    host_connection_id: String,
    address: String,
    encoding: String,
    tx_type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Delegate { validator: String, amount: u128 },
    Undelegate { validator: String, amount: u128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SudoPayload {
    pub message: String,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response<NeutronMsg>> {
    let register = NeutronMsg::register_interchain_account(
        msg.connection_id.clone(),
        msg.interchain_account_id.clone(),
    );
    CONFIG.save(
        deps.storage,
        &Config {
            connection_id: msg.connection_id,
            interchain_account_id: msg.interchain_account_id,
            denom: msg.denom,
            interchain_address: None,
        },
    )?;
    Ok(Response::new().add_message(register))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<NeutronMsg>> {
    deps.api
        .debug(format!("WASMDEBUG: execute: received msg: {:?}", msg).as_str());
    match msg {
        ExecuteMsg::Delegate { validator, amount } => {
            execute_delegate(deps, env, validator, amount)
        }
        ExecuteMsg::Undelegate { validator, amount } => {
            execute_undelegate(deps, env, validator, amount)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<InterchainQueries>, env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    match msg {
        QueryMsg::Ica {} => query_interchain_address(deps, env),
        QueryMsg::Config {} => query_config(deps),
    }
}

pub fn query_config(deps: Deps<InterchainQueries>) -> ContractResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    Ok(to_binary(&config)?)
}

pub fn query_interchain_address(deps: Deps<InterchainQueries>, env: Env) -> ContractResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    let query = InterchainQueries::InterchainAccountAddress {
        owner_address: env.contract.address.to_string(),
        interchain_account_id: config.interchain_account_id,
        connection_id: config.connection_id,
    };

    let res: QueryInterchainAccountAddressResponse = deps.querier.query(&query.into())?;

    Ok(to_binary(&res)?)
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
    _env: Env,
    validator: String,
    amount: u128,
) -> StdResult<Response<NeutronMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let delegator = config
        .interchain_address
        .ok_or_else(|| StdError::generic_err("Interchain account is not created yet"))?;

    let delegate_msg = MsgDelegate {
        delegator_address: delegator,
        validator_address: validator,
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

    let any_msg = ProtobufAny {
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(),
        value: to_binary(&buf)?,
    };

    let cosmos_msg = NeutronMsg::submit_tx(
        config.connection_id,
        config.interchain_account_id,
        vec![any_msg],
        "".to_string(),
    );

    let submsg = msg_with_sudo_callback(
        deps.branch(),
        cosmos_msg,
        SudoPayload {
            message: "message".to_string(),
        },
    )?;

    Ok(Response::default().add_submessages(vec![submsg]))
}

fn execute_undelegate(
    mut deps: DepsMut,
    _env: Env,
    validator: String,
    amount: u128,
) -> StdResult<Response<NeutronMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let delegator = config
        .interchain_address
        .ok_or_else(|| StdError::generic_err("Interchain account is not created yet"))?;
    let delegate_msg = MsgUndelegate {
        delegator_address: delegator,
        validator_address: validator,
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

    let any_msg = ProtobufAny {
        type_url: "/cosmos.staking.v1beta1.MsgUndelegate".to_string(),
        value: to_binary(&buf)?,
    };

    let cosmos_msg = NeutronMsg::submit_tx(
        config.connection_id,
        config.interchain_account_id,
        vec![any_msg],
        "".to_string(),
    );

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
        SudoMsg::OpenAck {
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_version,
        } => sudo_open_ack(
            deps,
            env,
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

fn sudo_open_ack(
    deps: DepsMut,
    _env: Env,
    _port_id: String,
    _channel_id: String,
    _counterparty_channel_id: String,
    counterparty_version: String,
) -> StdResult<Response> {
    let mut config = CONFIG.load(deps.storage)?;
    let parsed_version: Result<OpenAckVersion, _> =
        serde_json_wasm::from_str(counterparty_version.as_str());
    if let Ok(parsed_version) = parsed_version {
        config.interchain_address = Some(parsed_version.address);
        CONFIG.save(deps.storage, &config)?;
        return Ok(Response::default());
    }
    Err(StdError::generic_err("Can't parse counterparty_version"))
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
    let parsed_data = parse_response(data)?;

    for item in parsed_data {
        let item_type = item.msg_type.as_str();
        match item_type {
            "/cosmos.staking.v1beta1.MsgUndelegate" => {
                let out: MsgUndelegateResponse = parse_item(&item.data)?;
                let completion_time = out
                    .completion_time
                    .ok_or_else(|| StdError::generic_err("failed to get completion time"))?;
                deps.api
                    .debug(format!("Undelegation completion time: {:?}", completion_time).as_str());
            }
            _ => {
                deps.api.debug(
                    format!(
                        "This type of acknowledgement is not implemented: {:?}",
                        payload
                    )
                    .as_str(),
                );
            }
        }
    }

    Ok(Response::default())
}

fn sudo_timeout(deps: DepsMut, _env: Env, request: RequestPacket) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: sudo timeout request: {:?}", request).as_str());
    Ok(Response::default())
}

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
