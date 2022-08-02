use crate::custom_queries::InterchainQueries;
use crate::error::ContractResult;
use crate::storage::TMP_REGISTER_INTERCHAIN_QUERY_REQUEST;
use crate::types::{
    GetBalanceQueryParams, GetDelegatorDelegationsParams, GetTransfersParams, TmpRegisteredQuery,
    QUERY_BALANCE_QUERY_TYPE, QUERY_DELEGATOR_DELEGATIONS_QUERY_TYPE, QUERY_TRANSFERS,
    REGISTER_INTERCHAIN_QUERY_PATH, REGISTER_INTERCHAIN_QUERY_REPLY_ID,
};
use cosmwasm_std::{Binary, CosmosMsg, DepsMut, Env, Response, StdError, SubMsg};
use protobuf::Message;
use schemars::_serde_json::to_string;
use serde::Serialize;
use stargate::interchain::interchainqueries_tx::MsgRegisterInterchainQuery;

/// Registers an interchain query
fn register_interchain_query<T>(
    deps: DepsMut<InterchainQueries>,
    env: Env,
    connection_id: String,
    zone_id: String,
    update_period: u64,
    query_type: &str,
    query_data: &T,
) -> ContractResult<Response>
where
    T: ?Sized + Serialize,
{
    let query_data_json_encoded =
        to_string(&query_data).map_err(|e| StdError::generic_err(e.to_string()))?;

    let mut register_msg = MsgRegisterInterchainQuery::new();
    register_msg.query_data = query_data_json_encoded.clone();
    register_msg.query_type = String::from(query_type);
    register_msg.update_period = update_period;
    register_msg.connection_id = connection_id.clone();
    register_msg.zone_id = zone_id.clone();
    register_msg.sender = env.contract.address.to_string();

    let encoded_msg_bytes = register_msg.write_to_bytes()?;
    let encoded_register_msg = Binary::from(encoded_msg_bytes);

    let msg: CosmosMsg = CosmosMsg::Stargate {
        type_url: REGISTER_INTERCHAIN_QUERY_PATH.to_string(),
        value: encoded_register_msg,
    };

    // We need to know registered_query_id that is returned in MsgRegisterInterchainQuery execution
    // so we temporarily save all necessary data to use it in reply handler to save returned query id
    TMP_REGISTER_INTERCHAIN_QUERY_REQUEST.save(
        deps.storage,
        &TmpRegisteredQuery {
            connection_id: connection_id.clone(),
            zone_id: zone_id.clone(),
            query_type: query_type.to_string(),
            query_data: query_data_json_encoded.clone(),
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "register_interchain_query")
        .add_attribute("connection_id", connection_id.as_str())
        .add_attribute("zone_id", zone_id.as_str())
        .add_attribute("query_type", query_type)
        .add_attribute("update_period", update_period.to_string())
        .add_attribute("query_data", query_data_json_encoded.as_str())
        .add_submessage(SubMsg::reply_on_success(
            msg,
            REGISTER_INTERCHAIN_QUERY_REPLY_ID,
        )))
}

/// Registers an interchain query to get balance of account on remote chain for particular denom
pub fn register_balance_query(
    deps: DepsMut<InterchainQueries>,
    env: Env,
    connection_id: String,
    zone_id: String,
    addr: String,
    denom: String,
    update_period: u64,
) -> ContractResult<Response> {
    let query_data = GetBalanceQueryParams { addr, denom };

    register_interchain_query(
        deps,
        env,
        connection_id,
        zone_id,
        update_period,
        QUERY_BALANCE_QUERY_TYPE,
        &query_data,
    )
}

/// Registers an interchain query to get delegations of particular delegator on remote chain
pub fn register_delegator_delegations_query(
    deps: DepsMut<InterchainQueries>,
    env: Env,
    connection_id: String,
    zone_id: String,
    delegator: String,
    update_period: u64,
) -> ContractResult<Response> {
    let query_data = GetDelegatorDelegationsParams { delegator };

    register_interchain_query(
        deps,
        env,
        connection_id,
        zone_id,
        update_period,
        QUERY_DELEGATOR_DELEGATIONS_QUERY_TYPE,
        &query_data,
    )
}

/// Registers an interchain query to get transfer events to a recipient on a remote chain
pub fn register_transfers_query(
    deps: DepsMut<InterchainQueries>,
    env: Env,
    connection_id: String,
    zone_id: String,
    recipient: String,
    update_period: u64,
) -> ContractResult<Response> {
    let query_data = GetTransfersParams {
        recipient,
        ..Default::default()
    };

    register_interchain_query(
        deps,
        env,
        connection_id,
        zone_id,
        update_period,
        QUERY_TRANSFERS,
        &query_data,
    )
}
