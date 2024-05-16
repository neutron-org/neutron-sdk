use crate::bindings::marketmap::query::MarketMapQuery;
use crate::bindings::oracle::query::OracleQuery;
use crate::bindings::types::{Failure, InterchainQueryResult, RegisteredQuery};
use cosmwasm_std::{Binary, CustomQuery, QueryRequest};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::dex::query::DexQuery;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// The queries to interact with neutron specific blockchain modules.
pub enum NeutronQuery {
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
    RegisteredInterchainQueries {
        owners: Vec<String>,
        connection_id: String,
        pagination: PageRequest,
    },

    /// Query registered interchain query with a specific query_id
    RegisteredInterchainQuery {
        /// **query_id** is an ID registered interchain query
        query_id: u64,
    },

    /// Query total amount of burned neutron fees
    TotalBurnedNeutronsAmount {},

    /// Query minimum IBC fee
    MinIbcFee {},

    /// TokenFactory query. Given a subdenom minted by a contract via
    /// [`NeutronMsg::MintTokens`](crate::bindings::msg::NeutronMsg::MintTokens),
    /// returns the full denom as used by [`BankMsg::Send`](cosmwasm_std::BankMsg::Send).
    FullDenom {
        creator_addr: String,
        subdenom: String,
    },

    /// TokenFactory query. Returns the admin of a denom, if the denom is a TokenFactory denom.
    DenomAdmin {
        subdenom: String,
    },

    /// TokenFactory query. Returns the before send hook address of a denom, if the denom is a TokenFactory denom.
    BeforeSendHook {
        denom: String,
    },

    /// Contractmanager query. Returns the failures for a particular contract address.
    Failures {
        address: String,
        pagination: PageRequest,
    },

    Dex(DexQuery),

    MarketMap(MarketMapQuery),

    Oracle(OracleQuery),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageRequest {
    /// **key** is a value returned in PageResponse.next_key to begin
    /// querying the next page most efficiently. Only one of offset or key
    /// should be set.
    pub key: Binary,
    /// **offset** is a numeric offset that can be used when key is unavailable.
    /// It is less efficient than using key. Only one of offset or key should
    /// be set.
    pub offset: u64,
    /// **limit** is the total number of results to be returned in the result page.
    /// If left empty it will default to a value to be set by each app.
    pub limit: u64,
    /// **count_total** is set to true  to indicate that the result set should include
    /// a count of the total number of items available for pagination in UIs.
    /// count_total is only respected when offset is used. It is ignored when key
    /// is set.
    pub count_total: bool,
    /// reverse is set to true if results are to be returned in the descending order.
    pub reverse: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageResponse {
    /// **next_key** is the key to be passed to PageRequest.key to
    /// query the next page most efficiently. It will be empty if
    /// there are no more results.
    pub next_key: Option<Binary>,
    /// **total** is total number of results available if PageRequest.count_total
    /// was set, its value is undefined otherwise
    pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredQueriesResponse {
    /// **registered_queries** is a list of registered queries
    pub registered_queries: Vec<RegisteredQuery>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredQueryResponse {
    /// **registered_query** is a registered query
    pub registered_query: RegisteredQuery,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredQueryResultResponse {
    pub result: InterchainQueryResult,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryInterchainAccountAddressResponse {
    /// **interchain_account_address** is a interchain account address on the remote chain
    pub interchain_account_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryFailuresResponse {
    /// **failures** is a list of failures of sudo handler calls
    pub failures: Vec<Failure>,
}

impl CustomQuery for NeutronQuery {}

impl From<DexQuery> for QueryRequest<NeutronQuery> {
    fn from(msg: DexQuery) -> Self {
        QueryRequest::Custom(NeutronQuery::Dex(msg))
    }
}

impl From<MarketMapQuery> for QueryRequest<NeutronQuery> {
    fn from(msg: MarketMapQuery) -> Self {
        QueryRequest::Custom(NeutronQuery::MarketMap(msg))
    }
}

impl From<OracleQuery> for QueryRequest<NeutronQuery> {
    fn from(msg: OracleQuery) -> Self {
        QueryRequest::Custom(NeutronQuery::Oracle(msg))
    }
}
