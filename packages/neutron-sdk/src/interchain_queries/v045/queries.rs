use crate::bindings::query::NeutronQuery;
use crate::interchain_queries::queries::{check_query_type, get_registered_query, query_kv_result};
use crate::interchain_queries::types::QueryType;
use crate::interchain_queries::v045::types::{
    Balances, Delegations, FeePool, GovernmentProposal, StakingValidator, TotalSupply,
};
use crate::NeutronResult;
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

/// Returns bank total supply on remote chain for particular denom
/// * ***registered_query_id*** is an identifier of the corresponding registered interchain query
pub fn query_bank_total(
    deps: Deps<NeutronQuery>,
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
    deps: Deps<NeutronQuery>,
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
    deps: Deps<NeutronQuery>,
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
    deps: Deps<NeutronQuery>,
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
    deps: Deps<NeutronQuery>,
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
