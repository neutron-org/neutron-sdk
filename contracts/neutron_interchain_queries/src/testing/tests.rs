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
use crate::contract::{execute, query, sudo_tx_query_result};
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::state::{Transfer, RECIPIENT_TXS};
use crate::testing::mock_querier::WasmMockQuerier;
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin as CosmosCoin;
use cosmwasm_std::testing::{mock_env, mock_info, MockApi, MockStorage};
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Coin, Delegation, Env, MessageInfo, OwnedDeps, StdError,
};
use neutron_sdk::bindings::query::{
    InterchainQueries, QueryRegisteredQueryResponse, QueryRegisteredQueryResultResponse,
};
use neutron_sdk::bindings::types::{
    decode_hex, InterchainQueryResult, KVKey, KVKeys, RegisteredQuery, StorageValue,
};
use neutron_sdk::interchain_queries::helpers::{
    create_account_denom_balance_key, decode_and_convert,
};
use neutron_sdk::interchain_queries::types::{
    Balances, QueryType, TransactionFilterItem, TransactionFilterOp, TransactionFilterValue,
    RECIPIENT_FIELD,
};
use prost::Message as ProstMessage;

use neutron_sdk::interchain_queries::queries::{BalanceResponse, DelegatorDelegationsResponse};
use neutron_sdk::NeutronError;
use schemars::_serde_json::to_string;

enum QueryParam {
    Keys(Vec<KVKey>),
    TransactionsFilter(String),
}

fn build_registered_query_response(
    id: u64,
    param: QueryParam,
    query_type: String,
    last_submitted_result_local_height: u64,
) -> Binary {
    let mut resp = QueryRegisteredQueryResponse {
        registered_query: RegisteredQuery {
            id,
            owner: "".to_string(),
            keys: vec![],
            query_type,
            transactions_filter: "".to_string(),
            connection_id: "".to_string(),
            update_period: 0,
            last_submitted_result_local_height,
            last_submitted_result_remote_height: 0,
        },
    };
    match param {
        QueryParam::Keys(keys) => resp.registered_query.keys = keys,
        QueryParam::TransactionsFilter(transactions_filter) => {
            resp.registered_query.transactions_filter = transactions_filter
        }
    }

    Binary::from(to_string(&resp).unwrap().as_bytes())
}

fn build_interchain_query_balance_response(addr: Addr, denom: String, amount: String) -> Binary {
    let converted_addr_bytes = decode_and_convert(addr.as_str()).unwrap();

    let balance_key = create_account_denom_balance_key(converted_addr_bytes, &denom).unwrap();

    let balance_amount = CosmosCoin { denom, amount };

    let s = StorageValue {
        storage_prefix: "".to_string(),
        key: Binary(balance_key),
        value: Binary(balance_amount.encode_to_vec()),
    };
    Binary::from(
        to_string(&QueryRegisteredQueryResultResponse {
            result: InterchainQueryResult {
                kv_results: vec![s],
                height: 123456,
                revision: 2,
            },
        })
        .unwrap()
        .as_bytes(),
    )
}

