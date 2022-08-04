use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Binary;

/// encode_hex encodes bytes slice into hex string
pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// decode_hex decodes hex string into bytes vec
pub fn decode_hex(s: &str) -> Option<Vec<u8>> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredQueriesResponse {
    /// **registered_queries** is a list of registered queries
    pub registered_queries: Vec<RegisteredQuery>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredQueryResponse {
    /// **registered_query** is a registered query
    pub registered_query: RegisteredQuery,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredQueryResultResponse {
    pub result: QueryResult,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryInterchainAccountAddressResponse {
    /// **interchain_account_address** is a interchain account address on the remote chain
    pub interchain_account_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RegisteredQuery {
    /// The unique id of the registered query.
    pub id: u64,
    /// The address that registered the query.
    pub owner: String,
    /// The KV-storage keys for which we want to get values from remote chain
    pub keys: Vec<KVKey>,
    /// The query type identifier (i.e. 'kv' or 'tx' for now)
    pub query_type: String,
    /// The filter for transaction search ICQ
    pub transactions_filter: String,
    /// The chain of interest identifier.
    pub zone_id: String,
    /// The IBC connection ID for getting ConsensusState to verify proofs.
    pub connection_id: String,
    /// Parameter that defines how often the query must be updated.
    pub update_period: u64,
    /// The local height when the event to update the query result was emitted last time.
    pub last_emitted_height: u64,
    /// The local chain last block height when the query result was updated.
    #[serde(default)]
    pub last_submitted_result_local_height: u64,
    /// The remote chain last block height when the query result was updated.
    #[serde(default)]
    pub last_submitted_result_remote_height: u64,
}

/// QueryResult is a result data for a registered query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryResult {
    /// **kv_results** is a raw key-value pairs of query result
    pub kv_results: Vec<StorageValue>,

    /// **height** is a height of remote chain
    pub height: u64,

    #[serde(default)]
    /// **revision** is a revision of remote chain
    pub revision: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct StorageValue {
    pub storage_prefix: String,
    pub key: Binary,
    pub value: Binary,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
/// Type for wrapping any protobuf message
pub struct ProtobufAny {
    /// **type_url** describes the type of the serialized message
    pub type_url: String,

    ///  **value** must be a valid serialized protocol buffer of the above specified type
    pub value: Vec<u8>,
}

impl ProtobufAny {
    /// Helper to create new ProtobufAny type:
    /// * **type_url** describes the type of the serialized message
    /// * **value** must be a valid serialized protocol buffer of the above specified type
    pub fn new(type_url: String, value: Vec<u8>) -> Self {
        ProtobufAny { type_url, value }
    }
}

const KV_PATH_KEY_DELIMITER: &str = "/";
const KV_KEYS_DELIMITER: &str = ";";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct KVKey {
    pub path: String,
    pub key: Vec<u8>,
}

impl KVKey {
    pub fn from_string<S: AsRef<str>>(s: S) -> Option<KVKey> {
        let split: Vec<&str> = s.as_ref().split(KV_PATH_KEY_DELIMITER).collect();
        if split.len() < 2 {
            return None;
        }

        Some(KVKey {
            path: split[0].to_string(),
            key: decode_hex(split[1])?,
        })
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for &KVKey {
    fn into(self) -> String {
        let mut s = String::with_capacity(
            self.path.len() + KV_PATH_KEY_DELIMITER.len() + self.key.len() * 2,
        );

        s.push_str(&self.path);
        s.push_str(KV_PATH_KEY_DELIMITER);
        s.push_str(&encode_hex(&self.key));

        s
    }
}

/// KVKeys describes vec of KVKey structures
pub struct KVKeys(pub Vec<KVKey>);

impl KVKeys {
    pub fn from_string<S: AsRef<str>>(s: S) -> Option<KVKeys> {
        let split = s.as_ref().split(KV_KEYS_DELIMITER);

        Some(KVKeys(
            split
                .map(KVKey::from_string)
                .collect::<Option<Vec<KVKey>>>()?,
        ))
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for KVKeys {
    fn into(self) -> String {
        self.0
            .iter()
            .map(|kv| kv.into())
            .collect::<Vec<String>>()
            .join(KV_KEYS_DELIMITER)
    }
}
