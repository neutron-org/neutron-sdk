use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Binary;

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
    /// **interchain_account_address** is an interchain account address on the remote chain
    pub interchain_account_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RegisteredQuery {
    /// The unique id of the registered query.
    pub id: u64,
    /// The JSON encoded data of the query.
    pub query_data: String,
    /// The query type identifier (i.e. /cosmos.staking.v1beta1.Query/AllDelegations).
    pub query_type: String,
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
