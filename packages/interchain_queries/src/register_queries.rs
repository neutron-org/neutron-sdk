use crate::error::ContractResult;
use crate::storage::TMP_REGISTER_INTERCHAIN_QUERY_REQUEST;
use crate::types::{
    GetBalanceQueryParams, GetDelegatorDelegationsParams, GetTransfersParams, TmpRegisteredQuery,
    QUERY_BALANCE_QUERY_TYPE, QUERY_DELEGATOR_DELEGATIONS_QUERY_TYPE, QUERY_TRANSFERS,
    REGISTER_INTERCHAIN_QUERY_REPLY_ID,
};
use cosmwasm_std::{DepsMut, Env, Response, StdError, SubMsg};
use neutron_bindings::NeutronMsg;
use schemars::_serde_json::to_string;
use serde::Serialize;

/// Registers an interchain query
fn register_interchain_query<T>(
    deps: DepsMut,
    _env: Env,
    connection_id: String,
    zone_id: String,
    update_period: u64,
    query_type: &str,
    query_data: &T,
) -> ContractResult<Response<NeutronMsg>>
where
    T: ?Sized + Serialize,
{
    let query_data_json_encoded =
        to_string(&query_data).map_err(|e| StdError::generic_err(e.to_string()))?;

    let register_msg = NeutronMsg::register_interchain_query(
        String::from(query_type),
        query_data_json_encoded.clone(),
        zone_id.clone(),
        connection_id.clone(),
        update_period,
    );

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
            register_msg,
            REGISTER_INTERCHAIN_QUERY_REPLY_ID,
        )))
}

/// Registers an interchain query to get balance of account on remote chain for particular denom
pub fn register_balance_query(
    deps: DepsMut,
    env: Env,
    connection_id: String,
    zone_id: String,
    addr: String,
    denom: String,
    update_period: u64,
) -> ContractResult<Response<NeutronMsg>> {
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
    deps: DepsMut,
    env: Env,
    connection_id: String,
    zone_id: String,
    delegator: String,
    update_period: u64,
) -> ContractResult<Response<NeutronMsg>> {
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
    deps: DepsMut,
    env: Env,
    connection_id: String,
    zone_id: String,
    recipient: String,
    update_period: u64,
) -> ContractResult<Response<NeutronMsg>> {
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
