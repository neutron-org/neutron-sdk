use crate::error::{ContractError, ContractResult};
use crate::storage::get_registered_query_id;
use crate::types::{
    create_account_balances_prefix, create_delegations_key, decode_and_convert,
    GetBalanceQueryParams, GetDelegatorDelegationsParams, QUERY_BALANCE_QUERY_TYPE,
    QUERY_DELEGATOR_DELEGATIONS_QUERY_TYPE, QUERY_REGISTERED_QUERY_PATH,
    QUERY_REGISTERED_QUERY_RESULT_PATH,
};
use crate::types::{DelegatorDelegationsResponse, QueryBalanceResponse};
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin as CosmosCoin;
use cosmos_sdk_proto::cosmos::staking::v1beta1::Delegation;
use cosmwasm_std::{to_binary, Addr, Binary, Coin, Deps, Env, Uint128};
use stargate::interchain::interchainqueries_query::{
    QueryRegisteredQueryRequest, QueryRegisteredQueryResponse, QueryRegisteredQueryResultRequest,
    QueryRegisteredQueryResultResponse,
};

use prost::Message as ProstMessage;
use protobuf::Message;
use stargate::make_stargate_query;
use std::io::Cursor;
use std::str::FromStr;

/// Queries registered query info
pub fn get_registered_query(
    deps: Deps,
    interchain_query_id: u64,
) -> ContractResult<QueryRegisteredQueryResponse> {
    let mut query = QueryRegisteredQueryRequest::new();
    query.query_id = interchain_query_id;

    let encoded_query_bytes = query.write_to_bytes()?;

    let registered_query: QueryRegisteredQueryResponse = make_stargate_query(
        deps,
        QUERY_REGISTERED_QUERY_PATH.to_string(),
        encoded_query_bytes,
    )?;

    Ok(registered_query)
}

/// Queries interchain query result (raw KV storage values or transactions) from Interchain Queries Module
fn get_interchain_query_result(
    deps: Deps,
    interchain_query_id: u64,
) -> ContractResult<QueryRegisteredQueryResultResponse> {
    let mut interchain_query = QueryRegisteredQueryResultRequest::new();
    interchain_query.query_id = interchain_query_id;

    let encoded_query_bytes = interchain_query.write_to_bytes()?;

    let interchain_query_result: QueryRegisteredQueryResultResponse = make_stargate_query(
        deps,
        QUERY_REGISTERED_QUERY_RESULT_PATH.to_string(),
        encoded_query_bytes,
    )?;

    Ok(interchain_query_result)
}

/// Returns balance of account on remote chain for particular denom
pub fn query_balance(
    deps: Deps,
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

    if registered_query_result.result.is_none() {
        return Err(ContractError::EmptyStargateResult {
            query_type: QUERY_BALANCE_QUERY_TYPE.to_string(),
        });
    }

    #[allow(clippy::unwrap_used)]
    for result in registered_query_result.result.unwrap().kv_results {
        if result.key == balance_key {
            let balance: CosmosCoin = CosmosCoin::decode(Cursor::new(result.value))?;
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
    deps: Deps,
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

    if registered_query_result.result.is_none() {
        return Err(ContractError::EmptyStargateResult {
            query_type: QUERY_DELEGATOR_DELEGATIONS_QUERY_TYPE.to_string(),
        });
    }

    let mut delegations: Vec<cosmwasm_std::Delegation> = vec![];
    #[allow(clippy::unwrap_used)]
    for result in registered_query_result.result.unwrap().kv_results {
        if result.key.starts_with(delegations_key.as_slice()) {
            let delegation_sdk: Delegation = Delegation::decode(Cursor::new(result.value))?;
            let delegation_std = cosmwasm_std::Delegation {
                delegator: Addr::unchecked(delegation_sdk.delegator_address.as_str()),
                validator: delegation_sdk.validator_address,
                amount: Default::default(), // TODO: we can't get this value now, need to update the relayer
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
