// import all queries from v045 package
// to it available from v047 package (kinda proxy) since they work with Cosmos SDK 0.47 as usual
pub use crate::interchain_queries::v045::queries::*;

use crate::{
    bindings::query::NeutronQuery,
    interchain_queries::{
        queries::{check_query_type, get_registered_query, query_kv_result},
        types::QueryType,
        v047::types::Balances,
    },
    NeutronResult,
};
use cosmwasm_std::{Deps, Env};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// But at the same time we replace v045::BalancesResponse and v045::query_balance() with
// corresponding structure and method below to create a structure and a method
// compatible with Cosmos SDK 0.47

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BalanceResponse {
    pub balances: Balances,
    pub last_submitted_local_height: u64,
}

/// Returns balance of account on remote chain for particular denom
/// * ***registered_query_id*** is an identifier of the corresponding registered interchain query
pub fn query_balance(
    deps: Deps<NeutronQuery>,
    _env: Env,
    registered_query_id: u64,
) -> NeutronResult<BalanceResponse> {
    let registered_query = get_registered_query(deps, registered_query_id)?;

    check_query_type(registered_query.registered_query.query_type, QueryType::KV)?;

    let balances: Balances = query_kv_result(deps, registered_query_id)?;

    Ok(BalanceResponse {
        last_submitted_local_height: registered_query
            .registered_query
            .last_submitted_result_local_height,
        balances,
    })
}
