use crate::error::{ContractError, ContractResult};
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin as CosmosCoin;
use cosmos_sdk_proto::cosmos::staking::v1beta1::{Delegation, Validator};
use cosmwasm_std::{from_binary, Addr, Coin, Decimal, Uint128};
use neutron_bindings::types::StorageValue;
use prost::Message as ProstMessage;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::ops::Div;
use std::str::FromStr;

const DECIMAL_PLACES: u32 = 18;
const DECIMAL_FRACTIONAL: u128 = 10u128.pow(DECIMAL_PLACES);

const QUERY_TYPE_KV_VALUE: &str = "kv";
const QUERY_TYPE_TX_VALUE: &str = "tx";

/// Protobuf type url of standard Cosmos SDK bank transfer message
pub const COSMOS_SDK_TRANSFER_MSG_URL: &str = "/cosmos.bank.v1beta1.MsgSend";

/// Storage prefix for account balances store
/// https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/bank/types/key.go#L27
pub const BALANCES_PREFIX: u8 = 0x02;

/// Key for delegations in the **staking** module's storage
/// https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/staking/types/keys.go#L39
pub const DELEGATION_KEY: u8 = 0x31;

/// Key for validators in the **staking** module's storage
/// https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/staking/types/keys.go#L35
pub const VALIDATORS_KEY: u8 = 0x21;

/// Maximum length of address
pub const MAX_ADDR_LEN: usize = 255;

/// Name of the standard **bank** Cosmos-SDK module
pub const BANK_STORE_KEY: &str = "bank";

/// Name of the standard **staking** Cosmos-SDK module
pub const STAKING_STORE_KEY: &str = "staking";

/// Key for bond denomination param of Cosmos-SDK staking module
/// https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/staking/types/params.go#L39
pub const KEY_BOND_DENOM: &str = "BondDenom";

/// Name of the standard **params** Cosmos-SDK module
pub const PARAMS_STORE_KEY: &str = "params";

/// Default delimiter of **params** Cosmos-SDK module
pub const PARAMS_STORE_DELIMITER: &str = "/";

pub const RECIPIENT_FIELD: &str = "transfer.recipient";
pub const HEIGHT_FIELD: &str = "tx.height";

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum TransactionFilterOp {
    Eq,
    Lt,
    Gt,
    Lte,
    Gte,
}

#[derive(PartialEq, Eq, Debug)]
pub enum TransactionFilterValue {
    String(String),
    Int(u128),
}

impl<'de> Deserialize<'de> for TransactionFilterValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use serde_json::Value;

        let v = Value::deserialize(deserializer)?;
        let n = v.as_u64();
        if let Some(n) = n {
            Ok(Self::Int(n.into()))
        } else {
            let n = v
                .as_str()
                .ok_or_else(|| D::Error::custom("Value must be number or string"))?;
            Ok(Self::String(n.to_string()))
        }
    }
}

impl Serialize for TransactionFilterValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &*self {
            TransactionFilterValue::String(v) => serializer.serialize_str(v),
            TransactionFilterValue::Int(v) => serializer.serialize_u128(*v),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionFilterItem {
    pub field: String,
    pub op: TransactionFilterOp,
    pub value: TransactionFilterValue,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, JsonSchema)]
/// Describes possible interchain query types
pub enum QueryType {
    #[serde(rename = "kv")]
    /// **kv** is an interchain query type to query KV values from remote chain
    KV,

    /// **tx** is an interchain query type to query transactions from remote chain
    #[serde(rename = "tx")]
    TX,
}

