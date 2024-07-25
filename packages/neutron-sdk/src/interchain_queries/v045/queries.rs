use crate::{
    bindings::query::NeutronQuery,
    interchain_queries::{
        queries::{check_query_type, get_registered_query, query_kv_result},
        types::QueryType,
        v045::types::{
            Balances, Delegations, FeePool, GovernmentProposal, GovernmentProposalVotes,
            SigningInfo, StakingValidator, StdDelegation, TotalSupply, UnbondingDelegations,
        },
    },
    NeutronResult,
};
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
pub struct ValidatorSigningInfoResponse {
    pub signing_infos: SigningInfo,
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
pub struct ProposalVotesResponse {
    pub votes: GovernmentProposalVotes,
    pub last_submitted_local_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DelegatorDelegationsResponse {
    pub delegations: Vec<StdDelegation>,
    pub last_submitted_local_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DelegatorUnbondingDelegationsResponse {
    pub unbonding_delegations: UnbondingDelegations,
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

/// Returns validators signing infos from remote chain
/// * ***registered_query_id*** is an identifier of the corresponding registered interchain query
pub fn query_validators_signing_infos(
    deps: Deps<NeutronQuery>,
    _env: Env,
    registered_query_id: u64,
) -> NeutronResult<ValidatorSigningInfoResponse> {
    let registered_query = get_registered_query(deps, registered_query_id)?;

    check_query_type(registered_query.registered_query.query_type, QueryType::KV)?;

    let signing_infos: SigningInfo = query_kv_result(deps, registered_query_id)?;

    Ok(ValidatorSigningInfoResponse {
        last_submitted_local_height: registered_query
            .registered_query
            .last_submitted_result_local_height,
        signing_infos,
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

/// Returns list of government proposal votes on the remote chain
/// * ***registered_query_id*** is an identifier of the corresponding registered interchain query
pub fn query_government_proposal_votes(
    deps: Deps<NeutronQuery>,
    _env: Env,
    registered_query_id: u64,
) -> NeutronResult<ProposalVotesResponse> {
    let registered_query = get_registered_query(deps, registered_query_id)?;

    check_query_type(registered_query.registered_query.query_type, QueryType::KV)?;

    let votes: GovernmentProposalVotes = query_kv_result(deps, registered_query_id)?;

    Ok(ProposalVotesResponse {
        last_submitted_local_height: registered_query
            .registered_query
            .last_submitted_result_local_height,
        votes,
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

/// Returns list of unbonding delegations of particular delegator on remote chain
/// * ***registered_query_id*** is an identifier of the corresponding registered interchain query
pub fn query_unbonding_delegations(
    deps: Deps<NeutronQuery>,
    _env: Env,
    registered_query_id: u64,
) -> NeutronResult<DelegatorUnbondingDelegationsResponse> {
    let registered_query = get_registered_query(deps, registered_query_id)?;

    check_query_type(registered_query.registered_query.query_type, QueryType::KV)?;

    let unbonding_delegations: UnbondingDelegations = query_kv_result(deps, registered_query_id)?;

    Ok(DelegatorUnbondingDelegationsResponse {
        unbonding_delegations,
        last_submitted_local_height: registered_query
            .registered_query
            .last_submitted_result_local_height,
    })
}
