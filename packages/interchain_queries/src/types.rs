use crate::error::{ContractError, ContractResult};
use cosmwasm_std::{Coin, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub const DEFAULT_UPDATE_PERIOD: u64 = 10;

const QUERY_TYPE_KV_VALUE: &str = "kv";
const QUERY_TYPE_TX_VALUE: &str = "tx";

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryType {
    #[serde(rename = "kv")]
    KV,

    #[serde(rename = "tx")]
    TX,
}

impl QueryType {
    pub fn try_from_str(s: &str) -> Option<QueryType> {
        match s {
            QUERY_TYPE_KV_VALUE => Some(QueryType::KV),
            QUERY_TYPE_TX_VALUE => Some(QueryType::TX),
            _ => None,
        }
    }
}

impl Into<String> for QueryType {
    fn into(self) -> String {
        match self {
            QueryType::KV => QUERY_TYPE_KV_VALUE.to_string(),
            QueryType::TX => QUERY_TYPE_TX_VALUE.to_string(),
        }
    }
}

pub const QUERY_KV_VALUES: &str = "kv";
pub const QUERY_TRANSACTIONS: &str = "tx";

pub const REGISTER_INTERCHAIN_QUERY_PATH: &str =
    "/neutron.interchainadapter.interchainqueries.MsgRegisterInterchainQuery";

pub const QUERY_REGISTERED_QUERY_RESULT_PATH: &str =
    "/neutron.interchainadapter.interchainqueries.Query/QueryResult";

pub const QUERY_REGISTERED_QUERY_PATH: &str =
    "/neutron.interchainadapter.interchainqueries.Query/RegisteredQuery";

pub const QUERY_REGISTERED_QUERY_TRANSACTIONS_RESULT_PATH: &str =
    "/neutron.interchainadapter.interchainqueries.Query/QueryTransactions";

pub const COSMOS_SDK_TRANSFER_MSG_URL: &str = "/cosmos.bank.v1beta1.MsgSend";

const BALANCES_PREFIX: u8 = 0x02;
pub const DELEGATION_KEY: u8 = 0x31;

const MAX_ADDR_LEN: usize = 255;

pub const BANK_STORE_KEY: &str = "bank";
pub const STAKING_STORE_KEY: &str = "staking";

/// Decodes a bech32 encoded string and converts to base64 encoded bytes
/// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/types/bech32/bech32.go#L20
pub fn decode_and_convert(decoded: &str) -> ContractResult<Vec<u8>> {
    let (_hrp, bytes, _variant) = bech32::decode(decoded)?;

    Ok(bech32::convert_bits(bytes.as_slice(), 5, 8, false)?)
}

/// Prefixes the address bytes with its length
pub fn length_prefix(bz: Vec<u8>) -> ContractResult<Vec<u8>> {
    let bz_length = bz.len();

    if bz_length == 0 {
        return Ok(vec![]);
    }

    if bz_length > MAX_ADDR_LEN {
        return Err(ContractError::MaxAddrLength {
            max: MAX_ADDR_LEN,
            actual: bz_length,
        });
    }

    let mut p: Vec<u8> = vec![bz_length as u8];
    p.extend_from_slice(bz.as_slice());

    Ok(p)
}

// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/x/bank/types/key.go#L55
pub fn create_account_balances_prefix(addr: Vec<u8>) -> ContractResult<Vec<u8>> {
    let mut prefix: Vec<u8> = vec![BALANCES_PREFIX];
    prefix.extend_from_slice(length_prefix(addr)?.as_slice());

    Ok(prefix)
}

// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/x/staking/types/keys.go#L181
pub fn create_delegations_key(delegator_address: Vec<u8>) -> ContractResult<Vec<u8>> {
    let mut key: Vec<u8> = vec![DELEGATION_KEY];
    key.extend_from_slice(length_prefix(delegator_address)?.as_slice());

    Ok(key)
}

// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/x/staking/types/keys.go#L176
pub fn create_delegation_key(
    delegator_address: Vec<u8>,
    validator_address: Vec<u8>,
) -> ContractResult<Vec<u8>> {
    let mut delegations_key: Vec<u8> = create_delegations_key(delegator_address)?;
    delegations_key.extend_from_slice(length_prefix(validator_address)?.as_slice());

    Ok(delegations_key)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub struct GetTransfersParams {
    #[serde(rename = "transfer.recipient")]
    pub recipient: String,

    #[serde(skip_serializing)]
    pub start: u64,

    #[serde(skip_serializing)]
    pub end: u64,
}

pub fn protobuf_coin_to_std_coin(
    coin: cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
) -> ContractResult<Coin> {
    Ok(Coin::new(
        Uint128::from_str(coin.amount.as_str())?.u128(),
        coin.denom,
    ))
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryBalanceResponse {
    pub amount: Coin,
    pub last_submitted_local_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DelegatorDelegationsResponse {
    pub delegations: Vec<cosmwasm_std::Delegation>,
    pub last_submitted_local_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Transfer {
    pub tx_id: u64,
    pub sender: String,
    pub recipient: String,
    pub amount: Vec<Coin>,
    pub height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TransfersResponse {
    pub transfers: Vec<Transfer>,
    pub last_submitted_local_height: u64,
}
