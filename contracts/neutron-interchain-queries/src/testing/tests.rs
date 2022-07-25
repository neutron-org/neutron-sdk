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
use interchain_queries::msg::{ExecuteMsg, QueryMsg};
use interchain_queries::types::{
    DelegatorDelegationsResponse, QueryBalanceResponse, Transfer, TransfersResponse,
    QUERY_REGISTERED_QUERY_PATH, QUERY_REGISTERED_QUERY_TRANSACTIONS_RESULT_PATH,
};
use interchain_queries::types::{QueryType, QUERY_REGISTERED_QUERY_RESULT_PATH};
use protobuf::{Message, MessageField};
use stargate::interchain::interchainqueries_genesis::{KVKey, RegisteredQuery};
use stargate::interchain::interchainqueries_query::{
    QueryRegisteredQueryResponse, QuerySubmittedTransactionsResponse, Transaction,
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
        validators: vec![
            "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3".to_string(),
            "osmovaloper1ej2es5fjztqjcd4pwa0zyvaevtjd2y5w37wr9t".to_string(),
            "osmovaloper1lzhlnpahvznwfv4jmay2tgaha5kmz5qxwmj9we".to_string(),
        ],
    };

    let keys = register_query(&mut deps, mock_env(), mock_info("", &[]), msg);

    // protobuf encoded QueryRegisteredQueryResultResponse for balance query
    // TODO: come up with something better than using large base64 string. Good enough for sketch btw
    let delegations_response = "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";
    let delegations_resp_bytes = base64::decode(delegations_response).unwrap();

    deps.querier.add_stargate_response(
        QUERY_REGISTERED_QUERY_RESULT_PATH.to_string(),
        delegations_resp_bytes,
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
