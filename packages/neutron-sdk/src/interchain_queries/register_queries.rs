use crate::bindings::msg::NeutronMsg;
use crate::bindings::query::InterchainQueries;
use crate::bindings::types::KVKey;
use crate::errors::error::NeutronResult;
use crate::interchain_queries::helpers::{
    create_account_denom_balance_key, create_delegation_key, create_params_store_key,
    create_validator_key, decode_and_convert,
};
use crate::interchain_queries::types::{
    QueryType, TransactionFilterItem, TransactionFilterOp, TransactionFilterValue, BANK_STORE_KEY,
    HEIGHT_FIELD, KEY_BOND_DENOM, PARAMS_STORE_KEY, RECIPIENT_FIELD, STAKING_STORE_KEY,
};
use cosmwasm_std::{Binary, DepsMut, Env, StdError};
use schemars::_serde_json::to_string;

use super::types::QueryPayload;

#[allow(clippy::too_many_arguments)]
/// Creates a message to register an Interchain Query with provided params
fn new_interchain_query_msg(
    _deps: DepsMut<InterchainQueries>,
    _env: Env,
    connection_id: String,
    query_type: QueryType,
    kv_keys: Vec<KVKey>,
    transactions_filter: String,
    update_period: u64,
) -> NeutronResult<NeutronMsg> {
    match query_type {
        QueryType::KV => Ok(NeutronMsg::register_interchain_query(
            QueryPayload::KV(kv_keys),
            connection_id,
            update_period,
        )),
        QueryType::TX => Ok(NeutronMsg::register_interchain_query(
            QueryPayload::TX(transactions_filter),
            connection_id,
            update_period,
        )),
    }
}

/// Creates a message to register an Interchain Query to get balance of account on remote chain for particular denom
///
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **addr** address of an account on remote chain for which you want to get balances;
/// * **denom** denomination of the coin for which you want to get balance;
/// * **update_period** is used to say how often the query must be updated.
pub fn new_balance_query_msg(
    deps: DepsMut<InterchainQueries>,
    env: Env,
    connection_id: String,
    addr: String,
    denom: String,
    update_period: u64,
) -> NeutronResult<NeutronMsg> {
    let converted_addr_bytes = decode_and_convert(addr.as_str())?;

    let balance_key = create_account_denom_balance_key(converted_addr_bytes, denom)?;

    let kv_key = KVKey {
        path: BANK_STORE_KEY.to_string(),
        key: Binary(balance_key),
    };

    new_interchain_query_msg(
        deps,
        env,
        connection_id,
        QueryType::KV,
        vec![kv_key],
        String::new(),
        update_period,
    )
}

/// Creates a message to register an Interchain Query to get delegations of particular delegator on remote chain.
///
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **delegator** is an address of an account on remote chain for which you want to get list of delegations;
/// * **validators** is a list of validators addresses for which you want to get delegations from particular **delegator**;
/// * **update_period** is used to say how often the query must be updated.
pub fn new_delegator_delegations_query_msg(
    deps: DepsMut<InterchainQueries>,
    env: Env,
    connection_id: String,
    delegator: String,
    validators: Vec<String>,
    update_period: u64,
) -> NeutronResult<NeutronMsg> {
    let delegator_addr = decode_and_convert(delegator.as_str())?;

    // Allocate memory for such KV keys as:
    // * staking module params to get staking denomination
    // * validators structures to calculate amount of delegated tokens
    // * delegations structures to get info about delegations itself
    let mut keys: Vec<KVKey> = Vec::with_capacity(validators.len() * 2 + 1);

    // create KV key to get BondDenom from staking module params
    keys.push(KVKey {
        path: PARAMS_STORE_KEY.to_string(),
        key: Binary(create_params_store_key(STAKING_STORE_KEY, KEY_BOND_DENOM)),
    });

    for v in &validators {
        // create delegation key to get delegation structure
        let val_addr = decode_and_convert(v.as_str())?;
        keys.push(KVKey {
            path: STAKING_STORE_KEY.to_string(),
            key: Binary(create_delegation_key(&delegator_addr, &val_addr)?),
        });

        // create validator key to get validator structure
        keys.push(KVKey {
            path: STAKING_STORE_KEY.to_string(),
            key: Binary(create_validator_key(&val_addr)?),
        })
    }

    new_interchain_query_msg(
        deps,
        env,
        connection_id,
        QueryType::KV,
        keys,
        String::new(),
        update_period,
    )
}

/// Creates a message to register an Interchain Query to get transfer events to a recipient on a remote chain.
///
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **recipient** is an address of an account on remote chain for which you want to get list of transfer transactions;
/// * **update_period** is used to say how often the query must be updated.
/// * **min_height** is used to set min height for query (by default = 0).
pub fn new_transfers_query_msg(
    deps: DepsMut<InterchainQueries>,
    env: Env,
    connection_id: String,
    recipient: String,
    update_period: u64,
    min_height: Option<u128>,
) -> NeutronResult<NeutronMsg> {
    let mut query_data: Vec<TransactionFilterItem> = vec![TransactionFilterItem {
        field: RECIPIENT_FIELD.to_string(),
        op: TransactionFilterOp::Eq,
        value: TransactionFilterValue::String(recipient),
    }];
    if let Some(min_height) = min_height {
        query_data.push(TransactionFilterItem {
            field: HEIGHT_FIELD.to_string(),
            op: TransactionFilterOp::Gte,
            value: TransactionFilterValue::Int(min_height),
        })
    }
    let query_data_json_encoded =
        to_string(&query_data).map_err(|e| StdError::generic_err(e.to_string()))?;

    new_interchain_query_msg(
        deps,
        env,
        connection_id,
        QueryType::TX,
        vec![],
        query_data_json_encoded,
        update_period,
    )
}
