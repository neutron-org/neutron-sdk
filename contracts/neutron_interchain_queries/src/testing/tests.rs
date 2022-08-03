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

use super::mock_querier::mock_dependencies as dependencies;
use crate::contract::{execute, query, reply};
use crate::testing::mock_querier::WasmMockQuerier;
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin as CosmosCoin;
use cosmos_sdk_proto::cosmos::staking::v1beta1::Delegation as DelegationSdk;
use cosmwasm_std::testing::{mock_env, mock_info, MockApi, MockStorage};
use cosmwasm_std::{
    from_binary, Addr, Binary, Coin, Delegation, Env, MessageInfo, OwnedDeps, Reply,
    SubMsgResponse, SubMsgResult,
};
use interchain_queries::msg::{ExecuteMsg, QueryMsg};
use interchain_queries::types::REGISTER_INTERCHAIN_QUERY_REPLY_ID;
use interchain_queries::types::{
    create_account_balances_prefix, create_delegations_key, decode_and_convert,
    DelegatorDelegationsResponse, QueryBalanceResponse,
};
use neutron_bindings::query::InterchainQueries;
use neutron_bindings::types::{
    QueryRegisteredQueryResponse, QueryRegisteredQueryResultResponse, QueryResult, RegisteredQuery,
    StorageValue,
};
use prost::Message as ProstMessage;
use protobuf::Message;

use schemars::_serde_json::to_string;
use stargate::interchain::interchainqueries_tx::MsgRegisterInterchainQueryResponse;

pub fn build_registered_query_response(id: u64, last_submitted_result_local_height: u64) -> Binary {
    Binary::from(
        to_string(&QueryRegisteredQueryResponse {
            registered_query: RegisteredQuery {
                id,
                query_data: "".to_string(),
                query_type: "".to_string(),
                zone_id: "".to_string(),
                connection_id: "".to_string(),
                update_period: 0,
                last_emitted_height: 0,
                last_submitted_result_local_height,
                last_submitted_result_remote_height: 0,
            },
        })
        .unwrap()
        .as_bytes(),
    )
}

fn build_interchain_query_balance_response(addr: Addr, denom: String, amount: String) -> Binary {
    let converted_addr_bytes = decode_and_convert(addr.as_str()).unwrap();

    let mut balance_key = create_account_balances_prefix(converted_addr_bytes).unwrap();
    balance_key.extend_from_slice(denom.as_bytes());

    let balance_amount = CosmosCoin { denom, amount };

    let s = StorageValue {
        storage_prefix: "".to_string(),
        key: Binary(balance_key),
        value: Binary(balance_amount.encode_to_vec()),
    };
    Binary::from(
        to_string(&QueryRegisteredQueryResultResponse {
            result: QueryResult {
                kv_results: vec![s],
                height: 123456,
                revision: 2,
            },
        })
        .unwrap()
        .as_bytes(),
    )
}

fn build_delegator_delegations_query_response(delegator: Addr, validators: Vec<Addr>) -> Binary {
    let converted_addr_bytes = decode_and_convert(delegator.as_str()).unwrap();

    let delegations_key = create_delegations_key(converted_addr_bytes).unwrap();
    let values: Vec<StorageValue> = validators
        .iter()
        .map(|v| {
            let delegation = DelegationSdk {
                delegator_address: delegator.to_string(),
                validator_address: v.to_string(),
                shares: "1".to_string(),
            };
            StorageValue {
                storage_prefix: "".to_string(),
                key: Binary(delegations_key.clone()),
                value: Binary(delegation.encode_to_vec()),
            }
        })
        .collect();

    Binary::from(
        to_string(&QueryRegisteredQueryResultResponse {
            result: QueryResult {
                kv_results: values,
                height: 123456,
                revision: 2,
            },
        })
        .unwrap()
        .as_bytes(),
    )
}

