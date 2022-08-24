// Copyright 2022 Neutron
//
// Licensed under the Apache License, Version 2.0 (the "License");
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

use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{TxBody, TxRaw};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cosmwasm_std::{from_binary, to_binary};
use prost::Message as ProstMessage;

use crate::msg::{ExecuteMsg, GetRecipientTxsResponse, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{Transfer, RECIPIENT_TXS};
use neutron_sdk::bindings::msg::NeutronMsg;
use neutron_sdk::bindings::query::{InterchainQueries, QueryRegisteredQueryResponse};
use neutron_sdk::interchain_queries::queries::{
    query_balance, query_delegations, query_registered_query,
};
use neutron_sdk::interchain_queries::{
    register_balance_query, register_delegator_delegations_query, register_transfers_query,
    remove_interchain_query, update_interchain_query, TransferRecipientQuery,
};
use neutron_sdk::sudo::msg::SudoMsg;
use neutron_sdk::{NeutronError, NeutronResult};

use neutron_sdk::interchain_queries::types::COSMOS_SDK_TRANSFER_MSG_URL;
use serde_json_wasm;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> NeutronResult<Response> {
    //TODO
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<InterchainQueries>,
    env: Env,
    _: MessageInfo,
    msg: ExecuteMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    match msg {
        ExecuteMsg::RegisterBalanceQuery {
            zone_id,
            connection_id,
            addr,
            denom,
            update_period,
        } => register_balance_query(
            deps,
            env,
            connection_id,
            zone_id,
            addr,
            denom,
            update_period,
        ),
        ExecuteMsg::RegisterDelegatorDelegationsQuery {
            zone_id,
            connection_id,
            delegator,
            validators,
            update_period,
        } => register_delegator_delegations_query(
            deps,
            env,
            connection_id,
            zone_id,
            delegator,
            validators,
            update_period,
        ),
        ExecuteMsg::RegisterTransfersQuery {
            zone_id,
            connection_id,
            recipient,
            update_period,
        } => register_transfers_query(deps, env, connection_id, zone_id, recipient, update_period),
        ExecuteMsg::UpdateInterchainQuery {
            query_id,
            new_keys,
            new_update_period,
        } => update_interchain_query(query_id, new_keys, new_update_period),
        ExecuteMsg::RemoveInterchainQuery { query_id } => remove_interchain_query(query_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<InterchainQueries>, env: Env, msg: QueryMsg) -> NeutronResult<Binary> {
    match msg {
        //TODO: check if query.result.height is too old (for all interchain queries)
        QueryMsg::Balance { query_id } => query_balance(deps, env, query_id),
        QueryMsg::GetDelegations { query_id } => query_delegations(deps, env, query_id),
        QueryMsg::GetRegisteredQuery { query_id } => query_registered_query(deps, query_id),
        QueryMsg::GetRecipientTxs { recipient } => query_recipient_txs(deps, recipient),
    }
}

fn query_recipient_txs(deps: Deps<InterchainQueries>, recipient: String) -> NeutronResult<Binary> {
    let txs = RECIPIENT_TXS
        .load(deps.storage, &recipient)
        .unwrap_or_default();
    Ok(to_binary(&GetRecipientTxsResponse { transfers: txs })?)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn sudo(deps: DepsMut<InterchainQueries>, env: Env, msg: SudoMsg) -> NeutronResult<Response> {
    match msg {
        SudoMsg::TxQueryResult {
            query_id,
            height,
            data,
        } => sudo_tx_query_result(deps, env, query_id, height, data),
        SudoMsg::KVQueryResult { query_id } => sudo_kv_query_result(deps, env, query_id),
        _ => Ok(Response::default()),
    }
}

/// sudo_check_tx_query_result is an example callback for transaction query results that stores the
/// deposits received as a result on the registered query in the contract's state.
pub fn sudo_tx_query_result(
    deps: DepsMut<InterchainQueries>,
    _env: Env,
    query_id: u64,
    _height: u64,
    data: Binary,
) -> NeutronResult<Response> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_check_tx_query_result received; query_id: {:?}",
            query_id,
        )
        .as_str(),
    );

    // Decode the transaction data
    let tx: TxRaw = TxRaw::decode(data.as_slice())?;
    let body: TxBody = TxBody::decode(tx.body_bytes.as_slice())?;

    // Get the registered query by ID and retrieve the raw query string
    let registered_query: QueryRegisteredQueryResponse =
        from_binary(&query_registered_query(deps.as_ref(), query_id)?)?;
    let transactions_filter = registered_query.registered_query.transactions_filter;

    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_check_tx_query_result loaded query string; query_id: {:?},\
             transactions_filter: {:?}",
            query_id, transactions_filter,
        )
        .as_str(),
    );

    #[allow(clippy::match_single_binding)]
    // Depending of the query type, check the transaction data to see whether is satisfies
    // the original query. If you don't write specific checks for a transaction query type,
    // all submitted results will be treated as valid.
    //
    // TODO: come up with solution to determine transactions filter type
    match registered_query.registered_query.query_type.as_str() {
        _ => {
            // For transfer queries, query data looks like "{"transfer.recipient": "some_address"}"
            let query_data: TransferRecipientQuery =
                serde_json_wasm::from_str(transactions_filter.as_str())?;

            deps.api.debug(
                format!(
                    "WASMDEBUG: sudo_check_tx_query_result parsed query string; query_id: {:?}",
                    query_id
                )
                .as_str(),
            );

            let mut deposits: Vec<Transfer> = vec![];
            for message in body.messages {
                // Skip all messages in this transaction that are not Send messages.
                if message.type_url != *COSMOS_SDK_TRANSFER_MSG_URL.to_string() {
                    continue;
                }

                // Parse a Send message and check that it has the required recipient.
                let transfer_msg: MsgSend = MsgSend::decode(message.value.as_slice())?;
                if transfer_msg.to_address == query_data.recipient {
                    deps.api.debug(
                        format!(
                            "WASMDEBUG: sudo_check_tx_query_result found a matching transaction; \
                             query_id: {:?}, from_address: {:?}",
                            query_id, transfer_msg.from_address,
                        )
                        .as_str(),
                    );
                    for coin in transfer_msg.amount {
                        deposits.push(Transfer {
                            sender: transfer_msg.from_address.clone(),
                            amount: coin.amount,
                            denom: coin.denom,
                            recipient: query_data.recipient.clone(),
                        });
                    }
                }
            }

            // If we didn't find a Send message with the correct recipient, return an error, and
            // this query result will be rejected by Neutron: no data will be saved to state.
            if deposits.is_empty() {
                deps.api.debug(
                    format!(
                        "WASMDEBUG: sudo_check_tx_query_result failed to find a matching \
                         transaction; query_id: {:?}",
                        query_id
                    )
                    .as_str(),
                );
                Err(NeutronError::Std(StdError::generic_err(
                    "failed to find a matching transaction message",
                )))
            } else {
                let mut stored_deposits: Vec<Transfer> = RECIPIENT_TXS
                    .load(deps.storage, query_data.recipient.as_str())
                    .unwrap_or_default();
                stored_deposits.extend(deposits);
                RECIPIENT_TXS.save(
                    deps.storage,
                    query_data.recipient.as_str(),
                    &stored_deposits,
                )?;
                Ok(Response::new())
            }
        }
    }
}

/// sudo_kv_query_result is the contract's callback for KV query results. Note that only the query
/// id is provided, so you need to read the query result from the state.
pub fn sudo_kv_query_result(
    deps: DepsMut<InterchainQueries>,
    _env: Env,
    query_id: u64,
) -> NeutronResult<Response> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_kv_query_result received; query_id: {:?}",
            query_id,
        )
        .as_str(),
    );

    // TODO: provide an actual example. Currently to many things are going to change
    // after @pro0n00gler's PRs to implement this.

    Ok(Response::default())
}
