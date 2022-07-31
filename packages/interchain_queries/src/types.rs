use crate::error::{ContractError, ContractResult};
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin as CosmosCoin;
use cosmos_sdk_proto::cosmos::staking::v1beta1::{Delegation, Validator};
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use prost::Message as ProstMessage;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use stargate::interchain::interchainqueries_tx::StorageValue;
use std::io::Cursor;
use std::ops::Mul;
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
pub const VALIDATORS_KEY: u8 = 0x21;

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
pub fn length_prefix(bz: &[u8]) -> ContractResult<Vec<u8>> {
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
    p.extend_from_slice(bz);

    Ok(p)
}

// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/x/bank/types/key.go#L55
pub fn create_account_balances_prefix(addr: &[u8]) -> ContractResult<Vec<u8>> {
    let mut prefix: Vec<u8> = vec![BALANCES_PREFIX];
    prefix.extend_from_slice(length_prefix(addr)?.as_slice());

    Ok(prefix)
}

// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/x/staking/types/keys.go#L181
pub fn create_delegations_key(delegator_address: &[u8]) -> ContractResult<Vec<u8>> {
    let mut key: Vec<u8> = vec![DELEGATION_KEY];
    key.extend_from_slice(length_prefix(delegator_address)?.as_slice());

    Ok(key)
}

// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/x/staking/types/keys.go#L176
pub fn create_delegation_key(
    delegator_address: &[u8],
    validator_address: &[u8],
) -> ContractResult<Vec<u8>> {
    let mut delegations_key: Vec<u8> = create_delegations_key(delegator_address)?;
    delegations_key.extend_from_slice(length_prefix(validator_address)?.as_slice());

    Ok(delegations_key)
}

// https://github.com/cosmos/cosmos-sdk/blob/f2d94445c0f5f52cf5ed999b81048b575de94964/x/staking/types/keys.go#L55
pub fn create_validator_key(operator_address: &[u8]) -> ContractResult<Vec<u8>> {
    let mut key: Vec<u8> = vec![VALIDATORS_KEY];
    key.extend_from_slice(length_prefix(operator_address)?.as_slice());

    Ok(key)
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

pub struct KVResult(Vec<StorageValue>);

impl KVResult {
    pub fn new(kvs: Vec<StorageValue>) -> Self {
        KVResult(kvs)
    }
}

pub trait KVReconstruct: Sized {
    fn reconstruct(kvs: &KVResult) -> ContractResult<Self>;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Balances {
    pub coins: Vec<Coin>,
}

impl KVReconstruct for Balances {
    fn reconstruct(storage_values: &KVResult) -> ContractResult<Balances> {
        let mut coins: Vec<Coin> = vec![];

        for kv in &storage_values.0 {
            let balance: CosmosCoin = CosmosCoin::decode(Cursor::new(kv.value.clone()))?;
            let amount = Uint128::from_str(balance.amount.as_str())?;
            coins.push(Coin::new(amount.u128(), balance.denom));
        }

        Ok(Balances { coins })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Delegations {
    pub delegations: Vec<cosmwasm_std::Delegation>,
}

impl KVReconstruct for Delegations {
    fn reconstruct(storage_values: &KVResult) -> ContractResult<Delegations> {
        let mut delegations: Vec<cosmwasm_std::Delegation> = vec![];

        for chunk in storage_values.0.chunks(2) {
            let delegation_sdk: Delegation = Delegation::decode(Cursor::new(&chunk[0].value))?;
            let mut delegation_std = cosmwasm_std::Delegation {
                delegator: Addr::unchecked(delegation_sdk.delegator_address.as_str()),
                validator: delegation_sdk.validator_address,
                amount: Default::default(),
            };
            let share = Decimal::from_ratio(
                Uint128::from_str(&delegation_sdk.shares)?,
                Uint128::new(1_000_000_000_000_000_000u128),
            );

            let validator: Validator = Validator::decode(Cursor::new(&chunk[1].value.clone()))?;
            let m = share.mul(Uint128::from_str(&validator.tokens)?);
            let tokens = m.multiply_ratio(
                1_000_000_000_000_000_000u128,
                Uint128::from_str(&validator.delegator_shares)?,
            );
            delegation_std.amount = Coin::new(tokens.u128(), "kek".to_string());

            delegations.push(delegation_std);
        }

        Ok(Delegations { delegations })
    }
}