// registers an interchain query
fn register_query(
    deps: &mut OwnedDeps<MockStorage, MockApi, WasmMockQuerier, InterchainQueries>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> KVKeys {
    let reg_msgs = execute(deps.as_mut(), env, info, msg).unwrap();
    for attr in reg_msgs.attributes {
        if attr.key == "kv_keys" && !attr.value.is_empty() {
            return KVKeys::from_string(attr.value).unwrap();
        }
    }

    KVKeys(vec![])
}

#[test]
fn test_query_balance() {
    let mut deps = dependencies(&[]);

    let msg = ExecuteMsg::RegisterBalanceQuery {
        connection_id: "connection".to_string(),
        update_period: 10,
        addr: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
        denom: "uosmo".to_string(),
    };

    let keys = register_query(&mut deps, mock_env(), mock_info("", &[]), msg);

    let registered_query =
        build_registered_query_response(1, QueryParam::Keys(keys.0), QueryType::KV.into(), 987);

    deps.querier.add_registred_queries(1, registered_query);
    deps.querier.add_query_response(
        1,
        build_interchain_query_balance_response(
            Addr::unchecked("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"),
            "uosmo".to_string(),
            "8278104".to_string(),
        ),
    );
    let query_balance = QueryMsg::Balance { query_id: 1 };
    let resp: BalanceResponse =
        from_binary(&query(deps.as_ref(), mock_env(), query_balance).unwrap()).unwrap();
    assert_eq!(
        resp,
        BalanceResponse {
            last_submitted_local_height: 987,
            balances: Balances {
                coins: vec![Coin::new(8278104u128, "uosmo")]
            }
        }
    )
}

#[test]
fn test_query_delegator_delegations() {
    let mut deps = dependencies(&[]);

    let msg = ExecuteMsg::RegisterDelegatorDelegationsQuery {
        connection_id: "connection".to_string(),
        update_period: 10,
        delegator: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
        validators: vec![
            "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3".to_string(),
            "osmovaloper1ej2es5fjztqjcd4pwa0zyvaevtjd2y5w37wr9t".to_string(),
            "osmovaloper1lzhlnpahvznwfv4jmay2tgaha5kmz5qxwmj9we".to_string(),
        ],
    };

    let keys = register_query(&mut deps, mock_env(), mock_info("", &[]), msg);

    let delegations_response = QueryRegisteredQueryResultResponse {
        result: InterchainQueryResult {
            // response for `RegisterDelegatorDelegationsQuery` with necessary KV values to test reconstruction logic.
            // The values are taken from osmosis network
            kv_results: vec![
                // params value of staking module for key 'staking/BondDenom'
                // value: uosmo
                StorageValue {
                    storage_prefix: "params".to_string(),
                    key: Binary::from(base64::decode("c3Rha2luZy9Cb25kRGVub20=").unwrap()),
                    value: Binary::from(base64::decode("InVvc21vIg==").unwrap()),
                },
                // delegation
                // from: osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs
                // to: osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3
                // delegation_shares: "5177628000000000000000000"
                StorageValue {
                    storage_prefix: "staking".to_string(),
                    key: Binary::from(decode_hex("311420a959e0d22e201f727137f2d7c41a5dc63b90b8141ab940697a73dd080edafeb538ad408b5cae0264").unwrap()),
                    value: Binary::from(base64::decode("Citvc21vMXl6NTRuY3hqOWNzcDd1bjN4bGVkMDNxNnRocnJoeTljenRrZnpzEjJvc21vdmFsb3BlcjFyMnU1cTZ0Nncwd3Nzcms2bDY2bjN0MnEzZHcydXFueTRnajJlMxoZNTE3NzYyODAwMDAwMDAwMDAwMDAwMDAwMA==").unwrap()),
                },
                // validator: osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3
                // delegator_shares: "2845862840643000000000000000000"
                // total tokens: "2845862840643"
                StorageValue {
                    storage_prefix: "staking".to_string(),
                    key: Binary::from(decode_hex("21141ab940697a73dd080edafeb538ad408b5cae0264").unwrap()),
                    value: Binary::from(base64::decode("CjJvc21vdmFsb3BlcjFyMnU1cTZ0Nncwd3Nzcms2bDY2bjN0MnEzZHcydXFueTRnajJlMxJDCh0vY29zbW9zLmNyeXB0by5lZDI1NTE5LlB1YktleRIiCiCaZhCbacCetQorko3LfUUJX2UEyX38qBGVri8GyH8lcCADKg0yODQ1ODYyODQwNjQzMh8yODQ1ODYyODQwNjQzMDAwMDAwMDAwMDAwMDAwMDAwOqQCChRzdHJhbmdlbG92ZS12ZW50dXJlcxIQRDBEOEI4MEYxQzVDNzBCNRocaHR0cHM6Ly9zdHJhbmdlbG92ZS52ZW50dXJlcyrbAScuLi5iZWNhdXNlIG9mIHRoZSBhdXRvbWF0ZWQgYW5kIGlycmV2b2NhYmxlIGRlY2lzaW9uLW1ha2luZyBwcm9jZXNzIHdoaWNoIHJ1bGVzIG91dCBodW1hbiBtZWRkbGluZywgdGhlIERvb21zZGF5IG1hY2hpbmUgaXMgdGVycmlmeWluZyBhbmQgc2ltcGxlIHRvIHVuZGVyc3RhbmQgYW5kIGNvbXBsZXRlbHkgY3JlZGlibGUgYW5kIGNvbnZpbmNpbmcuJyAtIERyLiBTdHJhbmdlbG92ZUoAUkwKPAoRNTAwMDAwMDAwMDAwMDAwMDASEzEwMDAwMDAwMDAwMDAwMDAwMDAaEjUwMDAwMDAwMDAwMDAwMDAwMBIMCPetyYYGEKPoosUCWgEx").unwrap()),
                },
                // delegation
                // from: osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs
                // to: osmovaloper1ej2es5fjztqjcd4pwa0zyvaevtjd2y5w37wr9t
                // delegation_shares: "29620221000000000000000000"
                StorageValue {
                    storage_prefix: "staking".to_string(),
                    key: Binary::from(decode_hex("311420a959e0d22e201f727137f2d7c41a5dc63b90b814cc9598513212c12c36a1775e2233b962e4d5128e").unwrap()),
                    value: Binary::from(base64::decode("Citvc21vMXl6NTRuY3hqOWNzcDd1bjN4bGVkMDNxNnRocnJoeTljenRrZnpzEjJvc21vdmFsb3BlcjFlajJlczVmanp0cWpjZDRwd2Ewenl2YWV2dGpkMnk1dzM3d3I5dBoaMjk2MjAyMjEwMDAwMDAwMDAwMDAwMDAwMDA=").unwrap()),
                },
                // validator: osmovaloper1ej2es5fjztqjcd4pwa0zyvaevtjd2y5w37wr9t
                // delegator_shares: "3054477259038000000000000000000"
                // total tokens: "3054477259038"
                StorageValue {
                    storage_prefix: "staking".to_string(),
                    key: Binary::from(decode_hex("2114cc9598513212c12c36a1775e2233b962e4d5128e").unwrap()),
                    value: Binary::from(base64::decode("CjJvc21vdmFsb3BlcjFlajJlczVmanp0cWpjZDRwd2Ewenl2YWV2dGpkMnk1dzM3d3I5dBJDCh0vY29zbW9zLmNyeXB0by5lZDI1NTE5LlB1YktleRIiCiA27dgAuZV/uS9FdsILGWLBw8eYPy+ZEyv1Df2VsrjXDiADKg0zMDU0NDc3MjU5MDM4Mh8zMDU0NDc3MjU5MDM4MDAwMDAwMDAwMDAwMDAwMDAwOoEBChFGcmVucyAo8J+knSzwn6SdKRIQQzQ3ODQ1MjI2NjYyQUY0NxoSaHR0cHM6Ly9mcmVucy5hcm15IhtzZWN1cml0eUBraWRzb250aGVibG9jay54eXoqKVlvdXIgZnJpZW5kbHkgdmFsaWRhdG9yIGZvciBjb3Ntb3MgY2hhaW5zQP3HpQFKCwj3zq6PBhCfrO86UkoKOgoRNTAwMDAwMDAwMDAwMDAwMDASEjUwMDAwMDAwMDAwMDAwMDAwMBoRNTAwMDAwMDAwMDAwMDAwMDASDAjg1rSQBhDkudCDAVoDNTAw").unwrap()),
                },
                // delegation
                // from: osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs
                // to: osmovaloper1lzhlnpahvznwfv4jmay2tgaha5kmz5qxwmj9we
                // delegation_shares: "219920000000000000000000"
                StorageValue {
                    storage_prefix: "staking".to_string(),
                    key: Binary::from(decode_hex("311420a959e0d22e201f727137f2d7c41a5dc63b90b814f8aff987b760a6e4b2b2df48a5a3b7ed2db15006").unwrap()),
                    value: Binary::from(base64::decode("Citvc21vMXl6NTRuY3hqOWNzcDd1bjN4bGVkMDNxNnRocnJoeTljenRrZnpzEjJvc21vdmFsb3BlcjFsemhsbnBhaHZ6bndmdjRqbWF5MnRnYWhhNWttejVxeHdtajl3ZRoYMjE5OTIwMDAwMDAwMDAwMDAwMDAwMDAw").unwrap()),
                },
                // validator: osmovaloper1lzhlnpahvznwfv4jmay2tgaha5kmz5qxwmj9we
                // delegator_shares: "3201438898476000000000000000000"
                // total tokens: "3201438898476"
                StorageValue {
                    storage_prefix: "staking".to_string(),
                    key: Binary::from(decode_hex("2114f8aff987b760a6e4b2b2df48a5a3b7ed2db15006").unwrap()),
                    value: Binary::from(base64::decode("CjJvc21vdmFsb3BlcjFsemhsbnBhaHZ6bndmdjRqbWF5MnRnYWhhNWttejVxeHdtajl3ZRJDCh0vY29zbW9zLmNyeXB0by5lZDI1NTE5LlB1YktleRIiCiBPXCnkQvO+pU6oGbp4ZiJBBZ7RNoLYtXYFOEdpXGH+uSADKg0zMjAxNDM4ODk4NDc2Mh8zMjAxNDM4ODk4NDc2MDAwMDAwMDAwMDAwMDAwMDAwOp8CCgtDaXRhZGVsLm9uZRIQRUJCMDNFQjRCQjRDRkNBNxoTaHR0cHM6Ly9jaXRhZGVsLm9uZSroAUNpdGFkZWwub25lIGlzIGEgbXVsdGktYXNzZXQgbm9uLWN1c3RvZGlhbCBzdGFraW5nIHBsYXRmb3JtIHRoYXQgbGV0cyBhbnlvbmUgYmVjb21lIGEgcGFydCBvZiBkZWNlbnRyYWxpemVkIGluZnJhc3RydWN0dXJlIGFuZCBlYXJuIHBhc3NpdmUgaW5jb21lLiBTdGFrZSB3aXRoIG91ciBub2RlcyBvciBhbnkgb3RoZXIgdmFsaWRhdG9yIGFjcm9zcyBtdWx0aXBsZSBuZXR3b3JrcyBpbiBhIGZldyBjbGlja3NKAFJECjoKETUwMDAwMDAwMDAwMDAwMDAwEhIyMDAwMDAwMDAwMDAwMDAwMDAaETMwMDAwMDAwMDAwMDAwMDAwEgYIkKKzhgZaATE=").unwrap()),
                }
            ],
            height: 0,
            revision: 0,
        },
    };

    let registered_query =
        build_registered_query_response(1, QueryParam::Keys(keys.0), QueryType::KV.into(), 987);

    deps.querier
        .add_query_response(1, to_binary(&delegations_response).unwrap());
    deps.querier.add_registred_queries(1, registered_query);

    let query_delegations = QueryMsg::GetDelegations { query_id: 1 };
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
                    amount: Coin::new(5177628u128, "uosmo".to_string())
                },
                Delegation {
                    delegator: Addr::unchecked("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"),
                    validator: "osmovaloper1ej2es5fjztqjcd4pwa0zyvaevtjd2y5w37wr9t".to_string(),
                    amount: Coin::new(29620221u128, "uosmo".to_string())
                },
                Delegation {
                    delegator: Addr::unchecked("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"),
                    validator: "osmovaloper1lzhlnpahvznwfv4jmay2tgaha5kmz5qxwmj9we".to_string(),
                    amount: Coin::new(219920u128, "uosmo".to_string())
                }
            ],
        }
    )
}

