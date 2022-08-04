use crate::error::ContractResult;
use crate::types::{
    create_account_balances_prefix, create_delegation_key, create_validator_key,
    decode_and_convert, GetTransfersParams, QueryType, BANK_STORE_KEY, STAKING_STORE_KEY,
};
use cosmwasm_std::{DepsMut, Env, Response, StdError};
use neutron_bindings::msg::NeutronMsg;
use neutron_bindings::query::InterchainQueries;
use neutron_bindings::types::{KVKey, KVKeys};
use schemars::_serde_json::to_string;

#[allow(clippy::too_many_arguments)]
/// Registers an interchain query
fn register_interchain_query(
    _deps: DepsMut<InterchainQueries>,
    _env: Env,
    connection_id: String,
    zone_id: String,
    query_type: QueryType,
    kv_keys: Vec<KVKey>,
    transactions_filter: String,
    update_period: u64,
) -> ContractResult<Response<NeutronMsg>> {
    let register_msg = NeutronMsg::register_interchain_query(
        query_type.into(),
        kv_keys.clone(),
        transactions_filter.clone(),
        zone_id.clone(),
        connection_id.clone(),
        update_period,
    );

    Ok(Response::new()
        .add_attribute("action", "register_interchain_query")
        .add_attribute("connection_id", connection_id.as_str())
        .add_attribute("zone_id", zone_id.as_str())
        .add_attribute("query_type", query_type)
        .add_attribute("update_period", update_period.to_string())
        .add_attribute("transactions_filter", transactions_filter.as_str())
        .add_attribute("kv_keys", KVKeys(kv_keys))
        .add_message(register_msg))
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
) -> ContractResult<Response<NeutronMsg>> {
    let converted_addr_bytes = decode_and_convert(addr.as_str())?;

    let mut balance_key = create_account_balances_prefix(&converted_addr_bytes)?;
    balance_key.extend_from_slice(denom.as_bytes());

    let kv_key = KVKey {
        path: BANK_STORE_KEY.to_string(),
        key: balance_key,
    };

    register_interchain_query(
        deps,
        env,
        connection_id,
        zone_id,
        QueryType::KV,
        vec![kv_key],
        String::new(),
        update_period,
    )
}

/// Registers an interchain query to get delegations of particular delegator on remote chain
pub fn register_delegator_delegations_query(
    deps: DepsMut<InterchainQueries>,
    env: Env,
    connection_id: String,
    zone_id: String,
    delegator: String,
    validators: Vec<String>,
    update_period: u64,
) -> ContractResult<Response<NeutronMsg>> {
    let delegator_addr = decode_and_convert(delegator.as_str())?;

    let mut keys: Vec<KVKey> = Vec::with_capacity(validators.len() * 2);

    for v in &validators {
        let val_addr = decode_and_convert(v.as_str())?;
        keys.push(KVKey {
            path: STAKING_STORE_KEY.to_string(),
            key: create_delegation_key(&delegator_addr, &val_addr)?,
        });
        keys.push(KVKey {
            path: STAKING_STORE_KEY.to_string(),
            key: create_validator_key(&val_addr)?,
        })
    }

    register_interchain_query(
        deps,
        env,
        connection_id,
        zone_id,
        QueryType::KV,
        keys,
        String::new(),
        update_period,
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
) -> ContractResult<Response<NeutronMsg>> {
    let query_data = GetTransfersParams {
        recipient,
        ..Default::default()
    };

    let query_data_json_encoded =
        to_string(&query_data).map_err(|e| StdError::generic_err(e.to_string()))?;

    register_interchain_query(
        deps,
        env,
        connection_id,
        zone_id,
        QueryType::TX,
        vec![],
        query_data_json_encoded,
        update_period,
    )
}
