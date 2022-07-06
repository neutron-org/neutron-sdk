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

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, SubMsgResult,
};
use protobuf::Message;

use crate::error::{ContractError, ContractResult};
use crate::queries::{query_balance, query_delegations, query_transfer_transactions};
use crate::register_queries::{
    register_balance_query, register_delegator_delegations_query, register_transfers_query,
};
use crate::storage::{REGISTERED_INTERCHAIN_QUERIES, TMP_REGISTER_INTERCHAIN_QUERY_REQUEST};
use crate::types::REGISTER_INTERCHAIN_QUERY_REPLY_ID;
use interchain_queries::interchain_queries::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use stargate::interchain::interchainqueries_tx::MsgRegisterInterchainQueryResponse;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> ContractResult<Response> {
    //TODO
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
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
            update_period,
        } => register_delegator_delegations_query(
            deps,
            env,
            connection_id,
            zone_id,
            delegator,
            update_period,
        ),
        ExecuteMsg::RegisterTransfersQuery {
            zone_id,
            connection_id,
            recipient,
            update_period,
        } => register_transfers_query(deps, env, connection_id, zone_id, recipient, update_period),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> ContractResult<Response> {
    // Save registered query id to work with it in query handlers
    if msg.id == REGISTER_INTERCHAIN_QUERY_REPLY_ID {
        let register_query_request = TMP_REGISTER_INTERCHAIN_QUERY_REQUEST.load(deps.storage)?;

        return match msg.result {
            SubMsgResult::Ok(result) => {
                let result_data = match result.data {
                    None => return Err(ContractError::EmptyInterchainQueryResult),
                    Some(data) => data,
                };
                let register_interchain_query_response: MsgRegisterInterchainQueryResponse =
                    Message::parse_from_bytes(result_data.as_slice())?;

                REGISTERED_INTERCHAIN_QUERIES.save(
                    deps.storage,
                    (
                        register_query_request.zone_id.as_str(),
                        register_query_request.query_type.as_str(),
                        register_query_request.query_data.as_str(),
                    ),
                    &register_interchain_query_response.id,
                )?;

                Ok(Response::new().add_attribute("action", "register"))
            }
            SubMsgResult::Err(err) => Err(ContractError::RegisterInterchainQueryFailed(err)),
        };
    } else {
        Err(ContractError::InvalidReplyID(msg.id))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    match msg {
        //TODO: check if query.result.height is too old (for all interchain queries)
        QueryMsg::Balance {
            zone_id,
            addr,
            denom,
        } => query_balance(deps, env, zone_id, addr, denom),
        QueryMsg::GetDelegations { zone_id, delegator } => {
            query_delegations(deps, env, zone_id, delegator)
        }
        QueryMsg::GetTransfers { zone_id, recipient } => {
            query_transfer_transactions(deps, env, zone_id, recipient)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