#[test]
fn test_sudo_tx_query_result_callback() {
    let mut deps = dependencies(&[]);
    let env = mock_env();
    let watched_addr: String = "neutron1fj6yqrkpw6fmp7f7jhj57dujfpwal4m25dafzx".to_string();
    let query_id: u64 = 1u64;
    let height: u64 = 1u64;
    let msg = ExecuteMsg::RegisterTransfersQuery {
        connection_id: "connection".to_string(),
        update_period: 1u64,
        recipient: watched_addr.clone(),
        min_height: None,
    };
    execute(deps.as_mut(), env.clone(), mock_info("", &[]), msg).unwrap();
    let registered_query = build_registered_query_response(
        1,
        QueryParam::TransactionsFilter(
            to_string(&vec![&TransactionFilterItem {
                field: RECIPIENT_FIELD.to_string(),
                op: TransactionFilterOp::Eq,
                value: TransactionFilterValue::String(watched_addr.clone()),
            }])
            .unwrap(),
        ),
        QueryType::TX.into(),
        0,
    );
    deps.querier.add_registred_queries(1, registered_query);

    // simulate neutron's SudoTxQueryResult call with the following payload:
    // a sending from neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf to watched_addr of 10000 stake
    let data: Binary = Binary::from(base64::decode("CpMBCpABChwvY29zbW9zLmJhbmsudjFiZXRhMS5Nc2dTZW5kEnAKLm5ldXRyb24xMGg5c3RjNXY2bnRnZXlnZjV4Zjk0NW5qcXE1aDMycjU0cmY3a2YSLm5ldXRyb24xZmo2eXFya3B3NmZtcDdmN2poajU3ZHVqZnB3YWw0bTI1ZGFmengaDgoFc3Rha2USBTEwMDAwEmcKUApGCh8vY29zbW9zLmNyeXB0by5zZWNwMjU2azEuUHViS2V5EiMKIQJPYibh+Zef13ZkulPqI27rV5xswZ0H/vh1Tnymp1RHPhIECgIIARgAEhMKDQoFc3Rha2USBDEwMDAQwJoMGkAIiXNJXmA57KhyaWpKcLLr3602A5+hlvv/b4PgcDDm9y0qikC+biNZXin1dEMpHOvX9DwOWJ9utv6EKljiSyfT").unwrap());
    sudo_tx_query_result(deps.as_mut(), env.clone(), query_id, height, data).unwrap();

    // ensure the callback has worked and contract's state has changed
    let txs = RECIPIENT_TXS.load(&deps.storage, &watched_addr).unwrap();
    assert_eq!(
        txs,
        Vec::from([Transfer {
            recipient: watched_addr.clone(),
            sender: "neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf".to_string(),
            denom: "stake".to_string(),
            amount: "10000".to_string(),
        }])
    );

    // simulate neutron's SudoTxQueryResult call with the following payload:
    // a sending from neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf to another addr of 10000 stake
    let data: Binary = Binary::from(base64::decode("CpMBCpABChwvY29zbW9zLmJhbmsudjFiZXRhMS5Nc2dTZW5kEnAKLm5ldXRyb24xMGg5c3RjNXY2bnRnZXlnZjV4Zjk0NW5qcXE1aDMycjU0cmY3a2YSLm5ldXRyb24xNHV4dnUyMmxocmF6eXhhZGFxdjVkNmxzd3UwcDI3NmxsN2hya2waDgoFc3Rha2USBTEwMDAwEmcKUApGCh8vY29zbW9zLmNyeXB0by5zZWNwMjU2azEuUHViS2V5EiMKIQJPYibh+Zef13ZkulPqI27rV5xswZ0H/vh1Tnymp1RHPhIECgIIARgAEhMKDQoFc3Rha2USBDEwMDAQwJoMGkBEv2CW/0gIrankNl4aGs9LXy2BKA6kAWyl4MUxmXnbnjRpgaNbQIyo4i7nUgVsuOpqzAdudM2M53OSU0Dmo5tF").unwrap());
    let res = sudo_tx_query_result(deps.as_mut(), env.clone(), query_id, height, data);

    // ensure the callback has returned an error and contract's state hasn't changed
    assert_eq!(
        res.unwrap_err(),
        NeutronError::Std(StdError::generic_err(
            "failed to find a matching transaction message",
        ))
    );
    let txs = RECIPIENT_TXS.load(&deps.storage, &watched_addr).unwrap();
    assert_eq!(
        txs,
        Vec::from([Transfer {
            recipient: watched_addr.clone(),
            sender: "neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf".to_string(),
            denom: "stake".to_string(),
            amount: "10000".to_string(),
        }])
    );

    // simulate neutron's SudoTxQueryResult call with the following payload:
    // a sending from neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf to watched_addr of 10000 stake
    let data: Binary = Binary::from(base64::decode("CpMBCpABChwvY29zbW9zLmJhbmsudjFiZXRhMS5Nc2dTZW5kEnAKLm5ldXRyb24xMGg5c3RjNXY2bnRnZXlnZjV4Zjk0NW5qcXE1aDMycjU0cmY3a2YSLm5ldXRyb24xZmo2eXFya3B3NmZtcDdmN2poajU3ZHVqZnB3YWw0bTI1ZGFmengaDgoFc3Rha2USBTEwMDAwEmcKUApGCh8vY29zbW9zLmNyeXB0by5zZWNwMjU2azEuUHViS2V5EiMKIQJPYibh+Zef13ZkulPqI27rV5xswZ0H/vh1Tnymp1RHPhIECgIIARgAEhMKDQoFc3Rha2USBDEwMDAQwJoMGkAIiXNJXmA57KhyaWpKcLLr3602A5+hlvv/b4PgcDDm9y0qikC+biNZXin1dEMpHOvX9DwOWJ9utv6EKljiSyfT").unwrap());
    sudo_tx_query_result(deps.as_mut(), env, query_id, height, data).unwrap();

    // ensure the callback has worked and contract's state has changed again
    let txs = RECIPIENT_TXS.load(&deps.storage, &watched_addr).unwrap();
    assert_eq!(
        txs,
        Vec::from([
            Transfer {
                recipient: watched_addr.clone(),
                sender: "neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf".to_string(),
                denom: "stake".to_string(),
                amount: "10000".to_string(),
            },
            Transfer {
                recipient: watched_addr,
                sender: "neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf".to_string(),
                denom: "stake".to_string(),
                amount: "10000".to_string(),
            }
        ])
    );
}

