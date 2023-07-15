use std::str::FromStr;

use super::mock_querier::mock_dependencies as dependencies;
use crate::contract::{execute, instantiate, sudo_tx_query_result, INTERCHAIN_ACCOUNT_ID};
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::query_helpers::nft_transfer_filter;
use crate::state::{NftTransfer, INTERCHAIN_ACCOUNTS, SENDER_TXS, CONFIG, get_ica};
use cosmos_sdk_proto::cosmos::tx::v1beta1::{TxBody, TxRaw};
use cosmos_sdk_proto::cosmwasm::wasm::v1::MsgExecuteContract;
use cosmos_sdk_proto::traits::MessageExt;
use cosmos_sdk_proto::Any;
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{to_binary, Addr, Binary, Coin, Uint128};

use cw721::Cw721ExecuteMsg;
use neutron_sdk::bindings::query::{
    QueryRegisteredQueryResponse,
};
use neutron_sdk::bindings::types::{Height, KVKey, RegisteredQuery};
use neutron_sdk::interchain_queries::types::QueryType;
use neutron_sdk::interchain_txs::helpers::get_port_id;
use prost::Message;
use schemars::_serde_json::to_string;

enum QueryParam {
    Keys(Vec<KVKey>),
    TransactionsFilter(String),
}

fn build_msg_payload(
    recipient: String,
    sender: String,
    contract_address: String,
    token_id: String,
) -> Binary {
    let msg = Cw721ExecuteMsg::TransferNft {
        recipient,
        token_id,
    };

    let contract_exec_msg = MsgExecuteContract {
        sender,
        contract: contract_address,
        msg: to_binary(&msg).unwrap().into(),
        funds: vec![],
    };
    let mut buf: Vec<u8> = vec![];
    contract_exec_msg.encode(&mut buf).unwrap();

    let body = TxBody {
        messages: vec![Any {
            type_url: "/cosmwasm.wasm.v1.MsgExecuteContract".to_string(),
            value: buf,
        }],
        memo: "".to_string(),
        timeout_height: 4,
        extension_options: vec![],
        non_critical_extension_options: vec![],
    };

    let tx_raw = TxRaw {
        body_bytes: body.to_bytes().unwrap(),
        auth_info_bytes: Binary::default().to_vec(),
        signatures: vec![],
    };

    let mut tx_raw_buf = vec![] as Vec<u8>;
    tx_raw.encode(&mut tx_raw_buf).unwrap();

    tx_raw_buf.into()
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

const SENDER: &str = "stars10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf";
const TOKEN_ID: &str = "42";
const ICA_ADDRESS: &str = "stars10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf";
const CONTRACT_ADDRESS: &str = "stars1xv9tklw7d82sezh9haa573wufgy59vmwe6xxe5";
const CONNECTION_ID: &str = "connection-0";
#[test]
fn test_sudo_tx_query_result_callback() {
    let mut deps = dependencies(&[]);
    let env = mock_env();

    let query_id: u64 = 1u64;
    let height: u64 = 1u64;
    let msg = ExecuteMsg::RegisterTransferNftQuery {
        sender: SENDER.to_string(),
        token_id: TOKEN_ID.to_string(),
        min_height: 1000,
    };
    instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("", &[]),
        InstantiateMsg {
            connection_id: CONNECTION_ID.to_string(),
            contract_addr: CONTRACT_ADDRESS.to_string(),
        },
    )
    .unwrap();

    INTERCHAIN_ACCOUNTS
        .save(
            &mut deps.storage,
            get_port_id(
                env.clone().contract.address,
                Addr::unchecked(INTERCHAIN_ACCOUNT_ID),
            )
            ,
            &Some(("".to_string(), "".to_string())),
        )
        .unwrap();

    execute(deps.as_mut(), env.clone(), mock_info("", &[]), msg).unwrap();
    let config = CONFIG.load(&deps.storage).unwrap();
    
    let registered_query = build_registered_query_response(
        1,
        QueryParam::TransactionsFilter(
            to_string(&nft_transfer_filter(
                1000,
                ICA_ADDRESS.to_string(),
                CONTRACT_ADDRESS.to_string(),
                SENDER.to_string(),
                TOKEN_ID.to_string(),
            ))
            .unwrap(),
        ),
        QueryType::TX,
        0,
    );
    deps.querier.add_registred_queries(1, registered_query);

    // simulate neutron's SudoTxQueryResult call with the following payload:
    // Build the payload that would be received from stargaze
    let data = build_msg_payload(
        ICA_ADDRESS.to_string(),
        SENDER.to_string(),
        CONTRACT_ADDRESS.to_string(),
        TOKEN_ID.to_string(),
    );

    // Recieve and process the payload on neutron side
    sudo_tx_query_result(
        deps.as_mut(),
        env,
        query_id,
        Height {
            revision_number: 0,
            revision_height: height,
        },
        data,
    )
    .unwrap();

    // ensure the callback has worked and contract's state has changed
    let txs = SENDER_TXS.load(&deps.storage, SENDER).unwrap();
    assert_eq!(
        txs,
        Vec::from([NftTransfer {
            contract_address: CONTRACT_ADDRESS.to_string(),
            token_id: TOKEN_ID.to_string(),
            sender: SENDER.to_string(),
        }])
    );
}

#[test]
fn test_filter_output() {

    let min_height: u64 = 1000;
    let recipient: String = "stars1xv9tklw7d82sezh9haa573wufgy59vmwe6xxe5".to_string();
    let sender: String = "stars10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf".to_string();
    let contract_address: String = "stars1xv9tklw7d82sezh9haa573wufgy59vmwe6xxe5".to_string();
    let token_id: String = "42".to_string();

    let filter = nft_transfer_filter(min_height, recipient, sender, contract_address, token_id);

    // filter.into_iter().for_each(|f| {
    //     println!("{}", f.into());
    // });
}