use crate::errors::error::NeutronResult;
use crate::interchain_queries::types::{KVReconstruct, QueryType};
use crate::NeutronError;
use cosmwasm_std::{Deps, StdError};
use neutron_std::types::neutron::interchainqueries::{
    InterchainqueriesQuerier, QueryResult, RegisteredQuery,
};

/// Checks **actual** query type is **expected** query type
pub fn check_query_type(actual: String, expected: QueryType) -> NeutronResult<()> {
    let expected_str: String = expected.into();
    if actual != expected_str {
        return Err(NeutronError::InvalidQueryType { query_type: actual });
    }
    Ok(())
}

/// Queries registered query info
pub fn get_registered_query(
    deps: Deps,
    interchain_query_id: u64,
) -> NeutronResult<RegisteredQuery> {
    let querier = InterchainqueriesQuerier::new(&deps.querier);
    let query_res = querier.registered_query(interchain_query_id)?;
    let res = query_res
        .registered_query
        .ok_or_else(|| StdError::generic_err("no registered query"))?;
    Ok(res)
}

/// Reads submitted raw KV values for Interchain Query with **query_id** from the storage and reconstructs the result
pub fn query_kv_result<T: KVReconstruct>(deps: Deps, query_id: u64) -> NeutronResult<T> {
    let registered_query_result = get_raw_interchain_query_result(deps, query_id)?;
    KVReconstruct::reconstruct(registered_query_result.kv_results.as_slice())
}

/// Queries raw interchain query result (raw KV storage values or transactions) from Interchain Queries Module.
/// Usually it is better to implement [KVReconstruct] for your own type and then use [query_kv_result],
/// but in cases when Rust forbids to implement foreign trait [KVReconstruct] for some foreign type,
/// it is possible to use [get_raw_interchain_query_result] and reconstruct query result manually.
pub fn get_raw_interchain_query_result(
    deps: Deps,
    interchain_query_id: u64,
) -> NeutronResult<QueryResult> {
    let querier = InterchainqueriesQuerier::new(&deps.querier);
    let query_res = querier.query_result(interchain_query_id)?;
    let res = query_res
        .result
        .ok_or_else(|| StdError::generic_err("no result in registered query"))?;

    Ok(res)
}