#[test]
fn test_sudo_tx_query_result_min_height_callback() {
    let mut deps = dependencies(&[]);
    let env = mock_env();
    let watched_addr: String = "neutron1fj6yqrkpw6fmp7f7jhj57dujfpwal4m25dafzx".to_string();
    let query_id: u64 = 1u64;
    let height: u64 = 1u64;
    let msg = ExecuteMsg::RegisterTransfersQuery {
        connection_id: "connection".to_string(),
        update_period: 1u64,
        recipient: watched_addr.clone(),
        min_height: Some(100000),
    };
    execute(deps.as_mut(), env.clone(), mock_info("", &[]), msg).unwrap();
    let registered_query = build_registered_query_response(
        1,
        QueryParam::TransactionsFilter(
            to_string(&vec![&TransactionFilterItem {
                field: RECIPIENT_FIELD.to_string(),
                op: TransactionFilterOp::Eq,
                value: TransactionFilterValue::String(watched_addr.clone()),
            }])
            .unwrap(),
        ),
        QueryType::TX.into(),
        0,
    );
    deps.querier.add_registred_queries(1, registered_query);

    // simulate neutron's SudoTxQueryResult call with the following payload:
    // a sending from neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf to watched_addr of 10000 stake
    let data: Binary = Binary::from(base64::decode("CpMBCpABChwvY29zbW9zLmJhbmsudjFiZXRhMS5Nc2dTZW5kEnAKLm5ldXRyb24xMGg5c3RjNXY2bnRnZXlnZjV4Zjk0NW5qcXE1aDMycjU0cmY3a2YSLm5ldXRyb24xZmo2eXFya3B3NmZtcDdmN2poajU3ZHVqZnB3YWw0bTI1ZGFmengaDgoFc3Rha2USBTEwMDAwEmcKUApGCh8vY29zbW9zLmNyeXB0by5zZWNwMjU2azEuUHViS2V5EiMKIQJPYibh+Zef13ZkulPqI27rV5xswZ0H/vh1Tnymp1RHPhIECgIIARgAEhMKDQoFc3Rha2USBDEwMDAQwJoMGkAIiXNJXmA57KhyaWpKcLLr3602A5+hlvv/b4PgcDDm9y0qikC+biNZXin1dEMpHOvX9DwOWJ9utv6EKljiSyfT").unwrap());
    sudo_tx_query_result(deps.as_mut(), env.clone(), query_id, height, data).unwrap();

    // ensure the callback has worked and contract's state has changed
    let txs = RECIPIENT_TXS.load(&deps.storage, &watched_addr).unwrap();
    assert_eq!(
        txs,
        Vec::from([Transfer {
            recipient: watched_addr.clone(),
            sender: "neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf".to_string(),
            denom: "stake".to_string(),
            amount: "10000".to_string(),
        }])
    );

    // simulate neutron's SudoTxQueryResult call with the following payload:
    // a sending from neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf to another addr of 10000 stake
    let data: Binary = Binary::from(base64::decode("CpMBCpABChwvY29zbW9zLmJhbmsudjFiZXRhMS5Nc2dTZW5kEnAKLm5ldXRyb24xMGg5c3RjNXY2bnRnZXlnZjV4Zjk0NW5qcXE1aDMycjU0cmY3a2YSLm5ldXRyb24xNHV4dnUyMmxocmF6eXhhZGFxdjVkNmxzd3UwcDI3NmxsN2hya2waDgoFc3Rha2USBTEwMDAwEmcKUApGCh8vY29zbW9zLmNyeXB0by5zZWNwMjU2azEuUHViS2V5EiMKIQJPYibh+Zef13ZkulPqI27rV5xswZ0H/vh1Tnymp1RHPhIECgIIARgAEhMKDQoFc3Rha2USBDEwMDAQwJoMGkBEv2CW/0gIrankNl4aGs9LXy2BKA6kAWyl4MUxmXnbnjRpgaNbQIyo4i7nUgVsuOpqzAdudM2M53OSU0Dmo5tF").unwrap());
    let res = sudo_tx_query_result(deps.as_mut(), env.clone(), query_id, height, data);

    // ensure the callback has returned an error and contract's state hasn't changed
    assert_eq!(
        res.unwrap_err(),
        NeutronError::Std(StdError::generic_err(
            "failed to find a matching transaction message",
        ))
    );
    let txs = RECIPIENT_TXS.load(&deps.storage, &watched_addr).unwrap();
    assert_eq!(
        txs,
        Vec::from([Transfer {
            recipient: watched_addr.clone(),
            sender: "neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf".to_string(),
            denom: "stake".to_string(),
            amount: "10000".to_string(),
        }])
    );

    // simulate neutron's SudoTxQueryResult call with the following payload:
    // a sending from neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf to watched_addr of 10000 stake
    let data: Binary = Binary::from(base64::decode("CpMBCpABChwvY29zbW9zLmJhbmsudjFiZXRhMS5Nc2dTZW5kEnAKLm5ldXRyb24xMGg5c3RjNXY2bnRnZXlnZjV4Zjk0NW5qcXE1aDMycjU0cmY3a2YSLm5ldXRyb24xZmo2eXFya3B3NmZtcDdmN2poajU3ZHVqZnB3YWw0bTI1ZGFmengaDgoFc3Rha2USBTEwMDAwEmcKUApGCh8vY29zbW9zLmNyeXB0by5zZWNwMjU2azEuUHViS2V5EiMKIQJPYibh+Zef13ZkulPqI27rV5xswZ0H/vh1Tnymp1RHPhIECgIIARgAEhMKDQoFc3Rha2USBDEwMDAQwJoMGkAIiXNJXmA57KhyaWpKcLLr3602A5+hlvv/b4PgcDDm9y0qikC+biNZXin1dEMpHOvX9DwOWJ9utv6EKljiSyfT").unwrap());
    sudo_tx_query_result(deps.as_mut(), env, query_id, height, data).unwrap();

    // ensure the callback has worked and contract's state has changed again
    let txs = RECIPIENT_TXS.load(&deps.storage, &watched_addr).unwrap();
    assert_eq!(
        txs,
        Vec::from([
            Transfer {
                recipient: watched_addr.clone(),
                sender: "neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf".to_string(),
                denom: "stake".to_string(),
                amount: "10000".to_string(),
            },
            Transfer {
                recipient: watched_addr,
                sender: "neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf".to_string(),
                denom: "stake".to_string(),
                amount: "10000".to_string(),
            }
        ])
    );
}
