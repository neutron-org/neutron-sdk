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
use crate::contract::{execute, query};
use crate::testing::mock_querier::WasmMockQuerier;
use cosmwasm_std::testing::{mock_env, mock_info, MockApi, MockStorage};
use cosmwasm_std::{from_binary, Addr, Coin, Delegation, Env, MessageInfo, OwnedDeps};
use interchain_queries::msg::{
    DelegatorDelegationsResponse, ExecuteMsg, QueryBalanceResponse, QueryMsg, Transfer,
    TransfersResponse,
};
use interchain_queries::types::{
    Balances, QUERY_REGISTERED_QUERY_PATH, QUERY_REGISTERED_QUERY_TRANSACTIONS_RESULT_PATH,
};
use interchain_queries::types::{QueryType, QUERY_REGISTERED_QUERY_RESULT_PATH};
use protobuf::{Message, MessageField};
use stargate::interchain::interchainqueries_genesis::{KVKey, RegisteredQuery};
use stargate::interchain::interchainqueries_query::{
    QueryRegisteredQueryResponse, QueryRegisteredQueryResultResponse,
    QuerySubmittedTransactionsResponse, Transaction,
};

fn build_registered_query_response(
    id: u64,
    keys: Vec<KVKey>,
    query_type: String,
    last_submitted_result_local_height: u64,
) -> QueryRegisteredQueryResponse {
    QueryRegisteredQueryResponse {
        registered_query: MessageField::some(RegisteredQuery {
            id,
            query_type,
            keys,
            transactions_filter: "".to_string(),
            zone_id: "".to_string(),
            connection_id: "".to_string(),
            update_period: 0,
            last_emitted_height: 0,
            last_submitted_result_local_height,
            last_submitted_result_remote_height: 0,
            special_fields: Default::default(),
        }),
        special_fields: Default::default(),
    }
}

use stargate::interchain::interchainqueries_tx::{QueryResult, StorageValue};
use std::num::ParseIntError;

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

