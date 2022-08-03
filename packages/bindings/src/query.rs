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

impl CustomQuery for InterchainQueries {}
