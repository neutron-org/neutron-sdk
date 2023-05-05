use crate::{
    bindings::types::{KVKey, StorageValue},
    errors::error::NeutronResult,
};
use schemars::{JsonSchema, _serde_json::Value};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const QUERY_TYPE_KV_VALUE: &str = "kv";
pub const QUERY_TYPE_TX_VALUE: &str = "tx";

/// Maximum length of address
pub const MAX_ADDR_LEN: usize = 255;

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
    Int(u64),
}

impl<'de> Deserialize<'de> for TransactionFilterValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let v = Value::deserialize(deserializer)?;
        let n = v.as_u64();
        if let Some(n) = n {
            Ok(Self::Int(n))
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
        match self {
            TransactionFilterValue::String(v) => serializer.serialize_str(v),
            TransactionFilterValue::Int(v) => serializer.serialize_u64(*v),
        }
    }
}

pub const MAX_TX_FILTERS: usize = 32;

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionFilterItem {
    pub field: String,
    pub op: TransactionFilterOp,
    pub value: TransactionFilterValue,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// Describes possible interchain query types
pub enum QueryType {
    #[serde(rename = "kv")]
    /// **kv** is an interchain query type to query KV values from remote chain
    KV,

    /// **tx** is an interchain query type to query transactions from remote chain
    #[serde(rename = "tx")]
    TX,
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

/// Describes possible interchain query types with a payload
pub enum QueryPayload {
    /// **kv** is an interchain query type to query KV values from remote chain
    /// payload is kvkeys
    KV(Vec<KVKey>),

    /// **tx** is an interchain query type to query transactions from remote chain
    /// payload is transactions filter
    TX(Vec<TransactionFilterItem>),
}

/// Bytes representations of Bech32 address
pub type AddressBytes = Vec<u8>;

/// A **data structure** that can be reconstructed from slice of **StorageValue** structures.
/// Neutron provides `KVReconstruct` for many primitive and standard Cosmos-SDK types and query responses.
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
    fn reconstruct(kvs: &[StorageValue]) -> NeutronResult<Self>;
}
