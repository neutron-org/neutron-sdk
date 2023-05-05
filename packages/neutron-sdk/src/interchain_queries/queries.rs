use crate::bindings::query::{
    InterchainQueries, QueryRegisteredQueryResponse, QueryRegisteredQueryResultResponse,
};
use crate::interchain_queries::types::{
    KVReconstruct, QueryPayload, QueryType, TransactionFilterItem,
};
use crate::NeutronError;
use crate::{
    bindings::{msg::NeutronMsg, types::KVKey},
    errors::error::NeutronResult,
};
use cosmwasm_std::{Deps, DepsMut, Env};

/// Checks **actual** query type is **expected** query type
pub fn check_query_type(actual: QueryType, expected: QueryType) -> NeutronResult<()> {
    if actual != expected {
        return Err(NeutronError::InvalidQueryType {
            query_type: actual.into(),
        });
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
/// Creates a message to register an Interchain Query with provided params
pub fn new_register_interchain_query_msg(
    _deps: DepsMut<InterchainQueries>,
    _env: Env,
    connection_id: String,
    query_type: QueryType,
    kv_keys: Vec<KVKey>,
    transactions_filter: Vec<TransactionFilterItem>,
    update_period: u64,
) -> NeutronResult<NeutronMsg> {
    match query_type {
        QueryType::KV => NeutronMsg::register_interchain_query(
            QueryPayload::KV(kv_keys),
            connection_id,
            update_period,
        ),
        QueryType::TX => NeutronMsg::register_interchain_query(
            QueryPayload::TX(transactions_filter),
            connection_id,
            update_period,
        ),
    }
}

/// Queries registered query info
pub fn get_registered_query(
    deps: Deps<InterchainQueries>,
    interchain_query_id: u64,
) -> NeutronResult<QueryRegisteredQueryResponse> {
    let query = InterchainQueries::RegisteredInterchainQuery {
        query_id: interchain_query_id,
    };

    let res: QueryRegisteredQueryResponse = deps.querier.query(&query.into())?;
    Ok(res)
}

/// Reads submitted raw KV values for Interchain Query with **query_id** from the storage and reconstructs the result
pub fn query_kv_result<T: KVReconstruct>(
    deps: Deps<InterchainQueries>,
    query_id: u64,
) -> NeutronResult<T> {
    let registered_query_result = get_interchain_query_result(deps, query_id)?;

    KVReconstruct::reconstruct(&registered_query_result.result.kv_results)
}

/// Queries interchain query result (raw KV storage values or transactions) from Interchain Queries Module
fn get_interchain_query_result(
    deps: Deps<InterchainQueries>,
    interchain_query_id: u64,
) -> NeutronResult<QueryRegisteredQueryResultResponse> {
    let interchain_query = InterchainQueries::InterchainQueryResult {
        query_id: interchain_query_id,
    };
    let res = deps.querier.query(&interchain_query.into())?;
    Ok(res)
}
