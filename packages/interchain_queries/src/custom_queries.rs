use cosmwasm_std::{Binary, CustomQuery};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InterchainQueries {
    InterchainQueryResult {
        query_id: u64,
    },
    InterchainAccountAddress {
        owner_address: String,
        connection_id: String,
    },
    RegisteredInterchainQueries {},
    RegisteredInterchainQuery {
        query_id: u64,
    },
}

impl CustomQuery for InterchainQueries {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredQueriesResponse {
    pub registered_queries: Vec<RegisteredQuery>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredQueryResponse {
    pub registered_query: RegisteredQuery,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RegisteredQuery {
    pub id: u64,
    pub query_data: String,
    pub query_type: String,
    pub zone_id: String,
    pub connection_id: String,
    pub update_period: u64,
    pub last_emitted_height: u64,
    #[serde(default)]
    pub last_submitted_result_local_height: u64,
    #[serde(default)]
    pub last_submitted_result_remote_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct KVKey {
    pub path: String,
    pub key: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredQueryResultResponse {
    pub result: QueryResult,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryResult {
    pub kv_results: Vec<StorageValue>,
    pub height: u64,
    #[serde(default)]
    pub revision: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct StorageValue {
    pub storage_prefix: String,
    pub key: Binary,
    pub value: Binary,
    #[serde(rename = "Proof", default)]
    pub proof: ProofOps,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProofOps {
    pub ops: Vec<ProofOp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProofOp {
    #[serde(rename = "type")]
    pub type_: String,
    pub key: Binary,
    pub data: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryInterchainAccountAddressResponse {
    pub interchain_account_address: String,
}
