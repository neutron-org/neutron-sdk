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

use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};

use interchain_queries::error::{ContractError, ContractResult};
use interchain_queries::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use interchain_queries::queries::{query_balance, query_delegations, query_transfer_transactions};
use interchain_queries::register_queries::{
    register_balance_query, register_delegator_delegations_query, register_transfers_query,
};
use interchain_queries::reply::register_interchain_query_reply_handler;
use interchain_queries::types::REGISTER_INTERCHAIN_QUERY_REPLY_ID;

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
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> ContractResult<Response> {
    // Save registered query id to work with it in query handlers
    if msg.id == REGISTER_INTERCHAIN_QUERY_REPLY_ID {
        register_interchain_query_reply_handler(deps, env, msg)
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
        QueryMsg::GetTransfers {
            zone_id,
            recipient,
            start,
            end,
        } => query_transfer_transactions(deps, env, zone_id, recipient, start, end),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
