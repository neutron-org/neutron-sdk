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

use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use cosmos_sdk_proto::cosmos::staking::v1beta1::{MsgDelegate, MsgUndelegate};
use cosmos_sdk_proto::cosmos::tx::v1beta1::{TxBody, TxRaw};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, to_binary, Binary, Coin as CosmosCoin, CosmosMsg, CustomQuery, Deps, DepsMut, Env,
    MessageInfo, Reply, Response, StdError, StdResult, SubMsg,
};
use cw2::set_contract_version;
use prost::Message;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use neutron_sdk::bindings::msg::{IbcFee, MsgSubmitTxResponse, NeutronMsg};
use neutron_sdk::bindings::query::{
    InterchainQueries, QueryInterchainAccountAddressResponse, QueryRegisteredQueryResponse,
};
use neutron_sdk::bindings::types::ProtobufAny;
use neutron_sdk::interchain_queries::queries::{get_registered_query, query_balance};
use neutron_sdk::interchain_queries::types::{
    TransactionFilterItem, TransactionFilterOp, TransactionFilterValue,
    COSMOS_SDK_TRANSFER_MSG_URL, RECIPIENT_FIELD,
};
use neutron_sdk::interchain_queries::{
    new_register_balance_query_msg, new_register_transfers_query_msg,
};
use neutron_sdk::interchain_txs::helpers::{decode_acknowledgement_response, get_port_id};
use neutron_sdk::sudo::msg::{RequestPacket, SudoMsg};
use neutron_sdk::{NeutronError, NeutronResult};

use crate::storage::{
    read_reply_payload, read_sudo_payload, save_reply_payload, save_sudo_payload,
    AcknowledgementResult, GetRecipientTxsResponse, SudoPayload, Transfer, ACKNOWLEDGEMENT_RESULTS,
    IBC_FEE, INTERCHAIN_ACCOUNTS, RECIPIENT_TXS, SUDO_PAYLOAD_REPLY_ID,
};

// Default timeout for SubmitTX is two weeks
const DEFAULT_TIMEOUT_SECONDS: u64 = 60 * 60 * 24 * 7 * 2;

