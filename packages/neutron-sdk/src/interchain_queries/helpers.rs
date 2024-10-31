use crate::errors::error::{NeutronError, NeutronResult};
use crate::interchain_queries::types::{
    AddressBytes, QueryPayload, QueryType, TransactionFilterItem, MAX_ADDR_LEN,
};
use cosmwasm_std::{Addr, CosmosMsg, StdError, Uint128, Uint256};
use neutron_std::types::neutron::interchainqueries::{
    KvKey, MsgRegisterInterchainQuery, MsgRemoveInterchainQueryRequest,
    MsgUpdateInterchainQueryRequest,
};
use serde_json_wasm::to_string;
use std::fmt::Write as _;

/// Decodes a bech32 encoded string and converts to base64 encoded bytes
/// <https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/types/bech32/bech32.go#L20>
pub fn decode_and_convert(encoded: &str) -> NeutronResult<AddressBytes> {
    let (_hrp, bytes, _variant) = bech32::decode(encoded)?;

    Ok(bech32::convert_bits(&bytes, 5, 8, false)?)
}

/// Prefixes the address bytes with its length
pub fn length_prefix<AddrBytes: AsRef<[u8]>>(addr: AddrBytes) -> NeutronResult<Vec<u8>> {
    let bz_length = addr.as_ref().len();

    if bz_length == 0 {
        return Ok(vec![]);
    }

    if bz_length > MAX_ADDR_LEN {
        return Err(NeutronError::MaxAddrLength {
            max: MAX_ADDR_LEN,
            actual: bz_length,
        });
    }

    let mut p: Vec<u8> = vec![bz_length as u8];
    p.extend_from_slice(addr.as_ref());

    Ok(p)
}

pub fn uint256_to_u128(value: Uint256) -> Result<u128, StdError> {
    let converted: Uint128 = value
        .try_into()
        .map_err(|_| StdError::generic_err("Uint256 value exceeds u128 limits"))?;
    Ok(converted.u128())
}

/// Basic helper to define a register interchain query message:
/// * **contract** is a contract address that registers the interchain query.
///   Must be equal to the contract that sends the message.
/// * **query** is a query type identifier ('tx' or 'kv' for now) with a payload:
///   - when the query enum is 'kv' then payload is the KV-storage keys for which we want to get
///     values from remote chain;
///   - when the query enum is 'tx' then payload is the filters for transaction search ICQ,
///     maximum allowed number of filters is 32.
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **update_period** is used to say how often (in neutron blocks) the query must be updated.
pub fn register_interchain_query(
    contract: Addr,
    query: QueryPayload,
    connection_id: String,
    update_period: u64,
) -> NeutronResult<CosmosMsg> {
    Ok(match query {
        QueryPayload::KV(keys) => MsgRegisterInterchainQuery {
            sender: contract.to_string(),
            query_type: QueryType::KV.into(),
            keys,
            transactions_filter: String::new(),
            connection_id,
            update_period,
        },
        QueryPayload::TX(transactions_filters) => MsgRegisterInterchainQuery {
            sender: contract.to_string(),
            query_type: QueryType::TX.into(),
            keys: vec![],
            transactions_filter: to_string(&transactions_filters)
                .map_err(|e| StdError::generic_err(e.to_string()))?,
            connection_id,
            update_period,
        },
    }
    .into())
}

/// Basic helper to define a update interchain query message:
/// * **contract** is a contract address that updates the interchain query.
///   Must be equal to the contract that sends the message.
/// * **query_id** is ID of the query we want to update;
/// * **new_keys** is encoded keys to query;
/// * **new_update_period** is used to say how often (in neutron blocks) the query must be updated.
pub fn update_interchain_query(
    contract: Addr,
    query_id: u64,
    new_keys: Vec<KvKey>,
    new_update_period: u64,
    new_transactions_filter: Option<Vec<TransactionFilterItem>>,
) -> NeutronResult<CosmosMsg> {
    Ok(MsgUpdateInterchainQueryRequest {
        sender: contract.to_string(),
        query_id,
        new_keys,
        new_update_period,
        new_transactions_filter: match new_transactions_filter {
            Some(filters) => {
                to_string(&filters).map_err(|e| StdError::generic_err(e.to_string()))?
            }
            None => "".to_string(),
        },
    }
    .into())
}

/// Basic helper to define a remove interchain query message:
/// * **contract** is a contract address that removes the interchain query.
///   Must be equal to the contract that sends the message.
/// * **query_id** is ID of the query we want to remove.
pub fn remove_interchain_query(contract: Addr, query_id: u64) -> NeutronResult<CosmosMsg> {
    Ok(MsgRemoveInterchainQueryRequest {
        sender: contract.to_string(),
        query_id,
    }
    .into())
}

const KV_PATH_KEY_DELIMITER: &str = "/";

pub fn kv_key_from_string<S: AsRef<str>>(s: S) -> Option<KvKey> {
    let split: Vec<&str> = s.as_ref().split(KV_PATH_KEY_DELIMITER).collect();
    if split.len() < 2 {
        return None;
    }

    Some(KvKey {
        path: split[0].to_string(),
        key: decode_hex(split[1])?,
    })
}

/// Encodes bytes slice into hex string
pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        let _ = write!(s, "{:02x}", b);
    }
    s
}

/// Decodes hex string into bytes vec
pub fn decode_hex(s: &str) -> Option<Vec<u8>> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}
