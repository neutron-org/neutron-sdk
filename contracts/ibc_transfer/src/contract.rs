use crate::{
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg},
    state::{
        read_reply_payload, read_sudo_payload, save_reply_payload, save_sudo_payload,
        IBC_SUDO_ID_RANGE_END, IBC_SUDO_ID_RANGE_START,
    },
};
use cosmwasm_std::{
    entry_point, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
    StdResult, SubMsg,
};
use cw2::set_contract_version;
use neutron_sdk::interchain_txs::helpers::{decode_message_response, query_denom_min_ibc_fee};
use neutron_sdk::sudo::msg::{RequestPacket, TransferSudoMsg};
use neutron_std::types::cosmos::base::v1beta1::Coin as SDKCoin;
use neutron_std::types::neutron::transfer::{MsgTransfer, MsgTransferResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Default timeout for IbcTransfer is 10000000 blocks
const DEFAULT_TIMEOUT_HEIGHT: u64 = 10000000;
const FEE_DENOM: &str = "untrn";

const CONTRACT_NAME: &str = concat!("crates.io:neutron-sdk__", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, _: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        // NOTE: this is an example contract that shows how to make IBC transfers!
        // Please add necessary authorization or other protection mechanisms
        // if you intend to send funds over IBC
        ExecuteMsg::Send {
            channel,
            to,
            denom,
            amount,
            timeout_height,
        } => execute_send(deps, env, channel, to, denom, amount, timeout_height),
    }
}

// Example of different payload types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Type1 {
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Type2 {
    pub data: String,
}

// a callback handler for payload of Type1
fn sudo_callback1(deps: Deps, payload: Type1) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: callback1: sudo payload: {:?}", payload).as_str());
    Ok(Response::new())
}

// a callback handler for payload of Type2
fn sudo_callback2(deps: Deps, payload: Type2) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: callback2: sudo payload: {:?}", payload).as_str());
    Ok(Response::new())
}

// Enum representing payload to process during handling acknowledgement messages in Sudo handler
#[derive(Serialize, Deserialize)]
pub enum SudoPayload {
    HandlerPayload1(Type1),
    HandlerPayload2(Type2),
}

// saves payload to process later to the storage and returns a SubmitTX Cosmos SubMsg with necessary reply id
fn msg_with_sudo_callback<C: Into<CosmosMsg<T>>, T>(
    deps: DepsMut,
    msg: C,
    payload: SudoPayload,
) -> StdResult<SubMsg<T>> {
    let id = save_reply_payload(deps.storage, payload)?;
    Ok(SubMsg::reply_on_success(msg, id))
}

// prepare_sudo_payload is called from reply handler
// The method is used to extract sequence id and channel from SubmitTxResponse to process sudo payload defined in msg_with_sudo_callback later in Sudo handler.
// Such flow msg_with_sudo_callback() -> reply() -> prepare_sudo_payload() -> sudo() allows you "attach" some payload to your Transfer message
// and process this payload when an acknowledgement for the SubmitTx message is received in Sudo handler
fn prepare_sudo_payload(mut deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    let payload = read_reply_payload(deps.storage, msg.id)?;
    let resp: MsgTransferResponse = decode_message_response(
        &msg.result
            .into_result()
            .map_err(StdError::generic_err)?
            .msg_responses[0] // msg_responses must have exactly one Msg response: https://github.com/neutron-org/neutron/blob/28b1d2ce968aaf1866e92d5286487f079eba3370/wasmbinding/message_plugin.go#L307
            .clone()
            .value
            .to_vec(),
    )
    .map_err(|e| StdError::generic_err(format!("failed to parse response: {:?}", e)))?;
    let seq_id = resp.sequence_id;
    let channel_id = resp.channel;
    save_sudo_payload(deps.branch().storage, channel_id, seq_id, payload)?;
    Ok(Response::new())
}

#[entry_point]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        // It's convenient to use range of ID's to handle multiple reply messages
        IBC_SUDO_ID_RANGE_START..=IBC_SUDO_ID_RANGE_END => prepare_sudo_payload(deps, env, msg),
        _ => Err(StdError::generic_err(format!(
            "unsupported reply message id {}",
            msg.id
        ))),
    }
}

