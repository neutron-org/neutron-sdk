use crate::custom_queries::{
    InterchainQueries, QueryRegisteredQueryResponse, QueryRegisteredQueryResultResponse,
};
use crate::error::{ContractError, ContractResult};
use crate::storage::get_registered_query_id;
use crate::types::{
    create_account_balances_prefix, create_delegations_key, decode_and_convert,
    GetBalanceQueryParams, GetDelegatorDelegationsParams, QUERY_BALANCE_QUERY_TYPE,
    QUERY_DELEGATOR_DELEGATIONS_QUERY_TYPE,
};
use crate::types::{DelegatorDelegationsResponse, QueryBalanceResponse};
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin as CosmosCoin;
use cosmos_sdk_proto::cosmos::staking::v1beta1::Delegation;
use cosmwasm_std::{to_binary, Addr, Binary, Coin, Deps, Env, Uint128};

use prost::Message as ProstMessage;
use std::io::Cursor;
use std::str::FromStr;

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

/// Returns balance of account on remote chain for particular denom
pub fn query_balance(
    deps: Deps<InterchainQueries>,
    _env: Env,
    zone_id: String,
    addr: String,
    denom: String,
) -> ContractResult<Binary> {
    let query_data = GetBalanceQueryParams {
        addr: addr.clone(),
        denom: denom.clone(),
    };
    let registered_query_id = get_registered_query_id(
        deps,
        zone_id.as_str(),
        QUERY_BALANCE_QUERY_TYPE,
        &query_data,
    )?;

    let registered_query = get_registered_query(deps, registered_query_id)?;

    let registered_query_result = get_interchain_query_result(deps, registered_query_id)?;

    let converted_addr_bytes = decode_and_convert(addr.as_str())?;

    let mut balance_key = create_account_balances_prefix(converted_addr_bytes)?;
    balance_key.extend_from_slice(denom.as_bytes());

    #[allow(clippy::unwrap_used)]
    for result in registered_query_result.result.kv_results {
        if result.key == balance_key {
            let balance: CosmosCoin = CosmosCoin::decode(Cursor::new(result.value.as_ref()))?;
            let amount = Uint128::from_str(balance.amount.as_str())?;
            return Ok(to_binary(&QueryBalanceResponse {
                last_submitted_local_height: registered_query
                    .registered_query
                    .last_submitted_result_local_height,
                amount: Coin::new(amount.u128(), denom),
            })?);
        }
    }

    Err(ContractError::BalanceNotFound {
        denom,
        recipient: addr,
    })
}

/// Returns delegations of particular delegator on remote chain
pub fn query_delegations(
    deps: Deps<InterchainQueries>,
    _env: Env,
    zone_id: String,
    delegator: String,
) -> ContractResult<Binary> {
    let query_data = GetDelegatorDelegationsParams {
        delegator: delegator.clone(),
    };
    let registered_query_id = get_registered_query_id(
        deps,
        zone_id.as_str(),
        QUERY_DELEGATOR_DELEGATIONS_QUERY_TYPE,
        &query_data,
    )?;

    let registered_query = get_registered_query(deps, registered_query_id)?;

    let registered_query_result = get_interchain_query_result(deps, registered_query_id)?;

    let converted_addr_bytes = decode_and_convert(delegator.as_str())?;

    let delegations_key = create_delegations_key(converted_addr_bytes)?;

    let mut delegations: Vec<cosmwasm_std::Delegation> = vec![];
    #[allow(clippy::unwrap_used)]
    for result in registered_query_result.result.kv_results {
        if result.key.starts_with(delegations_key.as_slice()) {
            let delegation_sdk: Delegation =
                Delegation::decode(Cursor::new(result.value.as_ref()))?;
            let delegation_std = cosmwasm_std::Delegation {
                delegator: Addr::unchecked(delegation_sdk.delegator_address.as_str()),
                validator: delegation_sdk.validator_address,
                amount: Default::default(), // NOTE: implemented in the commit https://github.com/neutron-org/neutron-contracts/commit/07880d28ad5a95a798dabaaafc9d3677cf0338da?diff=split
            };
            delegations.push(delegation_std);
        }
    }

    Ok(to_binary(&DelegatorDelegationsResponse {
        delegations,
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
