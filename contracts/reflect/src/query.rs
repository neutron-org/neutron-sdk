use cosmwasm_std::{Binary, CustomQuery, QueryRequest};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Reflect(QueryRequest<InterchainQueries>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InterchainQueries {
    InterchainQueryResult {
        query_id: u64,
    },
    InterchainAccountAddress {
        owner_address: String,
        interchain_account_id: String,
        connection_id: String,
    },
    RegisteredInterchainQueries {},
    RegisteredInterchainQuery {
        query_id: u64,
    },
}

impl CustomQuery for InterchainQueries {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChainResponse {
    pub data: Binary,
}
