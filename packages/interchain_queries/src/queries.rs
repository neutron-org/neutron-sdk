use crate::error::{ContractError, ContractResult};
use crate::types::{
    protobuf_coin_to_std_coin, QueryType, COSMOS_SDK_TRANSFER_MSG_URL, DELEGATION_KEY,
    QUERY_REGISTERED_QUERY_PATH, QUERY_REGISTERED_QUERY_RESULT_PATH,
    QUERY_REGISTERED_QUERY_TRANSACTIONS_RESULT_PATH,
};
use crate::types::{
    DelegatorDelegationsResponse, QueryBalanceResponse, Transfer, TransfersResponse,
};
use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin as CosmosCoin;
use cosmos_sdk_proto::cosmos::staking::v1beta1::Delegation;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{TxBody, TxRaw};
use cosmwasm_std::{to_binary, Addr, Binary, Coin, Deps, Env, Uint128};
use stargate::interchain::interchainqueries_query::{
    QueryRegisteredQueryRequest, QueryRegisteredQueryResponse, QueryRegisteredQueryResultRequest,
    QueryRegisteredQueryResultResponse, QuerySubmittedTransactionsRequest,
    QuerySubmittedTransactionsResponse,
};

use prost::Message as ProstMessage;
use protobuf::Message;
use stargate::make_stargate_query;
use std::io::Cursor;
use std::str::FromStr;

fn parse_and_check_query_type(actual: String, expected: QueryType) -> ContractResult<QueryType> {
    if let Some(t) = QueryType::try_from_str(&actual) {
        if t != expected {
            return Err(ContractError::InvalidQueryType {
                expected,
                actual: t.into(),
            });
        }
        Ok(t)
    } else {
        return Err(ContractError::InvalidQueryType { expected, actual });
    }
}

/// Queries registered query info
fn get_registered_query(
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

/// Queries interchain query transactions search result from Interchain Queries Module
fn get_interchain_query_transactions_search_response(
    deps: Deps,
    interchain_query_id: u64,
    start: u64,
    end: u64,
) -> ContractResult<QuerySubmittedTransactionsResponse> {
    let mut interchain_query = QuerySubmittedTransactionsRequest::new();
    interchain_query.query_id = interchain_query_id;
    interchain_query.start = start;
    interchain_query.end = end;

    let encoded_query_bytes = interchain_query.write_to_bytes()?;

    let interchain_query_result: QuerySubmittedTransactionsResponse = make_stargate_query(
        deps,
        QUERY_REGISTERED_QUERY_TRANSACTIONS_RESULT_PATH.to_string(),
        encoded_query_bytes,
    )?;

    Ok(interchain_query_result)
}

/// Returns balance of account on remote chain for particular denom
pub fn query_balance(deps: Deps, _env: Env, registered_query_id: u64) -> ContractResult<Binary> {
    let registered_query = get_registered_query(deps, registered_query_id)?;
    let qt = parse_and_check_query_type(
        registered_query.registered_query.query_type.clone(),
        QueryType::KV,
    )?;

    let registered_query_result = get_interchain_query_result(deps, registered_query_id)?;

    if registered_query_result.result.is_none() {
        return Err(ContractError::EmptyStargateResult {
            query_type: qt,
            query_id: registered_query_id,
        });
    }

    #[allow(clippy::unwrap_used)]
    for (index, result) in registered_query_result
        .result
        .unwrap()
        .kv_results
        .into_iter()
        .enumerate()
    {
        if result.key == registered_query.registered_query.keys[index].key {
            let balance: CosmosCoin = CosmosCoin::decode(Cursor::new(result.value))?;
            let amount = Uint128::from_str(balance.amount.as_str())?;
            return Ok(to_binary(&QueryBalanceResponse {
                last_submitted_local_height: registered_query
                    .registered_query
                    .last_submitted_result_local_height,
                amount: Coin::new(amount.u128(), balance.denom),
            })?);
        }
    }

    Err(ContractError::BalanceNotFound {
        query_id: registered_query_id,
    })
}

/// Returns delegations of particular delegator on remote chain
pub fn query_delegations(
    deps: Deps,
    _env: Env,
    registered_query_id: u64,
) -> ContractResult<Binary> {
    let registered_query = get_registered_query(deps, registered_query_id)?;
    let qt = parse_and_check_query_type(
        registered_query.registered_query.query_type.clone(),
        QueryType::KV,
    )?;

    let registered_query_result = get_interchain_query_result(deps, registered_query_id)?;

    if registered_query_result.result.is_none() {
        return Err(ContractError::EmptyStargateResult {
            query_type: qt,
            query_id: registered_query_id,
        });
    }

    let mut delegations: Vec<cosmwasm_std::Delegation> = vec![];
    #[allow(clippy::unwrap_used)]
    for (index, result) in registered_query_result
        .result
        .unwrap()
        .kv_results
        .into_iter()
        .enumerate()
    {
        if result.key == registered_query.registered_query.keys[index].key {
            if result.key.starts_with(&[DELEGATION_KEY]) {
                let delegation_sdk: Delegation = Delegation::decode(Cursor::new(result.value))?;
                let delegation_std = cosmwasm_std::Delegation {
                    delegator: Addr::unchecked(delegation_sdk.delegator_address.as_str()),
                    validator: delegation_sdk.validator_address,
                    amount: Default::default(), // TODO:
                };
                delegations.push(delegation_std);
            }
        }
    }

    Ok(to_binary(&DelegatorDelegationsResponse {
        delegations,
        last_submitted_local_height: registered_query
            .registered_query
            .last_submitted_result_local_height,
    })?)
}

/// Returns transactions with transfer of remote zone's stake tokens on our interchain account
pub fn query_transfer_transactions(
    deps: Deps,
    _env: Env,
    registered_query_id: u64,
    start: u64,
    end: u64,
) -> ContractResult<Binary> {
    let registered_query = get_registered_query(deps, registered_query_id)?;
    parse_and_check_query_type(
        registered_query.registered_query.query_type.clone(),
        QueryType::TX,
    )?;

    let registered_query_result =
        get_interchain_query_transactions_search_response(deps, registered_query_id, start, end)?;

    let mut transfers: Vec<Transfer> = vec![];
    #[allow(clippy::unwrap_used)]
    for transaction in registered_query_result.transactions {
        let tx: TxRaw = TxRaw::decode(Cursor::new(transaction.data))?;

        let body: TxBody = TxBody::decode(Cursor::new(tx.body_bytes))?;

        for message in body.messages {
            if message.type_url != *COSMOS_SDK_TRANSFER_MSG_URL.to_string() {
                continue;
            }

            let transfer_msg: MsgSend = MsgSend::decode(Cursor::new(message.value))?;

            transfers.push(Transfer {
                tx_id: transaction.id,
                sender: transfer_msg.from_address,
                recipient: transfer_msg.to_address,
                amount: transfer_msg
                    .amount
                    .into_iter()
                    .map(protobuf_coin_to_std_coin)
                    .collect::<ContractResult<Vec<Coin>>>()?,
                height: transaction.height,
            })
        }
    }

    Ok(to_binary(&TransfersResponse {
        transfers,
        last_submitted_local_height: registered_query
            .registered_query
            .last_submitted_result_local_height,
    })?)
}
