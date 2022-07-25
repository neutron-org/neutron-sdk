use crate::error::ContractResult;
use crate::types::{
    create_account_balances_prefix, create_delegation_key, decode_and_convert, GetTransfersParams,
    QueryType, BANK_STORE_KEY, REGISTER_INTERCHAIN_QUERY_PATH, STAKING_STORE_KEY,
};
use cosmwasm_std::{Binary, CosmosMsg, DepsMut, Env, Response, StdError};
use protobuf::Message;
use schemars::_serde_json::to_string;
use stargate::interchain::interchainqueries_genesis::KVKey;
use stargate::interchain::interchainqueries_tx::MsgRegisterInterchainQuery;
use std::fmt::Write;

pub fn encode_hex(bytes: &[u8]) -> ContractResult<String> {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b)?;
    }
    Ok(s)
}

/// Registers an interchain query
fn register_interchain_query(
    _deps: DepsMut,
    env: Env,
    connection_id: String,
    zone_id: String,
    query_type: QueryType,
    kv_keys: Vec<KVKey>,
    transactions_filter: String,
    update_period: u64,
) -> ContractResult<Response> {
    let mut register_msg = MsgRegisterInterchainQuery::new();
    register_msg.keys = kv_keys.clone();
    register_msg.transactions_filter = transactions_filter.clone();
    register_msg.query_type = query_type.into();
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

    Ok(Response::new()
        .add_attribute("action", "register_interchain_query")
        .add_attribute("connection_id", connection_id.as_str())
        .add_attribute("zone_id", zone_id.as_str())
        .add_attribute("query_type", query_type)
        .add_attribute("update_period", update_period.to_string())
        .add_attribute("transactions_filter", transactions_filter.as_str())
        .add_attribute(
            "kv_keys",
            kv_keys
                .iter()
                .map(|kv| Ok(kv.path.clone() + "/" + encode_hex(&kv.key)?.as_str()))
                .collect::<ContractResult<Vec<String>>>()?
                .join(","),
        )
        .add_message(msg))
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
) -> ContractResult<Response> {
    let converted_addr_bytes = decode_and_convert(addr.as_str())?;

    let mut balance_key = create_account_balances_prefix(converted_addr_bytes)?;
    balance_key.extend_from_slice(denom.as_bytes());

    let mut kv_key = KVKey::new();
    kv_key.key = balance_key;
    kv_key.path = BANK_STORE_KEY.to_string();

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
    deps: DepsMut,
    env: Env,
    connection_id: String,
    zone_id: String,
    delegator: String,
    validators: Vec<String>,
    update_period: u64,
) -> ContractResult<Response> {
    let delegator_addr = decode_and_convert(delegator.as_str())?;

    let keys = validators
        .into_iter()
        .map(|v| {
            let val_addr = decode_and_convert(v.as_str())?;
            Ok(KVKey {
                path: STAKING_STORE_KEY.to_string(),
                key: create_delegation_key(delegator_addr.clone(), val_addr)?,
                special_fields: Default::default(),
            })
        })
        .collect::<ContractResult<Vec<KVKey>>>()?;

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
    deps: DepsMut,
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