// registers an interchain query (full register flow: execute + reply)
fn register_query(
    deps: &mut OwnedDeps<MockStorage, MockApi, WasmMockQuerier, InterchainQueries>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) {
    execute(deps.as_mut(), env, info, msg).unwrap();
    let mut reply_response = MsgRegisterInterchainQueryResponse::new();
    reply_response.id = 1u64;

    let reply_response_bytes = reply_response.write_to_bytes().unwrap();

    reply(
        deps.as_mut(),
        mock_env(),
        Reply {
            id: REGISTER_INTERCHAIN_QUERY_REPLY_ID,
            result: SubMsgResult::Ok(SubMsgResponse {
                events: vec![],
                data: Some(Binary(reply_response_bytes)),
            }),
        },
    )
    .unwrap();
}

#[test]
fn test_query_balance() {
    let mut deps = dependencies(&[]);

    let msg = ExecuteMsg::RegisterBalanceQuery {
        zone_id: "zone".to_string(),
        connection_id: "connection".to_string(),
        update_period: 10,
        addr: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
        denom: "uosmo".to_string(),
    };

    register_query(&mut deps, mock_env(), mock_info("", &[]), msg);

    let registered_query = build_registered_query_response(1, 987);

    deps.querier.add_registred_queries(1, registered_query);
    deps.querier.add_query_response(
        1,
        build_interchain_query_balance_response(
            Addr::unchecked("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"),
            "uosmo".to_string(),
            "8278104".to_string(),
        ),
    );
    let query_balance = QueryMsg::Balance {
        zone_id: "zone".to_string(),
        addr: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
        denom: "uosmo".to_string(),
    };
    let resp: QueryBalanceResponse =
        from_binary(&query(deps.as_ref(), mock_env(), query_balance).unwrap()).unwrap();
    assert_eq!(
        resp,
        QueryBalanceResponse {
            last_submitted_local_height: 987,
            amount: Coin::new(8278104u128, "uosmo")
        }
    )
}

#[test]
fn test_query_delegator_delegations() {
    let mut deps = dependencies(&[]);

    let msg = ExecuteMsg::RegisterDelegatorDelegationsQuery {
        zone_id: "zone".to_string(),
        connection_id: "connection".to_string(),
        update_period: 10,
        delegator: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
    };

    register_query(&mut deps, mock_env(), mock_info("", &[]), msg);

    let delegations_response = build_delegator_delegations_query_response(
        Addr::unchecked("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"),
        vec![
            Addr::unchecked("osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3"),
            Addr::unchecked("osmovaloper1ej2es5fjztqjcd4pwa0zyvaevtjd2y5w37wr9t"),
            Addr::unchecked("osmovaloper1lzhlnpahvznwfv4jmay2tgaha5kmz5qxwmj9we"),
        ],
    );

    let registered_query = build_registered_query_response(1, 987);

    deps.querier.add_query_response(1, delegations_response);
    deps.querier.add_registred_queries(1, registered_query);

    let query_delegations = QueryMsg::GetDelegations {
        zone_id: "zone".to_string(),
        delegator: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
    };
    let resp: DelegatorDelegationsResponse =
        from_binary(&query(deps.as_ref(), mock_env(), query_delegations).unwrap()).unwrap();

    assert_eq!(
        resp,
        DelegatorDelegationsResponse {
            last_submitted_local_height: 987,
            delegations: vec![
                Delegation {
                    delegator: Addr::unchecked("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"),
                    validator: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3".to_string(),
                    amount: Default::default()
                },
                Delegation {
                    delegator: Addr::unchecked("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"),
                    validator: "osmovaloper1ej2es5fjztqjcd4pwa0zyvaevtjd2y5w37wr9t".to_string(),
                    amount: Default::default()
                },
                Delegation {
                    delegator: Addr::unchecked("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"),
                    validator: "osmovaloper1lzhlnpahvznwfv4jmay2tgaha5kmz5qxwmj9we".to_string(),
                    amount: Default::default()
                }
            ],
        }
    )
}