// registers an interchain query
fn register_query(
    deps: &mut OwnedDeps<MockStorage, MockApi, WasmMockQuerier>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Vec<KVKey> {
    let reg_msgs = execute(deps.as_mut(), env, info, msg).unwrap();
    for attr in reg_msgs.attributes {
        if attr.key == "kv_keys" && attr.value != "" {
            return attr
                .value
                .split(",")
                .map(|k| {
                    let kv: Vec<String> = k.split("/").map(String::from).collect();
                    KVKey {
                        path: kv[0].clone(),
                        key: decode_hex(kv[1].as_str()).unwrap(),
                        special_fields: Default::default(),
                    }
                })
                .collect::<Vec<KVKey>>();
        }
    }
    return vec![];
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

    let keys = register_query(&mut deps, mock_env(), mock_info("", &[]), msg);

    // protobuf encoded QueryRegisteredQueryResultResponse for balance query
    // TODO: come up with something better than using large base64 string. Good enough for sketch btw
    let balance_response = "Ct4KCtgKCgRiYW5rEhsCFCCpWeDSLiAfcnE38tfEGl3GO5C4dW9zbW8aEAoFdW9zbW8SBzgyNzgxMDQioAoKgggKCmljczIzOmlhdmwSGwIUIKlZ4NIuIB9ycTfy18QaXcY7kLh1b3NtbxrWBwrTBwobAhQgqVng0i4gH3JxN/LXxBpdxjuQuHVvc21vEhAKBXVvc21vEgc4Mjc4MTA0Gg4IARgBIAEqBgAC/pSuBCIsCAESKAIE/pSuBCBFIiq0yrTUzDfOSVNvzeJNwjjTVfLk9CMBHDW3W0KJ5CAiLAgBEigEBv6UrgQg3Lf4OOBjBRVgxhjtj+k5yT3y61uKEkbzGHmA2ubFLPEgIiwIARIoBg7+lK4EIO1MyDP/CRs3YbrmGTO/JoXDbfGjuqMNTg51TsC/rtf4ICIsCAESKAok/pSuBCC9mQSdzwF4IcuhAZGfaPuixawCUsH20rpavJgHDe9BzSAiLggBEgcMPv6UrgQgGiEg/YCmEk9uB7y0eZLzJFauWnmD3wyYxmybek2yrH2pSNkiLggBEgcOYP6UrgQgGiEgs7DIU+g7q4BVxtTu/T3hrMyym9OrTacobvqW9l722UQiLwgBEggQogH+lK4EIBohIKJdq3PxtO69ZeL8M6KOXbnw53U4q2Pr3LW8blf1yYiYIi8IARIIEqIC/pSuBCAaISBHOrSpUPkBG3kqeGWn+BKeVAw9QpGa79A8CoSvcp4S3iIvCAESCBTQBf6UrgQgGiEg2DZEH8UsCN+eaovwKOkW4blZ/iUkI5+PZBwV1k815UEiLQgBEikWkguola4EIEvUT+XYLd8oYtkKJSFnuDAu6X3en/5CUDvdkvFlLMZuICIvCAESCBraG66WrgQgGiEgdzkfs6jauh9c40SBajV1ISW0wFxNvPrMeJBhRtkLugoiLwgBEggcjk2ol64EIBohID01nVjsR0+d6ZKGI0qTN0Bb4KN6gETbmbVzoAzgxGZgIi4IARIqII6hAaqXrgQgnh/RzOjBG6LrzZs2IrDNv5ZBcdP4QrCv3vWRgBts+SwgIjAIARIJIti4AqqXrgQgGiEgirjigGcIwt6hwHldRcufY1KuNv3bWDTcnV1eZeeuDBUiLggBEiokmvEErJeuBCCYuMO4oQkcuovw4N4itiXuhDLpQGx/Qlz76Zxt/zXSnCAiMAgBEgko0JAMrJeuBCAaISCpCwi5NcBiDP/VA4cFX5YZWQUj3DDugdzDITW+vPufKSIuCAESKirQ0BSsl64EIEGc6nNgvc/EFjsdQLfvLy0R6ydzJTnrbGJ46+Ic/n4/ICIwCAESCSyWzC6sl64EIBohIE5IkOe8GGpp4+x/IROqYVNUf+DEwtQRjfhzzY4KiHbKIjAIARIJLoKNWqyXrgQgGiEgE2LuK/SWN21hoBE7UC7AGnZMKFc/55RTKQPGeq7EbwwKmAIKDGljczIzOnNpbXBsZRIEYmFuaxqBAgr+AQoEYmFuaxIgyBQU6gwKLTGPhq/4rBzZTt0/+cv85kfvDDp6QhIWn3oaCQgBGAEgASoBACInCAESAQEaIEk4K+ZIN3GimF4PzvZcTeWI3YbfyoOJuGQIM0w66ew3IiUIARIhAUZQS+gJTRhgjdU0BdK56oZ4+ZmaeCcgOQvEyqqzN77IIicIARIBARog32VptwaxvgTiBPipOTcbHIRq47iRDB306+oYFHYm52oiJwgBEgEBGiAxLaRupWefQxvwWViJsgwyRGkJNZQCVSlr4C5E4a+ZGyInCAESAQEaIDF7KUxIDn3vx616fDBMwkA2rOmhgwR8fFX8fMAKJ8ElGNAL";

    let balance_resp_bytes = base64::decode(balance_response).unwrap();

    deps.querier.add_stargate_response(
        QUERY_REGISTERED_QUERY_RESULT_PATH.to_string(),
        balance_resp_bytes,
    );

    let registered_query = build_registered_query_response(1, keys, QueryType::KV.into(), 987);

    deps.querier.add_stargate_response(
        QUERY_REGISTERED_QUERY_PATH.to_string(),
        registered_query.write_to_bytes().unwrap(),
    );

    let query_balance = QueryMsg::Balance { query_id: 1 };
    let resp: QueryBalanceResponse =
        from_binary(&query(deps.as_ref(), mock_env(), query_balance).unwrap()).unwrap();
    assert_eq!(
        resp,
        QueryBalanceResponse {
            last_submitted_local_height: registered_query
                .registered_query
                .last_submitted_result_local_height,
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
        zone_id: "zone".to_string(),
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
        result: MessageField::some(QueryResult {
            kv_results: vec![
            StorageValue {
                storage_prefix: "staking".to_string(),
                key: decode_hex("311420a959e0d22e201f727137f2d7c41a5dc63b90b8141ab940697a73dd080edafeb538ad408b5cae0264").unwrap(),
                value: base64::decode("Citvc21vMXl6NTRuY3hqOWNzcDd1bjN4bGVkMDNxNnRocnJoeTljenRrZnpzEjJvc21vdmFsb3BlcjFyMnU1cTZ0Nncwd3Nzcms2bDY2bjN0MnEzZHcydXFueTRnajJlMxoZNTE3NzYyODAwMDAwMDAwMDAwMDAwMDAwMA==").unwrap(),
                Proof: Default::default(),
                special_fields: Default::default(),
            },
            StorageValue {
                storage_prefix: "staking".to_string(),
                key: decode_hex("21141ab940697a73dd080edafeb538ad408b5cae0264").unwrap(),
                value: base64::decode("CjJvc21vdmFsb3BlcjFyMnU1cTZ0Nncwd3Nzcms2bDY2bjN0MnEzZHcydXFueTRnajJlMxJDCh0vY29zbW9zLmNyeXB0by5lZDI1NTE5LlB1YktleRIiCiCaZhCbacCetQorko3LfUUJX2UEyX38qBGVri8GyH8lcCADKg0yODQ1ODYyODQwNjQzMh8yODQ1ODYyODQwNjQzMDAwMDAwMDAwMDAwMDAwMDAwOqQCChRzdHJhbmdlbG92ZS12ZW50dXJlcxIQRDBEOEI4MEYxQzVDNzBCNRocaHR0cHM6Ly9zdHJhbmdlbG92ZS52ZW50dXJlcyrbAScuLi5iZWNhdXNlIG9mIHRoZSBhdXRvbWF0ZWQgYW5kIGlycmV2b2NhYmxlIGRlY2lzaW9uLW1ha2luZyBwcm9jZXNzIHdoaWNoIHJ1bGVzIG91dCBodW1hbiBtZWRkbGluZywgdGhlIERvb21zZGF5IG1hY2hpbmUgaXMgdGVycmlmeWluZyBhbmQgc2ltcGxlIHRvIHVuZGVyc3RhbmQgYW5kIGNvbXBsZXRlbHkgY3JlZGlibGUgYW5kIGNvbnZpbmNpbmcuJyAtIERyLiBTdHJhbmdlbG92ZUoAUkwKPAoRNTAwMDAwMDAwMDAwMDAwMDASEzEwMDAwMDAwMDAwMDAwMDAwMDAaEjUwMDAwMDAwMDAwMDAwMDAwMBIMCPetyYYGEKPoosUCWgEx").unwrap(),
                 Proof: Default::default(),
                special_fields: Default::default(),
            },
//=--------------------
            StorageValue {
                storage_prefix: "staking".to_string(),
                key: decode_hex("311420a959e0d22e201f727137f2d7c41a5dc63b90b814cc9598513212c12c36a1775e2233b962e4d5128e").unwrap(),
                value: base64::decode("Citvc21vMXl6NTRuY3hqOWNzcDd1bjN4bGVkMDNxNnRocnJoeTljenRrZnpzEjJvc21vdmFsb3BlcjFlajJlczVmanp0cWpjZDRwd2Ewenl2YWV2dGpkMnk1dzM3d3I5dBoaMjk2MjAyMjEwMDAwMDAwMDAwMDAwMDAwMDA=").unwrap(),
                Proof: Default::default(),
                special_fields: Default::default(),
            },
            StorageValue {
                storage_prefix: "staking".to_string(),
                key: decode_hex("2114cc9598513212c12c36a1775e2233b962e4d5128e").unwrap(),
                value: base64::decode("CjJvc21vdmFsb3BlcjFlajJlczVmanp0cWpjZDRwd2Ewenl2YWV2dGpkMnk1dzM3d3I5dBJDCh0vY29zbW9zLmNyeXB0by5lZDI1NTE5LlB1YktleRIiCiA27dgAuZV/uS9FdsILGWLBw8eYPy+ZEyv1Df2VsrjXDiADKg0zMDU0NDc3MjU5MDM4Mh8zMDU0NDc3MjU5MDM4MDAwMDAwMDAwMDAwMDAwMDAwOoEBChFGcmVucyAo8J+knSzwn6SdKRIQQzQ3ODQ1MjI2NjYyQUY0NxoSaHR0cHM6Ly9mcmVucy5hcm15IhtzZWN1cml0eUBraWRzb250aGVibG9jay54eXoqKVlvdXIgZnJpZW5kbHkgdmFsaWRhdG9yIGZvciBjb3Ntb3MgY2hhaW5zQP3HpQFKCwj3zq6PBhCfrO86UkoKOgoRNTAwMDAwMDAwMDAwMDAwMDASEjUwMDAwMDAwMDAwMDAwMDAwMBoRNTAwMDAwMDAwMDAwMDAwMDASDAjg1rSQBhDkudCDAVoDNTAw").unwrap(),
                Proof: Default::default(),
                special_fields: Default::default(),
            },
                //===================
            StorageValue {
                storage_prefix: "staking".to_string(),
                key: decode_hex("311420a959e0d22e201f727137f2d7c41a5dc63b90b814f8aff987b760a6e4b2b2df48a5a3b7ed2db15006").unwrap(),
                value: base64::decode("Citvc21vMXl6NTRuY3hqOWNzcDd1bjN4bGVkMDNxNnRocnJoeTljenRrZnpzEjJvc21vdmFsb3BlcjFsemhsbnBhaHZ6bndmdjRqbWF5MnRnYWhhNWttejVxeHdtajl3ZRoYMjE5OTIwMDAwMDAwMDAwMDAwMDAwMDAw").unwrap(),
                Proof: Default::default(),
                special_fields: Default::default(),
            },
            StorageValue {
                storage_prefix: "staking".to_string(),
                key: decode_hex("2114f8aff987b760a6e4b2b2df48a5a3b7ed2db15006").unwrap(),
                value: base64::decode("CjJvc21vdmFsb3BlcjFsemhsbnBhaHZ6bndmdjRqbWF5MnRnYWhhNWttejVxeHdtajl3ZRJDCh0vY29zbW9zLmNyeXB0by5lZDI1NTE5LlB1YktleRIiCiBPXCnkQvO+pU6oGbp4ZiJBBZ7RNoLYtXYFOEdpXGH+uSADKg0zMjAxNDM4ODk4NDc2Mh8zMjAxNDM4ODk4NDc2MDAwMDAwMDAwMDAwMDAwMDAwOp8CCgtDaXRhZGVsLm9uZRIQRUJCMDNFQjRCQjRDRkNBNxoTaHR0cHM6Ly9jaXRhZGVsLm9uZSroAUNpdGFkZWwub25lIGlzIGEgbXVsdGktYXNzZXQgbm9uLWN1c3RvZGlhbCBzdGFraW5nIHBsYXRmb3JtIHRoYXQgbGV0cyBhbnlvbmUgYmVjb21lIGEgcGFydCBvZiBkZWNlbnRyYWxpemVkIGluZnJhc3RydWN0dXJlIGFuZCBlYXJuIHBhc3NpdmUgaW5jb21lLiBTdGFrZSB3aXRoIG91ciBub2RlcyBvciBhbnkgb3RoZXIgdmFsaWRhdG9yIGFjcm9zcyBtdWx0aXBsZSBuZXR3b3JrcyBpbiBhIGZldyBjbGlja3NKAFJECjoKETUwMDAwMDAwMDAwMDAwMDAwEhIyMDAwMDAwMDAwMDAwMDAwMDAaETMwMDAwMDAwMDAwMDAwMDAwEgYIkKKzhgZaATE=").unwrap(),
                Proof: Default::default(),
                special_fields: Default::default(),
            }
            ],
            blocks: vec![],
            height: 0,
            revision: 0,
            special_fields: Default::default(),
        }),
        special_fields: Default::default(),
    };

    deps.querier.add_stargate_response(
        QUERY_REGISTERED_QUERY_RESULT_PATH.to_string(),
        delegations_response.write_to_bytes().unwrap(),
    );

    let registered_query = build_registered_query_response(1, keys, QueryType::KV.into(), 987);

    deps.querier.add_stargate_response(
        QUERY_REGISTERED_QUERY_PATH.to_string(),
        registered_query.write_to_bytes().unwrap(),
    );

    let query_delegations = QueryMsg::GetDelegations { query_id: 1 };
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
                    amount: Coin::new(5177628u128, "kek".to_string())
                },
                Delegation {
                    delegator: Addr::unchecked("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"),
                    validator: "osmovaloper1ej2es5fjztqjcd4pwa0zyvaevtjd2y5w37wr9t".to_string(),
                    amount: Coin::new(29620221u128, "kek".to_string())
                },
                Delegation {
                    delegator: Addr::unchecked("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"),
                    validator: "osmovaloper1lzhlnpahvznwfv4jmay2tgaha5kmz5qxwmj9we".to_string(),
                    amount: Coin::new(219920u128, "kek".to_string())
                }
            ],
        }
    )
}