fn execute_send(
    mut deps: DepsMut,
    env: Env,
    channel: String,
    to: String,
    denom: String,
    amount: u128,
    timeout_height: Option<u64>,
) -> StdResult<Response> {
    // contract must pay for relaying of acknowledgements
    // See more info here: https://docs.neutron.org/neutron/feerefunder/overview
    let fee = query_denom_min_ibc_fee(deps.as_ref(), FEE_DENOM)?;
    let msg1 = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: channel.clone(),
        sender: env.contract.address.to_string(),
        receiver: to.clone(),
        token: Some(SDKCoin {
            denom: denom.clone(),
            amount: amount.to_string(),
        }),
        timeout_height: Some(neutron_std::types::ibc::core::client::v1::Height {
            revision_number: 2,
            revision_height: timeout_height.unwrap_or(DEFAULT_TIMEOUT_HEIGHT),
        }),
        timeout_timestamp: 0,
        memo: "".to_string(),
        fee: Some(fee.clone()),
    };
    let msg2 = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: channel,
        sender: env.contract.address.to_string(),
        receiver: to,
        token: Some(SDKCoin {
            denom,
            amount: (2 * amount).to_string(),
        }),
        timeout_height: Some(neutron_std::types::ibc::core::client::v1::Height {
            revision_number: 2,
            revision_height: timeout_height.unwrap_or(DEFAULT_TIMEOUT_HEIGHT),
        }),
        timeout_timestamp: 0,
        memo: "".to_string(),
        fee: Some(fee.clone()),
    };
    // prepare first transfer message with payload of Type1
    let submsg1 = msg_with_sudo_callback(
        deps.branch(),
        msg1,
        SudoPayload::HandlerPayload1(Type1 {
            message: "message".to_string(),
        }),
    )?;
    // prepare second transfer message with payload of Type2
    // both messages have different reply ids, which allows to send them in one tx and handle both replies separately
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

#[entry_point]
pub fn sudo(deps: DepsMut, _env: Env, msg: TransferSudoMsg) -> StdResult<Response> {
    match msg {
        // For handling successful (non-error) acknowledgements
        TransferSudoMsg::Response { request, data } => sudo_response(deps, request, data),

        // For handling error acknowledgements
        TransferSudoMsg::Error { request, details } => sudo_error(deps, request, details),

        // For handling error timeouts
        TransferSudoMsg::Timeout { request } => sudo_timeout(deps, request),
    }
}

fn sudo_error(deps: DepsMut, req: RequestPacket, data: String) -> StdResult<Response> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_error: sudo error received: {:?} {}",
            req, data
        )
        .as_str(),
    );
    Ok(Response::new())
}

fn sudo_timeout(deps: DepsMut, req: RequestPacket) -> StdResult<Response> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_timeout: sudo timeout ack received: {:?}",
            req
        )
        .as_str(),
    );
    Ok(Response::new())
}

fn sudo_response(deps: DepsMut, req: RequestPacket, data: Binary) -> StdResult<Response> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_response: sudo received: {:?} {}",
            req, data
        )
        .as_str(),
    );
    let seq_id = req
        .sequence
        .ok_or_else(|| StdError::generic_err("sequence not found"))?;
    let channel_id = req
        .source_channel
        .ok_or_else(|| StdError::generic_err("channel_id not found"))?;

    match read_sudo_payload(deps.storage, channel_id, seq_id)? {
        // here we can do different logic depending on the type of the payload we saved in msg_with_sudo_callback() call
        // This allows us to distinguish different transfer message from each other.
        // For example some protocols can send one transfer to refund user for some action and another transfer to top up some balance.
        // Such different actions may require different handling of their responses.
        SudoPayload::HandlerPayload1(t1) => sudo_callback1(deps.as_ref(), t1),
        SudoPayload::HandlerPayload2(t2) => sudo_callback2(deps.as_ref(), t2),
    }
    // at this place we can safely remove the data under (channel_id, seq_id) key
    // but it costs an extra gas, so its on you how to use the storage
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    deps.api.debug("WASMDEBUG: migrate");
    Ok(Response::default())
}