const CONTRACT_NAME: &str = concat!("crates.io:neutron-contracts__", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    deps.api.debug("WASMDEBUG: instantiate");
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _: MessageInfo,
    msg: ExecuteMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    deps.api
        .debug(format!("WASMDEBUG: execute: received msg: {:?}", msg).as_str());
    match msg {
        ExecuteMsg::Register {
            connection_id,
            interchain_account_id,
        } => execute_register_ica(deps, env, connection_id, interchain_account_id),
        ExecuteMsg::Delegate {
            validator,
            interchain_account_id,
            amount,
            denom,
            timeout,
        } => execute_delegate(
            deps,
            env,
            interchain_account_id,
            validator,
            amount,
            denom,
            timeout,
        ),
        ExecuteMsg::Undelegate {
            validator,
            interchain_account_id,
            amount,
            denom,
            timeout,
        } => execute_undelegate(
            deps,
            env,
            interchain_account_id,
            validator,
            amount,
            denom,
            timeout,
        ),
        ExecuteMsg::CleanAckResults {} => execute_clean_ack_results(deps),
        ExecuteMsg::SetFees {
            denom,
            recv_fee,
            ack_fee,
            timeout_fee,
        } => execute_set_fees(deps, recv_fee, ack_fee, timeout_fee, denom),
        ExecuteMsg::RegisterBalanceQuery {
            connection_id,
            addr,
            denom,
            update_period,
        } => register_balance_query(connection_id, addr, denom, update_period),
        ExecuteMsg::RegisterTransfersQuery {
            connection_id,
            recipient,
            update_period,
            min_height,
        } => register_transfers_query(connection_id, recipient, update_period, min_height),
        ExecuteMsg::RemoveInterchainQuery { query_id } => remove_interchain_query(query_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<InterchainQueries>, env: Env, msg: QueryMsg) -> NeutronResult<Binary> {
    match msg {
        QueryMsg::InterchainAccountAddress {
            interchain_account_id,
            connection_id,
        } => query_interchain_address(deps, env, interchain_account_id, connection_id),
        QueryMsg::InterchainAccountAddressFromContract {
            interchain_account_id,
        } => query_interchain_address_contract(deps, env, interchain_account_id),
        QueryMsg::AcknowledgementResult {
            interchain_account_id,
            sequence_id,
        } => query_acknowledgement_result(deps, env, interchain_account_id, sequence_id),
        QueryMsg::Balance { query_id } => Ok(to_binary(&query_balance(deps, env, query_id)?)?),
        QueryMsg::GetRecipientTxs { recipient } => query_recipient_txs(deps, recipient),
    }
}

pub fn query_interchain_address(
    deps: Deps<InterchainQueries>,
    env: Env,
    interchain_account_id: String,
    connection_id: String,
) -> NeutronResult<Binary> {
    let query = InterchainQueries::InterchainAccountAddress {
        owner_address: env.contract.address.to_string(),
        interchain_account_id,
        connection_id,
    };

    let res: QueryInterchainAccountAddressResponse = deps.querier.query(&query.into())?;
    Ok(to_binary(&res)?)
}

pub fn query_interchain_address_contract(
    deps: Deps<InterchainQueries>,
    env: Env,
    interchain_account_id: String,
) -> NeutronResult<Binary> {
    Ok(to_binary(&get_ica(deps, &env, &interchain_account_id)?)?)
}

pub fn query_acknowledgement_result(
    deps: Deps<InterchainQueries>,
    env: Env,
    interchain_account_id: String,
    sequence_id: u64,
) -> NeutronResult<Binary> {
    let port_id = get_port_id(env.contract.address.as_str(), &interchain_account_id);
    let res = ACKNOWLEDGEMENT_RESULTS.may_load(deps.storage, (port_id, sequence_id))?;
    Ok(to_binary(&res)?)
}

fn query_recipient_txs(deps: Deps<InterchainQueries>, recipient: String) -> NeutronResult<Binary> {
    let txs = RECIPIENT_TXS
        .load(deps.storage, &recipient)
        .unwrap_or_default();
    Ok(to_binary(&GetRecipientTxsResponse { transfers: txs })?)
}

fn msg_with_sudo_callback<C: Into<CosmosMsg<T>>, T>(
    deps: DepsMut,
    msg: C,
    payload: SudoPayload,
) -> StdResult<SubMsg<T>> {
    save_reply_payload(deps.storage, payload)?;
    Ok(SubMsg::reply_on_success(msg, SUDO_PAYLOAD_REPLY_ID))
}

fn get_fee_item(denom: String, amount: u128) -> Vec<CosmosCoin> {
    if amount == 0 {
        vec![]
    } else {
        vec![coin(amount, denom)]
    }
}

fn execute_set_fees(
    deps: DepsMut,
    recv_fee: u128,
    ack_fee: u128,
    timeout_fee: u128,
    denom: String,
) -> NeutronResult<Response<NeutronMsg>> {
    let fee = IbcFee {
        recv_fee: get_fee_item(denom.clone(), recv_fee),
        ack_fee: get_fee_item(denom.clone(), ack_fee),
        timeout_fee: get_fee_item(denom, timeout_fee),
    };

    IBC_FEE.save(deps.storage, &fee)?;

    Ok(Response::default())
}

fn execute_register_ica(
    deps: DepsMut,
    env: Env,
    connection_id: String,
    interchain_account_id: String,
) -> NeutronResult<Response<NeutronMsg>> {
    let register =
        NeutronMsg::register_interchain_account(connection_id, interchain_account_id.clone());
    let key = get_port_id(env.contract.address.as_str(), &interchain_account_id);
    INTERCHAIN_ACCOUNTS.save(deps.storage, key, &None)?;
    Ok(Response::new().add_message(register))
}

fn execute_delegate(
    mut deps: DepsMut,
    env: Env,
    interchain_account_id: String,
    validator: String,
    amount: u128,
    denom: String,
    timeout: Option<u64>,
) -> NeutronResult<Response<NeutronMsg>> {
    let fee = IBC_FEE.load(deps.storage)?;
    let (delegator, connection_id) = get_ica(deps.as_ref(), &env, &interchain_account_id)?;
    let delegate_msg = MsgDelegate {
        delegator_address: delegator,
        validator_address: validator,
        amount: Some(Coin {
            denom,
            amount: amount.to_string(),
        }),
    };
    let mut buf = Vec::with_capacity(delegate_msg.encoded_len());

    if let Err(e) = delegate_msg.encode(&mut buf) {
        return Err(NeutronError::Std(StdError::generic_err(format!(
            "Encode error: {}",
            e
        ))));
    }

    let any_msg = ProtobufAny {
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(),
        value: Binary::from(buf),
    };

    let cosmos_msg = NeutronMsg::submit_tx(
        connection_id,
        interchain_account_id.clone(),
        vec![any_msg],
        "".to_string(),
        timeout.unwrap_or(DEFAULT_TIMEOUT_SECONDS),
        fee,
    );

    // We use a submessage here because we need the process message reply to save
    // the outgoing IBC packet identifier for later.
    let submsg = msg_with_sudo_callback(
        deps.branch(),
        cosmos_msg,
        SudoPayload {
            port_id: get_port_id(env.contract.address.as_str(), &interchain_account_id),
            message: "message".to_string(),
            amount,
        },
    )?;

    Ok(Response::default().add_submessages(vec![submsg]))
}

fn execute_undelegate(
    mut deps: DepsMut,
    env: Env,
    interchain_account_id: String,
    validator: String,
    amount: u128,
    denom: String,
    timeout: Option<u64>,
) -> NeutronResult<Response<NeutronMsg>> {
    let fee = IBC_FEE.load(deps.storage)?;
    let (delegator, connection_id) = get_ica(deps.as_ref(), &env, &interchain_account_id)?;
    let delegate_msg = MsgUndelegate {
        delegator_address: delegator,
        validator_address: validator,
        amount: Some(Coin {
            denom,
            amount: amount.to_string(),
        }),
    };
    let mut buf = Vec::new();
    buf.reserve(delegate_msg.encoded_len());

    if let Err(e) = delegate_msg.encode(&mut buf) {
        return Err(NeutronError::Std(StdError::generic_err(format!(
            "Encode error: {}",
            e
        ))));
    }

    let any_msg = ProtobufAny {
        type_url: "/cosmos.staking.v1beta1.MsgUndelegate".to_string(),
        value: Binary::from(buf),
    };

    let cosmos_msg = NeutronMsg::submit_tx(
        connection_id,
        interchain_account_id.clone(),
        vec![any_msg],
        "".to_string(),
        timeout.unwrap_or(DEFAULT_TIMEOUT_SECONDS),
        fee,
    );

    let submsg = msg_with_sudo_callback(
        deps.branch(),
        cosmos_msg,
        SudoPayload {
            port_id: get_port_id(env.contract.address.as_str(), &interchain_account_id),
            message: "message".to_string(),
            amount,
        },
    )?;

    Ok(Response::default().add_submessages(vec![submsg]))
}

fn execute_clean_ack_results(deps: DepsMut) -> NeutronResult<Response<NeutronMsg>> {
    let keys: Vec<StdResult<(String, u64)>> = ACKNOWLEDGEMENT_RESULTS
        .keys(deps.storage, None, None, cosmwasm_std::Order::Descending)
        .collect();
    for key in keys {
        ACKNOWLEDGEMENT_RESULTS.remove(deps.storage, key?);
    }
    Ok(Response::default())
}

pub fn register_balance_query(
    connection_id: String,
    addr: String,
    denom: String,
    update_period: u64,
) -> NeutronResult<Response<NeutronMsg>> {
    let msg = new_register_balance_query_msg(connection_id, addr, denom, update_period)?;

    Ok(Response::new().add_message(msg))
}

pub fn register_transfers_query(
    connection_id: String,
    recipient: String,
    update_period: u64,
    min_height: Option<u64>,
) -> NeutronResult<Response<NeutronMsg>> {
    let msg =
        new_register_transfers_query_msg(connection_id, recipient, update_period, min_height)?;

    Ok(Response::new().add_message(msg))
}

pub fn remove_interchain_query(query_id: u64) -> NeutronResult<Response<NeutronMsg>> {
    let remove_msg = NeutronMsg::remove_interchain_query(query_id);
    Ok(Response::new().add_message(remove_msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut<InterchainQueries>, env: Env, msg: SudoMsg) -> NeutronResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: sudo: received sudo msg: {:?}", msg).as_str());
    match msg {
        SudoMsg::Response { request, data } => sudo_response(deps, request, data),
        SudoMsg::Error { request, details } => sudo_error(deps, request, details),
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
        SudoMsg::TxQueryResult {
            query_id,
            height,
            data,
        } => sudo_tx_query_result(deps, env, query_id, height, data),
        _ => Ok(Response::default()),
    }
}

fn sudo_open_ack(
    deps: DepsMut<InterchainQueries>,
    _env: Env,
    port_id: String,
    _channel_id: String,
    _counterparty_channel_id: String,
    counterparty_version: String,
) -> NeutronResult<Response> {
    let parsed_version: Result<OpenAckVersion, _> =
        serde_json_wasm::from_str(counterparty_version.as_str());
    if let Ok(parsed_version) = parsed_version {
        INTERCHAIN_ACCOUNTS.save(
            deps.storage,
            port_id,
            &Some((
                parsed_version.address,
                parsed_version.controller_connection_id,
            )),
        )?;
        return Ok(Response::default());
    }
    Err(NeutronError::Std(StdError::generic_err(
        "Can't parse counterparty_version",
    )))
}

pub fn sudo_tx_query_result(
    deps: DepsMut<InterchainQueries>,
    _env: Env,
    query_id: u64,
    _height: u64,
    data: Binary,
) -> NeutronResult<Response> {
    // Decode the transaction data
    let tx: TxRaw = TxRaw::decode(data.as_slice())?;
    let body: TxBody = TxBody::decode(tx.body_bytes.as_slice())?;

    // Get the registered query by ID and retrieve the raw query string
    let registered_query: QueryRegisteredQueryResponse =
        get_registered_query(deps.as_ref(), query_id)?;
    let transactions_filter = registered_query.registered_query.transactions_filter;

    #[allow(clippy::match_single_binding)]
    // Depending of the query type, check the transaction data to see whether is satisfies
    // the original query. If you don't write specific checks for a transaction query type,
    // all submitted results will be treated as valid.
    //
    // TODO: come up with solution to determine transactions filter type
    match registered_query.registered_query.query_type {
        _ => {
            // For transfer queries, query data looks like `[{"field:"transfer.recipient", "op":"eq", "value":"some_address"}]`
            let query_data: Vec<TransactionFilterItem> =
                serde_json_wasm::from_str(transactions_filter.as_str())?;

            let recipient = query_data
                .iter()
                .find(|x| x.field == RECIPIENT_FIELD && x.op == TransactionFilterOp::Eq)
                .map(|x| match &x.value {
                    TransactionFilterValue::String(v) => v.as_str(),
                    _ => "",
                })
                .unwrap_or("");

            let deposits = recipient_deposits_from_tx_body(body, recipient)?;
            // If we didn't find a Send message with the correct recipient, return an error, and
            // this query result will be rejected by Neutron: no data will be saved to state.
            if deposits.is_empty() {
                return Err(NeutronError::Std(StdError::generic_err(
                    "failed to find a matching transaction message",
                )));
            }

            let mut stored_deposits: Vec<Transfer> = RECIPIENT_TXS
                .load(deps.storage, recipient)
                .unwrap_or_default();
            stored_deposits.extend(deposits);
            RECIPIENT_TXS.save(deps.storage, recipient, &stored_deposits)?;
            Ok(Response::new())
        }
    }
}

fn recipient_deposits_from_tx_body(
    tx_body: TxBody,
    recipient: &str,
) -> NeutronResult<Vec<Transfer>> {
    let mut deposits: Vec<Transfer> = vec![];
    // Only handle up to MAX_ALLOWED_MESSAGES messages, everything else
    // will be ignored to prevent 'out of gas' conditions.
    // Note: in real contracts you will have to somehow save ignored
    // data in order to handle it later.
    for msg in tx_body.messages.iter().take(20) {
        // Skip all messages in this transaction that are not Send messages.
        if msg.type_url != *COSMOS_SDK_TRANSFER_MSG_URL.to_string() {
            continue;
        }

        // Parse a Send message and check that it has the required recipient.
        let transfer_msg: MsgSend = MsgSend::decode(msg.value.as_slice())?;
        if transfer_msg.to_address == recipient {
            for coin in transfer_msg.amount {
                deposits.push(Transfer {
                    sender: transfer_msg.from_address.clone(),
                    amount: coin.amount.clone(),
                    denom: coin.denom,
                    recipient: recipient.to_string(),
                });
            }
        }
    }
    Ok(deposits)
}

fn sudo_response(
    deps: DepsMut<InterchainQueries>,
    request: RequestPacket,
    data: Binary,
) -> NeutronResult<Response> {
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
    let channel_id = request
        .source_channel
        .ok_or_else(|| StdError::generic_err("channel_id not found"))?;
    let payload = read_sudo_payload(deps.storage, channel_id, seq_id)?;
    deps.api
        .debug(format!("WASMDEBUG: sudo_response: sudo payload: {:?}", payload).as_str());

    if payload.amount == 6666 {
        // This one is for testing contract failures
        deps.api.debug("WASMDEBUG: This is a expected test error");
        return Err(NeutronError::Std(StdError::generic_err(
            "This is a expected test error",
        )));
    }

    let parsed_data = decode_acknowledgement_response(data)?;

    let mut item_types = vec![];
    for item in parsed_data {
        let item_type = item.msg_type.as_str();
        item_types.push(item_type.to_string());
    }

    // update but also check that we don't update same seq_id twice
    ACKNOWLEDGEMENT_RESULTS.update(
        deps.storage,
        (payload.port_id, seq_id),
        |maybe_ack| -> StdResult<AcknowledgementResult> {
            match maybe_ack {
                Some(_ack) => Err(StdError::generic_err("trying to update same seq_id")),
                None => Ok(AcknowledgementResult::Success(item_types)),
            }
        },
    )?;

    Ok(Response::default())
}

fn sudo_timeout(
    deps: DepsMut<InterchainQueries>,
    _env: Env,
    request: RequestPacket,
) -> NeutronResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: sudo timeout request: {:?}", request).as_str());

    let seq_id = request
        .sequence
        .ok_or_else(|| StdError::generic_err("sequence not found"))?;
    let channel_id = request
        .source_channel
        .ok_or_else(|| StdError::generic_err("channel_id not found"))?;
    let payload = read_sudo_payload(deps.storage, channel_id, seq_id)?;

    // update but also check that we don't update same seq_id twice
    ACKNOWLEDGEMENT_RESULTS.update(
        deps.storage,
        (payload.port_id, seq_id),
        |maybe_ack| -> StdResult<AcknowledgementResult> {
            match maybe_ack {
                Some(_ack) => Err(StdError::generic_err("trying to update same seq_id")),
                None => Ok(AcknowledgementResult::Timeout(payload.message)),
            }
        },
    )?;

    Ok(Response::default())
}

fn sudo_error(
    deps: DepsMut<InterchainQueries>,
    request: RequestPacket,
    details: String,
) -> NeutronResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: sudo error: {}", details).as_str());
    deps.api
        .debug(format!("WASMDEBUG: request packet: {:?}", request).as_str());
    let seq_id = request
        .sequence
        .ok_or_else(|| StdError::generic_err("sequence not found"))?;
    let channel_id = request
        .source_channel
        .ok_or_else(|| StdError::generic_err("channel_id not found"))?;
    let payload = read_sudo_payload(deps.storage, channel_id, seq_id)?;

    // update but also check that we don't update same seq_id twice
    ACKNOWLEDGEMENT_RESULTS.update(
        deps.storage,
        (payload.port_id, seq_id),
        |maybe_ack| -> StdResult<AcknowledgementResult> {
            match maybe_ack {
                Some(_ack) => Err(StdError::generic_err("trying to update same seq_id")),
                None => Ok(AcknowledgementResult::Error((payload.message, details))),
            }
        },
    )?;

    Ok(Response::default())
}

fn prepare_sudo_payload(mut deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    let payload = read_reply_payload(deps.storage)?;
    let resp: MsgSubmitTxResponse = serde_json_wasm::from_slice(
        msg.result
            .into_result()
            .map_err(StdError::generic_err)?
            .data
            .ok_or_else(|| StdError::generic_err("no result"))?
            .as_slice(),
    )
    .map_err(|e| StdError::generic_err(format!("failed to parse response: {:?}", e)))?;
    deps.api
        .debug(format!("WASMDEBUG: reply msg: {:?}", resp).as_str());
    let seq_id = resp.sequence_id;
    let channel_id = resp.channel;
    save_sudo_payload(deps.branch().storage, channel_id, seq_id, payload)?;
    Ok(Response::new())
}

fn get_ica(
    deps: Deps<impl CustomQuery>,
    env: &Env,
    interchain_account_id: &str,
) -> Result<(String, String), StdError> {
    let key = get_port_id(env.contract.address.as_str(), interchain_account_id);

    INTERCHAIN_ACCOUNTS
        .load(deps.storage, key)?
        .ok_or_else(|| StdError::generic_err("Interchain account is not created yet"))
}

#[entry_point]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: reply msg: {:?}", msg).as_str());
    match msg.id {
        SUDO_PAYLOAD_REPLY_ID => prepare_sudo_payload(deps, env, msg),
        _ => Err(StdError::generic_err(format!(
            "unsupported reply message id {}",
            msg.id
        ))),
    }
}
