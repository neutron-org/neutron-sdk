use cosmwasm_std::{Deps, CustomQuery};
use crate::state::TOKEN_ID_SENDER;
use neutron_sdk::{
    bindings::msg::NeutronMsg,
    interchain_queries::types::{
        QueryPayload, TransactionFilterItem, TransactionFilterOp, TransactionFilterValue,
    },
    NeutronResult,
};

// [{"field": "{eventType}.{attributeKey}", "val": "{attributeValue}", "op": "gte"}, ...]
pub const HEIGHT_FIELD: &str = "tx.height";
pub const WASM_EXECUTE_MSG_TYPE: &str = "/cosmwasm.wasm.v1.MsgExecuteContract";

/// Creates a message to register an Interchain Query to get transfer events to a recipient on a remote chain.
///
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **recipient** is an address of an account on remote chain for which you want to get list of transfer transactions;
/// * **update_period** is used to say how often the query must be updated.
/// * **min_height** is used to set min height for query (by default = 0).
pub fn new_register_transfer_nft_query_msg(
    connection_id: String,
    update_period: u64,
    min_height: u64,
    recipient: String,
    sender: String,
    contract_address: String,
    token_id: String,
) -> NeutronResult<NeutronMsg> {
    let query_data = nft_transfer_filter(min_height, recipient, sender, contract_address, token_id);

    // [{"field": "{eventType}.{attributeKey}", "val": "{attributeValue}", "op": "gte"}, ...]
    NeutronMsg::register_interchain_query(
        QueryPayload::TX(query_data),
        connection_id,
        update_period,
    )
}

pub fn nft_transfer_filter(
    min_height: u64,
    recipient: String,
    sender: String,
    contract_address: String,
    token_id: String,
) -> Vec<TransactionFilterItem> {
    let query_data = vec![
        TransactionFilterItem {
            field: HEIGHT_FIELD.to_string(),
            op: TransactionFilterOp::Gte,
            value: TransactionFilterValue::Int(min_height),
        },
        TransactionFilterItem {
            field: "wasm.recipient".to_string(),
            op: TransactionFilterOp::Eq,
            value: TransactionFilterValue::String(recipient),
        },
        TransactionFilterItem {
            field: "wasm.action".to_string(),
            op: TransactionFilterOp::Eq,
            value: TransactionFilterValue::String("transfer_nft".to_string()),
        },
        TransactionFilterItem {
            field: "wasm.sender".to_string(),
            op: TransactionFilterOp::Eq,
            value: TransactionFilterValue::String(sender),
        },
        TransactionFilterItem {
            field: "wasm._contract_address".to_string(),
            op: TransactionFilterOp::Eq,
            value: TransactionFilterValue::String(contract_address),
        },
        TransactionFilterItem {
            field: "wasm.token_id".to_string(),
            op: TransactionFilterOp::Eq,
            value: TransactionFilterValue::String(token_id),
        },
    ];
    query_data
}

pub fn verify_query<T: CustomQuery>(deps: Deps<T>, token_id: String) -> NeutronResult<String> {
    // We load the nft transfer associated with the token_id
    let sender = TOKEN_ID_SENDER.load(deps.storage, token_id)?;
    // We verify the nft is currently owned by the ica_account (TODO)

    Ok(sender.to_string())
}