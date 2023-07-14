use std::str::FromStr;

use super::mock_querier::mock_dependencies as dependencies;
use crate::contract::{execute, query, sudo_tx_query_result};
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::query_helpers::nft_transfer_filter;
use crate::state::{NftTransfer, SENDER_TXS};
use cosmwasm_std::testing::{mock_env, mock_info, MockApi, MockStorage};
use cosmwasm_std::{Binary, Coin, Uint128};
use neutron_sdk::bindings::query::{
    NeutronQuery, QueryRegisteredQueryResponse, QueryRegisteredQueryResultResponse,
};
use neutron_sdk::bindings::types::{
    decode_hex, Height, InterchainQueryResult, KVKey, KVKeys, RegisteredQuery, StorageValue,
};
use neutron_sdk::interchain_queries::types::QueryType;
use schemars::_serde_json::to_string;

enum QueryParam {
    Keys(Vec<KVKey>),
    TransactionsFilter(String),
}

fn build_registered_query_response(
    id: u64,
    param: QueryParam,
    query_type: QueryType,
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
            last_submitted_result_remote_height: Height {
                revision_number: 0,
                revision_height: 0,
            },
            deposit: Vec::from([Coin {
                denom: "stake".to_string(),
                amount: Uint128::from_str("100").unwrap(),
            }]),
            submit_timeout: 0,
            registered_at_height: 0,
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

#[test]
fn test_sudo_tx_query_result_callback() {
    let mut deps = dependencies(&[]);
    let env = mock_env();
    let watched_addr: String = "neutron1fj6yqrkpw6fmp7f7jhj57dujfpwal4m25dafzx".to_string();
    let query_id: u64 = 1u64;
    let height: u64 = 1u64;
    let msg = ExecuteMsg::RegisterTransferNftQuery {
        connection_id: "connection".to_string(),
        update_period: 1u64,
        recipient: watched_addr.clone(),
        sender: "sender".to_string(),
        contract_address: "contract".to_string(),
        token_id: "42".to_string(),
        min_height: 1000,
    };
    execute(deps.as_mut(), env.clone(), mock_info("", &[]), msg).unwrap();
    let registered_query = build_registered_query_response(
        1,
        QueryParam::TransactionsFilter(
            to_string(&nft_transfer_filter(
                1000,
                "contract".to_string(),
                "sender".to_string(),
                "sender".to_string(),
                "42".to_string(),
            ))
            .unwrap(),
        ),
        QueryType::TX,
        0,
    );
    deps.querier.add_registred_queries(1, registered_query);

    // simulate neutron's SudoTxQueryResult call with the following payload:
    // a sending from neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf to watched_addr of 10000 stake
    let data: Binary = Binary::from(base64::decode("CpMBCpABChwvY29zbW9zLmJhbmsudjFiZXRhMS5Nc2dTZW5kEnAKLm5ldXRyb24xMGg5c3RjNXY2bnRnZXlnZjV4Zjk0NW5qcXE1aDMycjU0cmY3a2YSLm5ldXRyb24xZmo2eXFya3B3NmZtcDdmN2poajU3ZHVqZnB3YWw0bTI1ZGFmengaDgoFc3Rha2USBTEwMDAwEmcKUApGCh8vY29zbW9zLmNyeXB0by5zZWNwMjU2azEuUHViS2V5EiMKIQJPYibh+Zef13ZkulPqI27rV5xswZ0H/vh1Tnymp1RHPhIECgIIARgAEhMKDQoFc3Rha2USBDEwMDAQwJoMGkAIiXNJXmA57KhyaWpKcLLr3602A5+hlvv/b4PgcDDm9y0qikC+biNZXin1dEMpHOvX9DwOWJ9utv6EKljiSyfT").unwrap());
    sudo_tx_query_result(
        deps.as_mut(),
        env.clone(),
        query_id,
        Height {
            revision_number: 0,
            revision_height: height,
        },
        data,
    )
    .unwrap();

    // ensure the callback has worked and contract's state has changed
    let txs = SENDER_TXS.load(&deps.storage, &watched_addr).unwrap();
    assert_eq!(
        txs,
        Vec::from([NftTransfer {
            contract_address: "contract".to_string(),
            token_id: "42".to_string(),
            sender: "neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf".to_string(),
        }])
    );
}
