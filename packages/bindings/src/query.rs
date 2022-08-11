use crate::types::{InterchainQueryResult, RegisteredQuery};
use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// The queries to interact with neutron specific blockchain modules.
pub enum InterchainQueries {
    /// Query a result of registered interchain query on remote chain
    InterchainQueryResult {
        /// **query_id** is an ID registered interchain query
        query_id: u64,
    },

    /// Query a registered interchain account address for a specific connection_id
    /// Every contract may have as many interchain accounts as necessary.
    InterchainAccountAddress {
        /// **owner_address** is an address of contract which registered interchain account
        owner_address: String,

        /// **interchain_account_id** is an identifier of your interchain account. Can be any string
        /// This identifier allows contracts to have multiple interchain accounts on remote chains
        interchain_account_id: String,

        /// **connection_id** is an IBC connection identifier between Neutron and remote chain
        connection_id: String,
    },

    /// Query all registered interchain queries on all remote chains
    RegisteredInterchainQueries {},

    /// Query registered interchain query with a specific queiry_id
    RegisteredInterchainQuery {
        /// **query_id** is an ID registered interchain query
        query_id: u64,
    },
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
    pub result: InterchainQueryResult,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryInterchainAccountAddressResponse {
    /// **interchain_account_address** is a interchain account address on the remote chain
    pub interchain_account_address: String,
}

impl CustomQuery for InterchainQueries {}