impl QueryType {
    /// Tries to parse query type from string
    /// Returns **None** if string is invalid query type
    pub fn try_from_str(s: &str) -> Option<QueryType> {
        match s {
            QUERY_TYPE_KV_VALUE => Some(QueryType::KV),
            QUERY_TYPE_TX_VALUE => Some(QueryType::TX),
            _ => None,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for QueryType {
    fn into(self) -> String {
        match self {
            QueryType::KV => QUERY_TYPE_KV_VALUE.to_string(),
            QueryType::TX => QUERY_TYPE_TX_VALUE.to_string(),
        }
    }
}

/// Bytes representations of Bech32 address
pub type AddressBytes = Vec<u8>;

/// A **data structure** that can be reconstructed from slice of **StorageValue** structures.
/// Neutron provides `KVReconstruct` for many primitive and standard Cosmos-SDK types and query responses.
/// The complete list is [here][TODO_LINK]. All of these can be deserialized using Neutron out of the box.
///
/// Third-party projects may provide `KVReconstruct` implementations for types that they introduce.
/// For example if some query is not implemented in Neutron standard library, developers can create their own type/query and implement `KVReconstruct` for it.
///
///
/// Usually used together with `query_kv_result` function. For example, there is an Interchain Query with some `query_id` to query balance from remote chain.
/// And there is an implemented `KVReconstruct` for `Balance` structure.
/// So you can easily get reconstructed response for the query just in one line:
/// ```rust ignore
/// let balances: Balances = query_kv_result(deps, query_id)?;
/// ```
///
/// Anyone can implement `KVReconstruct` for any type and use `query_kv_result` without any problems.
pub trait KVReconstruct: Sized {
    /// Reconstructs this value from the slice of **StorageValue**'s.
    fn reconstruct(kvs: &[StorageValue]) -> ContractResult<Self>;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
/// A structure that can be reconstructed from **StorageValues**'s for the **Balance Interchain Query**.
/// Contains coins that are held by some account on remote chain.
pub struct Balances {
    pub coins: Vec<Coin>,
}

impl KVReconstruct for Balances {
    fn reconstruct(storage_values: &[StorageValue]) -> ContractResult<Balances> {
        let mut coins: Vec<Coin> = vec![];

        for kv in storage_values {
            let balance: CosmosCoin = CosmosCoin::decode(kv.value.as_slice())?;
            let amount = Uint128::from_str(balance.amount.as_str())?;
            coins.push(Coin::new(amount.u128(), balance.denom));
        }

        Ok(Balances { coins })
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
/// A structure that can be reconstructed from **StorageValues**'s for the **Delegator Delegation Interchain Query**.
/// Contains delegations which some delegator has on remote chain.
pub struct Delegations {
    pub delegations: Vec<cosmwasm_std::Delegation>,
}

impl KVReconstruct for Delegations {
    fn reconstruct(storage_values: &[StorageValue]) -> ContractResult<Delegations> {
        let mut delegations: Vec<cosmwasm_std::Delegation> = vec![];

        // first StorageValue is denom
        if storage_values[0].value.is_empty() {
            // Incoming denom cannot be empty, it should always be configured on chain.
            // If we receive empty denom, that means incoming data structure is corrupted
            // and we cannot build `cosmwasm_std::Delegation`'s using this data.
            return Err(ContractError::InvalidQueryResultFormat(
                "denom is empty".into(),
            ));
        }
        let denom: String = from_binary(&storage_values[0].value)?;

        // the rest are delegations and validators alternately
        for chunk in storage_values[1..].chunks(2) {
            if chunk[0].value.is_empty() {
                // Incoming delegation can actually be empty, this just means that delegation
                // is not present on remote chain, which is to be expected. So, if it doesn't
                // exist, we can safely skip this and following chunk.
                continue;
            }
            let delegation_sdk: Delegation = Delegation::decode(chunk[0].value.as_slice())?;

            let mut delegation_std = cosmwasm_std::Delegation {
                delegator: Addr::unchecked(delegation_sdk.delegator_address.as_str()),
                validator: delegation_sdk.validator_address,
                amount: Default::default(),
            };

            if chunk[1].value.is_empty() {
                // At this point, incoming validator cannot be empty, that would be invalid,
                // because delegation is already defined, so, building `cosmwasm_std::Delegation`
                // from this data is impossible, incoming data is corrupted.post
                return Err(ContractError::InvalidQueryResultFormat(
                    "validator is empty".into(),
                ));
            }
            let validator: Validator = Validator::decode(chunk[1].value.as_slice())?;

            let delegation_shares =
                Decimal::from_atomics(Uint128::from_str(&delegation_sdk.shares)?, DECIMAL_PLACES)?;

            let delegator_shares = Decimal::from_atomics(
                Uint128::from_str(&validator.delegator_shares)?,
                DECIMAL_PLACES,
            )?;

            let validator_tokens = Decimal::from_atomics(Uint128::from_str(&validator.tokens)?, 0)?;

            // https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/staking/keeper/querier.go#L463
            // delegated_tokens = quotient(delegation.shares * validator.tokens / validator.total_shares);
            let delegated_tokens = delegation_shares
                .checked_mul(validator_tokens)?
                .div(delegator_shares)
                .atomics()
                .u128()
                .div(DECIMAL_FRACTIONAL);

            delegation_std.amount = Coin::new(delegated_tokens, &denom);

            delegations.push(delegation_std);
        }

        Ok(Delegations { delegations })
    }
}
