// Copyright 2022 Neutron Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::error::{ContractError, ContractResult};
use crate::types::{Balances, Delegations, KVReconstruct, QueryType};
use cosmwasm_std::{to_binary, Binary, Deps, Env};
use neutron_bindings::query::InterchainQueries;
use neutron_bindings::query::{QueryRegisteredQueryResponse, QueryRegisteredQueryResultResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryBalanceResponse {
    pub balances: Balances,
    pub last_submitted_local_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DelegatorDelegationsResponse {
    pub delegations: Vec<cosmwasm_std::Delegation>,
    pub last_submitted_local_height: u64,
}

/// Parse **actual** query type, checks that it's valid and assert it with **expected** query type
pub fn check_query_type(actual: String, expected: QueryType) -> ContractResult<QueryType> {
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
fn get_registered_query(
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

/// Reads submitted raw KV values for Interchain Query with **query_id** from the storage and reconstructs the result
pub fn query_kv_result<T: KVReconstruct>(
    deps: Deps<InterchainQueries>,
    query_id: u64,
) -> ContractResult<T> {
    let registered_query_result = get_interchain_query_result(deps, query_id)?;

    KVReconstruct::reconstruct(&registered_query_result.result.kv_results)
}

/// Returns balance of account on remote chain for particular denom
/// * ***registered_query_id*** is an identifier of the corresponding registered interchain query
pub fn query_balance(
    deps: Deps<InterchainQueries>,
    _env: Env,
    registered_query_id: u64,
) -> ContractResult<Binary> {
    let registered_query = get_registered_query(deps, registered_query_id)?;

    check_query_type(registered_query.registered_query.query_type, QueryType::KV)?;

    let balances: Balances = query_kv_result(deps, registered_query.registered_query.id)?;

    Ok(to_binary(&QueryBalanceResponse {
        last_submitted_local_height: registered_query
            .registered_query
            .last_submitted_result_local_height,
        balances,
    })?)
}

/// Returns delegations of particular delegator on remote chain
/// * ***registered_query_id*** is an identifier of the corresponding registered interchain query
pub fn query_delegations(
    deps: Deps<InterchainQueries>,
    _env: Env,
    registered_query_id: u64,
) -> ContractResult<Binary> {
    let registered_query = get_registered_query(deps, registered_query_id)?;

    check_query_type(registered_query.registered_query.query_type, QueryType::KV)?;

    let delegations: Delegations = query_kv_result(deps, registered_query.registered_query.id)?;

    Ok(to_binary(&DelegatorDelegationsResponse {
        delegations: delegations.delegations,
        last_submitted_local_height: registered_query
            .registered_query
            .last_submitted_result_local_height,
    })?)
}

/// Queries registered interchain query by **query_id**
pub fn query_registered_query(
    deps: Deps<InterchainQueries>,
    query_id: u64,
) -> ContractResult<Binary> {
    Ok(to_binary(&get_registered_query(deps, query_id)?)?)
}
