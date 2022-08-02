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
use interchain_queries::custom_queries::{
    InterchainQueries, ProofOps, QueryRegisteredQueryResponse, QueryRegisteredQueryResultResponse,
    QueryResult, RegisteredQuery, StorageValue,
};
use interchain_queries::msg::{ExecuteMsg, QueryMsg};
use interchain_queries::types::REGISTER_INTERCHAIN_QUERY_REPLY_ID;
use interchain_queries::types::{
    create_account_balances_prefix, create_delegations_key, decode_and_convert,
    DelegatorDelegationsResponse, QueryBalanceResponse,
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
        proof: ProofOps { ops: vec![] },
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
                proof: ProofOps { ops: vec![] },
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

    // protobuf encoded QueryRegisteredQueryResultResponse for balance query
    // TODO: come up with something better than using large base64 string. Good enough for sketch btw
    let delegations_response = "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";
    let delegations_resp_bytes = base64::decode(delegations_response).unwrap();

    deps.querier.add_stargate_response(
        QUERY_REGISTERED_QUERY_RESULT_PATH.to_string(),
        delegations_resp_bytes,
    );

    let registered_query = build_registered_query_response(1, 987);

    deps.querier.add_stargate_response(
        QUERY_REGISTERED_QUERY_PATH.to_string(),
        registered_query.write_to_bytes().unwrap(),
    );

    let query_delegations = QueryMsg::GetDelegations {
        zone_id: "zone".to_string(),
        delegator: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
    };
    let resp: DelegatorDelegationsResponse =
        from_binary(&query(deps.as_ref(), mock_env(), query_delegations).unwrap()).unwrap();

    assert_eq!(
        resp,
        DelegatorDelegationsResponse {
            last_submitted_local_height: registered_query
                .registered_query
                .last_submitted_result_local_height,
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