#[test]
fn test_query_transfers() {
    let mut deps = dependencies(&[]);

    let msg = ExecuteMsg::RegisterTransfersQuery {
        zone_id: "zone".to_string(),
        connection_id: "connection".to_string(),
        update_period: 10,
        recipient: "osmo1stlkm9sadmy0kg3tm4l8ucytvl7xwalug85q5a".to_string(),
    };

    register_query(&mut deps, mock_env(), mock_info("", &[]), msg);

    // protobuf encoded QueryRegisteredQueryResultResponse for balance query
    let transfers_response = QuerySubmittedTransactionsResponse {
        transactions: vec![Transaction {
            id: 100,
            height: 4623885u64,
            data: base64::decode("CpABCo0BChwvY29zbW9zLmJhbmsudjFiZXRhMS5Nc2dTZW5kEm0KK29zbW8xa2tnZDd6dno5dnJkOHZ5OWNyMzd5eHdkbXc5NGNydXN3NXBxemcSK29zbW8xc3Rsa205c2FkbXkwa2czdG00bDh1Y3l0dmw3eHdhbHVnODVxNWEaEQoFdW9zbW8SCDUwMDAwMDAwEmUKUQpGCh8vY29zbW9zLmNyeXB0by5zZWNwMjU2azEuUHViS2V5EiMKIQOyiHa4C7rJvMSdoQYF4VcCndaYtlP7Hn8MnpBZ/Rh1ohIECgIIfxiPBxIQCgoKBXVvc21vEgEwEKCNBhpAHBjxUTgIkKwcN/5dMYH+PeCHS9fuvj52W0RvOOUDZ5oeOv/siKybbHzYtjvjQXLu9d36jRWU5KXW9sAWUbbxmw==").unwrap(),
            special_fields: Default::default(),
        }],
        special_fields: Default::default(),
    };

    deps.querier.add_stargate_response(
        QUERY_REGISTERED_QUERY_TRANSACTIONS_RESULT_PATH.to_string(),
        transfers_response.write_to_bytes().unwrap(),
    );

    let registered_query = build_registered_query_response(1, vec![], QueryType::TX.into(), 987);

    deps.querier.add_stargate_response(
        QUERY_REGISTERED_QUERY_PATH.to_string(),
        registered_query.write_to_bytes().unwrap(),
    );

    let query_transfers = QueryMsg::GetTransfers {
        query_id: 1,
        start: 0,
        end: 0,
    };
    let resp: TransfersResponse =
        from_binary(&query(deps.as_ref(), mock_env(), query_transfers).unwrap()).unwrap();
    assert_eq!(
        resp,
        TransfersResponse {
            last_submitted_local_height: registered_query
                .registered_query
                .last_submitted_result_local_height,
            transfers: vec![Transfer {
                tx_id: 100,
                sender: "osmo1kkgd7zvz9vrd8vy9cr37yxwdmw94crusw5pqzg".to_string(),
                amount: vec![Coin::new(50000000u128, "uosmo")],
                height: 4623885,
                recipient: "osmo1stlkm9sadmy0kg3tm4l8ucytvl7xwalug85q5a".to_string(),
            }]
        }
    )
}
