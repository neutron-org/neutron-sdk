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

use crate::bindings::query::{
    InterchainQueries, QueryRegisteredQueryResponse, QueryRegisteredQueryResultResponse,
};
use crate::interchain_queries::types::{
    Balances, Delegations, FeePool, GovernmentProposal, KVReconstruct, QueryType, StakingValidator,
    TotalSupply,
};

use crate::{NeutronError, NeutronResult};
use cosmwasm_std::{Deps, Env};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BalanceResponse {
    pub balances: Balances,
    pub last_submitted_local_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TotalSupplyResponse {
    pub supply: TotalSupply,
    pub last_submitted_local_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FeePoolResponse {
    pub pool: FeePool,
    pub last_submitted_local_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ValidatorResponse {
    pub validator: StakingValidator,
    pub last_submitted_local_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProposalResponse {
    pub proposals: GovernmentProposal,
    pub last_submitted_local_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DelegatorDelegationsResponse {
    pub delegations: Vec<cosmwasm_std::Delegation>,
    pub last_submitted_local_height: u64,
}

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
    deps: Deps<InterchainQueries>,
    interchain_query_id: u64,
) -> NeutronResult<QueryRegisteredQueryResponse> {
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
) -> NeutronResult<QueryRegisteredQueryResultResponse> {
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
) -> NeutronResult<T> {
    let registered_query_result = get_interchain_query_result(deps, query_id)?;

    KVReconstruct::reconstruct(&registered_query_result.result.kv_results)
}

/// Returns balance of account on remote chain for particular denom
/// * ***registered_query_id*** is an identifier of the corresponding registered interchain query
pub fn query_balance(
    deps: Deps<InterchainQueries>,
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

/// Returns bank total supply on remote chain for particular denom
/// * ***registered_query_id*** is an identifier of the corresponding registered interchain query
pub fn query_bank_total(
    deps: Deps<InterchainQueries>,
    _env: Env,
    registered_query_id: u64,
) -> NeutronResult<TotalSupplyResponse> {
    let registered_query = get_registered_query(deps, registered_query_id)?;

    check_query_type(registered_query.registered_query.query_type, QueryType::KV)?;

    let total_supply: TotalSupply = query_kv_result(deps, registered_query_id)?;

    Ok(TotalSupplyResponse {
        last_submitted_local_height: registered_query
            .registered_query
            .last_submitted_result_local_height,
        supply: total_supply,
    })
}

/// Returns distribution fee pool on remote chain
/// * ***registered_query_id*** is an identifier of the corresponding registered interchain query
pub fn query_distribution_fee_pool(
    deps: Deps<InterchainQueries>,
    _env: Env,
    registered_query_id: u64,
) -> NeutronResult<FeePoolResponse> {
    let registered_query = get_registered_query(deps, registered_query_id)?;

    check_query_type(registered_query.registered_query.query_type, QueryType::KV)?;

    let fee_pool: FeePool = query_kv_result(deps, registered_query_id)?;

    Ok(FeePoolResponse {
        last_submitted_local_height: registered_query
            .registered_query
            .last_submitted_result_local_height,
        pool: fee_pool,
    })
}

/// Returns staking validator from remote chain
/// * ***registered_query_id*** is an identifier of the corresponding registered interchain query
pub fn query_staking_validators(
    deps: Deps<InterchainQueries>,
    _env: Env,
    registered_query_id: u64,
) -> NeutronResult<ValidatorResponse> {
    let registered_query = get_registered_query(deps, registered_query_id)?;

    check_query_type(registered_query.registered_query.query_type, QueryType::KV)?;

    let validator: StakingValidator = query_kv_result(deps, registered_query_id)?;

    Ok(ValidatorResponse {
        last_submitted_local_height: registered_query
            .registered_query
            .last_submitted_result_local_height,
        validator,
    })
}

/// Returns list of government proposals on the remote chain
/// * ***registered_query_id*** is an identifier of the corresponding registered interchain query
pub fn query_government_proposals(
    deps: Deps<InterchainQueries>,
    _env: Env,
    registered_query_id: u64,
) -> NeutronResult<ProposalResponse> {
    let registered_query = get_registered_query(deps, registered_query_id)?;

    check_query_type(registered_query.registered_query.query_type, QueryType::KV)?;

    let proposals: GovernmentProposal = query_kv_result(deps, registered_query_id)?;

    Ok(ProposalResponse {
        last_submitted_local_height: registered_query
            .registered_query
            .last_submitted_result_local_height,
        proposals,
    })
}

/// Returns delegations of particular delegator on remote chain
/// * ***registered_query_id*** is an identifier of the corresponding registered interchain query
pub fn query_delegations(
    deps: Deps<InterchainQueries>,
    _env: Env,
    registered_query_id: u64,
) -> NeutronResult<DelegatorDelegationsResponse> {
    let registered_query = get_registered_query(deps, registered_query_id)?;

    check_query_type(registered_query.registered_query.query_type, QueryType::KV)?;

    let delegations: Delegations = query_kv_result(deps, registered_query_id)?;

    Ok(DelegatorDelegationsResponse {
        delegations: delegations.delegations,
        last_submitted_local_height: registered_query
            .registered_query
            .last_submitted_result_local_height,
    })
}
