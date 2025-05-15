use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::storage::{
    add_error_to_queue, read_errors_from_queue, read_reply_payload, read_sudo_payload,
    save_reply_payload, save_sudo_payload, AcknowledgementResult, SudoPayload,
    ACKNOWLEDGEMENT_RESULTS, INTERCHAIN_ACCOUNTS, SUDO_PAYLOAD_REPLY_ID,
};
use cosmos_sdk_proto::traits::Message;
use cosmwasm_std::{
    to_json_binary, Binary, CosmosMsg, CustomQuery, Deps, DepsMut, Env, MessageInfo, Reply,
    Response, StdError, StdResult, SubMsg,
};
use cw2::set_contract_version;
use neutron_sdk::interchain_txs::helpers::{
    query_denom_min_ibc_fee, register_interchain_account, submit_tx,
};
use neutron_sdk::{
    interchain_txs::helpers::{decode_message_response, get_port_id},
    interchain_txs::v047::helpers::decode_acknowledgement_response,
    sudo::msg::{RequestPacket, SudoMsg},
    NeutronError, NeutronResult,
};
use neutron_std::shim::Timestamp;
use neutron_std::types::cosmos::base::v1beta1::Coin;
use neutron_std::types::cosmos::staking::v1beta1::{
    MsgDelegate, MsgDelegateResponse, MsgUndelegate, MsgUndelegateResponse,
};
use neutron_std::types::ibc::core::channel::v1::Order;
use neutron_std::types::neutron::interchaintxs::v1::{InterchaintxsQuerier, MsgSubmitTxResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Default timeout for SubmitTX is two weeks
const DEFAULT_TIMEOUT_SECONDS: u64 = 60 * 60 * 24 * 7 * 2;
const FEE_DENOM: &str = "untrn";

const CONTRACT_NAME: &str = concat!("crates.io:neutron-sdk__", env!("CARGO_PKG_NAME"));
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

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> NeutronResult<Response> {
    deps.api.debug("WASMDEBUG: instantiate");
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _: MessageInfo,
    msg: ExecuteMsg,
) -> NeutronResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: execute: received msg: {:?}", msg).as_str());
    match msg {
        ExecuteMsg::Register {
            connection_id,
            interchain_account_id,
            register_fee,
            ordering,
        } => execute_register_ica(
            deps,
            env,
            connection_id,
            interchain_account_id,
            register_fee,
            ordering,
        ),
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
    }
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> NeutronResult<Binary> {
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
        QueryMsg::ErrorsQueue {} => query_errors_queue(deps),
    }
}

// returns ICA address from Neutron ICA SDK module
pub fn query_interchain_address(
    deps: Deps,
    env: Env,
    interchain_account_id: String,
    connection_id: String,
) -> NeutronResult<Binary> {
    let querier = InterchaintxsQuerier::new(&deps.querier);
    let res = querier.interchain_account_address(
        env.contract.address.to_string(),
        interchain_account_id,
        connection_id,
    )?;

    Ok(to_json_binary(&res)?)
}

// returns ICA address from the contract storage. The address was saved in sudo_open_ack method
pub fn query_interchain_address_contract(
    deps: Deps,
    env: Env,
    interchain_account_id: String,
) -> NeutronResult<Binary> {
    Ok(to_json_binary(&get_ica(
        deps,
        &env,
        &interchain_account_id,
    )?)?)
}

// returns the result
pub fn query_acknowledgement_result(
    deps: Deps,
    env: Env,
    interchain_account_id: String,
    sequence_id: u64,
) -> NeutronResult<Binary> {
    let port_id = get_port_id(env.contract.address.as_str(), &interchain_account_id);
    let res = ACKNOWLEDGEMENT_RESULTS.may_load(deps.storage, (port_id, sequence_id))?;
    Ok(to_json_binary(&res)?)
}

pub fn query_errors_queue(deps: Deps) -> NeutronResult<Binary> {
    let res = read_errors_from_queue(deps.storage)?;
    Ok(to_json_binary(&res)?)
}

// saves payload to process later to the storage and returns a SubmitTX Cosmos SubMsg with necessary reply id
fn msg_with_sudo_callback<C: Into<CosmosMsg<T>>, T>(
    deps: DepsMut,
    msg: C,
    payload: SudoPayload,
) -> StdResult<SubMsg<T>> {
    save_reply_payload(deps.storage, payload)?;
    Ok(SubMsg::reply_on_success(msg, SUDO_PAYLOAD_REPLY_ID))
}

