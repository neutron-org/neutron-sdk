use crate::error::{ContractError, ContractResult};
use crate::types::{Balances, Delegations, KVReconstruct, KVResult, QueryType};
use neutron_bindings::query::InterchainQueries;

use cosmwasm_std::{to_binary, Binary, Deps, Env};
use neutron_bindings::types::{QueryRegisteredQueryResponse, QueryRegisteredQueryResultResponse};

use crate::msg::{DelegatorDelegationsResponse, QueryBalanceResponse};

fn check_query_type(actual: String, expected: QueryType) -> ContractResult<QueryType> {
    if let Some(t) = QueryType::try_from_str(&actual) {
        if t != expected {
            return Err(ContractError::InvalidQueryType {
                expected,
                actual: t.into(),
            });
        }
        return Ok(t);
    }

    Err(ContractError::InvalidQueryType { expected, actual })
}

/// Queries registered query info
pub(crate) fn get_registered_query(
    deps: Deps<InterchainQueries>,
    interchain_query_id: u64,
) -> ContractResult<QueryRegisteredQueryResponse> {
    let query = InterchainQueries::RegisteredInterchainQuery {
        query_id: interchain_query_id,
    };

    let res: QueryRegisteredQueryResponse = deps.querier.query(&query.into())?;
    Ok(res)
}

/// Queries interchain query result (raw KV storage values or transactions) from Interchain Queries Module
fn get_interchain_query_result(
    deps: Deps<InterchainQueries>,
    interchain_query_id: u64,
) -> ContractResult<QueryRegisteredQueryResultResponse> {
    let interchain_query = InterchainQueries::InterchainQueryResult {
        query_id: interchain_query_id,
    };
    let res = deps.querier.query(&interchain_query.into())?;
    Ok(res)
}

pub fn query_kv_result<T: KVReconstruct>(
    deps: Deps<InterchainQueries>,
    registered_query: QueryRegisteredQueryResponse,
) -> ContractResult<T> {
    check_query_type(
        registered_query.registered_query.query_type.clone(),
        QueryType::KV,
    )?;

    let registered_query_result =
        get_interchain_query_result(deps, registered_query.registered_query.id)?;

    KVReconstruct::reconstruct(&KVResult::new(registered_query_result.result.kv_results))
}

/// Returns balance of account on remote chain for particular denom
pub fn query_balance(
    deps: Deps<InterchainQueries>,
    _env: Env,
    registered_query_id: u64,
) -> ContractResult<Binary> {
    let registered_query = get_registered_query(deps, registered_query_id)?;

    let balances: Balances = query_kv_result(deps, registered_query.clone())?;

    Ok(to_binary(&QueryBalanceResponse {
        last_submitted_local_height: registered_query
            .registered_query
            .last_submitted_result_local_height,
        balances,
    })?)
}

/// Returns delegations of particular delegator on remote chain
pub fn query_delegations(
    deps: Deps<InterchainQueries>,
    _env: Env,
    registered_query_id: u64,
) -> ContractResult<Binary> {
    let registered_query = get_registered_query(deps, registered_query_id)?;

    let delegations: Delegations = query_kv_result(deps, registered_query.clone())?;

    Ok(to_binary(&DelegatorDelegationsResponse {
        delegations: delegations.delegations,
        last_submitted_local_height: registered_query
            .registered_query
            .last_submitted_result_local_height,
    })?)
}

pub fn query_registered_query(
    deps: Deps<InterchainQueries>,
    query_id: u64,
) -> ContractResult<Binary> {
    Ok(to_binary(&get_registered_query(deps, query_id)?)?)
}
