use crate::bindings::query::{
    NeutronQuery, QueryRegisteredQueryResponse, QueryRegisteredQueryResultResponse,
};
use crate::errors::error::NeutronResult;
use crate::interchain_queries::types::{KVReconstruct, QueryType};
use crate::NeutronError;
use cosmwasm_std::Deps;

/// Checks **actual** query type is **expected** query type
pub fn check_query_type(actual: QueryType, expected: QueryType) -> NeutronResult<()> {
    if actual != expected {
        return Err(NeutronError::InvalidQueryType {
            query_type: actual.into(),
        });
    }
    Ok(())
}

/// Queries registered query info
pub fn get_registered_query(
    deps: Deps<NeutronQuery>,
    interchain_query_id: u64,
) -> NeutronResult<QueryRegisteredQueryResponse> {
    let query = NeutronQuery::RegisteredInterchainQuery {
        query_id: interchain_query_id,
    };

    let res: QueryRegisteredQueryResponse = deps.querier.query(&query.into())?;
    Ok(res)
}

/// Reads submitted raw KV values for Interchain Query with **query_id** from the storage and reconstructs the result
pub fn query_kv_result<T: KVReconstruct>(
    deps: Deps<NeutronQuery>,
    query_id: u64,
) -> NeutronResult<T> {
    let registered_query_result = get_raw_interchain_query_result(deps, query_id)?;

    KVReconstruct::reconstruct(&registered_query_result.result.kv_results)
}

/// Queries raw interchain query result (raw KV storage values or transactions) from Interchain Queries Module.
/// Usually it is better to implement [KVReconstruct] for your own type and then use [query_kv_result],
/// but in cases when Rust forbids to implement foreign trait [KVReconstruct] for some foreign type,
/// it is possible to use [get_raw_interchain_query_result] and reconstruct query result manually.
pub fn get_raw_interchain_query_result(
    deps: Deps<NeutronQuery>,
    interchain_query_id: u64,
) -> NeutronResult<QueryRegisteredQueryResultResponse> {
    let interchain_query = NeutronQuery::InterchainQueryResult {
        query_id: interchain_query_id,
    };
    let res = deps.querier.query(&interchain_query.into())?;
    Ok(res)
}