fn execute_register_ica(
    deps: DepsMut,
    env: Env,
    connection_id: String,
    interchain_account_id: String,
    register_fee: Vec<Coin>,
    ordering: Option<Order>,
) -> NeutronResult<Response> {
    let register = register_interchain_account(
        env.contract.address.clone(),
        connection_id,
        interchain_account_id.clone(),
        register_fee,
        ordering,
    );
    let key = get_port_id(&env.contract.address.to_string(), &interchain_account_id);
    // we are saving empty data here because we handle response of registering ICA in sudo_open_ack method
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
) -> NeutronResult<Response> {
    // contract must pay for relaying of acknowledgements
    // See more info here: https://docs.neutron.org/neutron/feerefunder/overview
    let fee = query_denom_min_ibc_fee(deps.as_ref(), FEE_DENOM)?;
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

    let any_msg = neutron_std::shim::Any {
        type_url: "/cosmos.staking.v1beta1.MsgDelegateResponse".to_string(),
        value: buf,
    };

    let cosmos_msg = submit_tx(
        env.contract.address.clone(),
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
) -> NeutronResult<Response> {
    // contract must pay for relaying of acknowledgements
    // See more info here: https://docs.neutron.org/neutron/feerefunder/overview
    let fee = query_denom_min_ibc_fee(deps.as_ref(), FEE_DENOM)?;
    let (delegator, connection_id) = get_ica(deps.as_ref(), &env, &interchain_account_id)?;
    let delegate_msg = MsgUndelegate {
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

    let any_msg = neutron_std::shim::Any {
        type_url: "/cosmos.staking.v1beta1.MsgUndelegate".to_string(),
        value: buf,
    };

    let cosmos_msg = submit_tx(
        env.contract.address.clone(),
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
        },
    )?;

    Ok(Response::default().add_submessages(vec![submsg]))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: sudo: received sudo msg: {:?}", msg).as_str());

    match msg {
        // For handling successful (non-error) acknowledgements.
        SudoMsg::Response { request, data } => sudo_response(deps, request, data),

        // For handling error acknowledgements.
        SudoMsg::Error { request, details } => sudo_error(deps, request, details),

        // For handling error timeouts.
        SudoMsg::Timeout { request } => sudo_timeout(deps, env, request),

        // For handling successful registering of ICA
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
        _ => Ok(Response::default()),
    }
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    deps.api.debug("WASMDEBUG: migrate");
    Ok(Response::default())
}

// handler
fn sudo_open_ack(
    deps: DepsMut,
    _env: Env,
    port_id: String,
    _channel_id: String,
    _counterparty_channel_id: String,
    counterparty_version: String,
) -> StdResult<Response> {
    // The version variable contains a JSON value with multiple fields,
    // including the generated account address.
    let parsed_version: Result<OpenAckVersion, _> =
        serde_json_wasm::from_str(counterparty_version.as_str());

    // Update the storage record associated with the interchain account.
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
    Err(StdError::generic_err("Can't parse counterparty_version"))
}

fn sudo_response(deps: DepsMut, request: RequestPacket, data: Binary) -> StdResult<Response> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_response: sudo received: {:?} {:?}",
            request, data
        )
        .as_str(),
    );

    // In this particular case, we return an error because not having the sequence id
    // in the request value implies that a fatal error occurred on Neutron side.
    let seq_id = request
        .sequence
        .ok_or_else(|| StdError::generic_err("sequence not found"))?;

    // In this particular case, we return an error because not having the sequence id
    // in the request value implies that a fatal error occurred on Neutron side.
    let channel_id = request
        .source_channel
        .ok_or_else(|| StdError::generic_err("channel_id not found"))?;

    // In this particular example, this is a matter of developer's choice. Not being able to read
    // the payload here means that there was a problem with the contract while submitting an
    // interchain transaction. You can decide that this is not worth killing the channel,
    // write an error log and / or save the acknowledgement to an errors queue for later manual
    // processing. The decision is based purely on your application logic.
    let payload = read_sudo_payload(deps.storage, channel_id, seq_id).ok();
    if payload.is_none() {
        let error_msg = "WASMDEBUG: Error: Unable to read sudo payload";
        deps.api.debug(error_msg);
        add_error_to_queue(deps.storage, error_msg.to_string());
        return Ok(Response::default());
    }

    deps.api
        .debug(format!("WASMDEBUG: sudo_response: sudo payload: {:?}", payload).as_str());

    // In this particular case, we return an error because not being able to parse this data
    // that a fatal error occurred on Neutron side, or that the remote chain sent us unexpected data.
    // Both cases require immediate attention.
    let parsed_data = decode_acknowledgement_response(data)?;

    let mut item_types = vec![];
    for item in parsed_data {
        let item_type = item.type_url.as_str();
        item_types.push(item_type.to_string());
        match item_type {
            "/cosmos.staking.v1beta1.MsgUndelegate" => {
                // In this particular case, a mismatch between the string message type and the
                // serialised data layout looks like a fatal error that has to be investigated.
                let out: MsgUndelegateResponse = decode_message_response(&item.value)?;

                // In this particular case, we demonstrate that minor errors should not
                // close the channel, and should be treated in a forgiving manner.
                let completion_time = out.completion_time.or_else(|| {
                    let error_msg = "WASMDEBUG: sudo_response: Recoverable error. Failed to get completion time";
                    deps.api
                        .debug(error_msg);
                    add_error_to_queue(deps.storage, error_msg.to_string());
                    Some(Timestamp::default())
                });
                deps.api
                    .debug(format!("Undelegation completion time: {:?}", completion_time).as_str());
            }
            "/cosmos.staking.v1beta1.MsgDelegateResponse" => {
                // In this particular case, a mismatch between the string message type and the
                // serialised data layout looks like a fatal error that has to be investigated.
                let _out: MsgDelegateResponse = decode_message_response(&item.value)?;
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

    if let Some(payload) = payload {
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
    }

    Ok(Response::default())
}

fn sudo_timeout(deps: DepsMut, _env: Env, request: RequestPacket) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: sudo timeout request: {:?}", request).as_str());

    // In this particular case, we return an error because not having the sequence id
    // in the request value implies that a fatal error occurred on Neutron side.
    let seq_id = request
        .sequence
        .ok_or_else(|| StdError::generic_err("sequence not found"))?;

    // In this particular case, we return an error because not having the sequence id
    // in the request value implies that a fatal error occurred on Neutron side.
    let channel_id = request
        .source_channel
        .ok_or_else(|| StdError::generic_err("channel_id not found"))?;

    // update but also check that we don't update same seq_id twice
    // In this particular example, this is a matter of developer's choice. Not being able to read
    // the payload here means that there was a problem with the contract while submitting an
    // interchain transaction. You can decide that this is not worth killing the channel,
    // write an error log and / or save the acknowledgement to an errors queue for later manual
    // processing. The decision is based purely on your application logic.
    // Please be careful because it may lead to an unexpected state changes because state might
    // has been changed before this call and will not be reverted because of suppressed error.
    let payload = read_sudo_payload(deps.storage, channel_id, seq_id).ok();
    if let Some(payload) = payload {
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
    } else {
        let error_msg = "WASMDEBUG: Error: Unable to read sudo payload";
        deps.api.debug(error_msg);
        add_error_to_queue(deps.storage, error_msg.to_string());
    }

    Ok(Response::default())
}

fn sudo_error(deps: DepsMut, request: RequestPacket, details: String) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: sudo error: {}", details).as_str());
    deps.api
        .debug(format!("WASMDEBUG: request packet: {:?}", request).as_str());

    // In this particular case, we return an error because not having the sequence id
    // in the request value implies that a fatal error occurred on Neutron side.
    let seq_id = request
        .sequence
        .ok_or_else(|| StdError::generic_err("sequence not found"))?;

    // In this particular case, we return an error because not having the sequence id
    // in the request value implies that a fatal error occurred on Neutron side.
    let channel_id = request
        .source_channel
        .ok_or_else(|| StdError::generic_err("channel_id not found"))?;
    let payload = read_sudo_payload(deps.storage, channel_id, seq_id).ok();

    if let Some(payload) = payload {
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
    } else {
        let error_msg = "WASMDEBUG: Error: Unable to read sudo payload";
        deps.api.debug(error_msg);
        add_error_to_queue(deps.storage, error_msg.to_string());
    }

    Ok(Response::default())
}

// prepare_sudo_payload is called from reply handler
// The method is used to extract sequence id and channel from SubmitTxResponse to process sudo payload defined in msg_with_sudo_callback later in Sudo handler.
// Such flow msg_with_sudo_callback() -> reply() -> prepare_sudo_payload() -> sudo() allows you "attach" some payload to your SubmitTx message
// and process this payload when an acknowledgement for the SubmitTx message is received in Sudo handler
fn prepare_sudo_payload(mut deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    let payload = read_reply_payload(deps.storage)?;
    let resp: MsgSubmitTxResponse = decode_message_response(
        &msg.result
            .into_result()
            .map_err(StdError::generic_err)?
            .msg_responses[0] // msg_responses must have exactly one Msg response: https://github.com/neutron-org/neutron/blob/28b1d2ce968aaf1866e92d5286487f079eba3370/wasmbinding/message_plugin.go#L443
            .clone()
            .value
            .to_vec(),
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

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
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
