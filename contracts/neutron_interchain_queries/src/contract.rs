use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{TxBody, TxRaw};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Uint128, WasmMsg, from_binary,
};
use cw2::set_contract_version;
use prost::Message as ProstMessage;

use crate::msg::{ExecuteMsg, GetRecipientTxsResponse, InstantiateMsg, MigrateMsg, QueryMsg, TransferNftResponse,
};
use crate::query_helpers::{new_register_transfer_nft_query_msg, WASM_EXECUTE_MSG_TYPE};
use crate::state::{NftTransfer, RECIPIENT_TXS, TRANSFERS};
use neutron_sdk::bindings::msg::{NeutronMsg, MsgExecuteContract};
use neutron_sdk::bindings::query::{NeutronQuery, QueryRegisteredQueryResponse};
use neutron_sdk::bindings::types::{Height, KVKey};
use neutron_sdk::interchain_queries::{
    check_query_type, get_registered_query, query_kv_result,
    v045::types::{COSMOS_SDK_TRANSFER_MSG_URL, RECIPIENT_FIELD},
};
use neutron_sdk::sudo::msg::SudoMsg;
use neutron_sdk::{NeutronError, NeutronResult};

use neutron_sdk::interchain_queries::types::{
    QueryType, TransactionFilterItem, TransactionFilterOp, TransactionFilterValue,
};
use serde_json_wasm;

/// defines the incoming transfers limit to make a case of failed callback possible.
const MAX_ALLOWED_TRANSFER: u64 = 20000;
const MAX_ALLOWED_MESSAGES: usize = 20;

const CONTRACT_NAME: &str = concat!("crates.io:neutron-sdk__", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> NeutronResult<Response> {
    deps.api.debug("WASMDEBUG: instantiate");
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut<NeutronQuery>,
    _env: Env,
    _: MessageInfo,
    msg: ExecuteMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    match msg {
        ExecuteMsg::RegisterTransferNftQuery { connection_id, update_period, min_height, recipient, sender, contract_address, token_id } => {
            register_transfer_nft_query(connection_id, update_period, min_height, recipient, sender, contract_address, token_id)
        }
        // todo: add NFT ownership query
        ExecuteMsg::RemoveInterchainQuery { query_id } => remove_interchain_query(query_id),
    }
}

pub fn register_transfer_nft_query(
    connection_id: String,
    update_period: u64,
    min_height: u64,
    recipient: String,
    sender: String,
    contract_address: String,
    token_id: String,
) -> NeutronResult<Response<NeutronMsg>> {
    let query_msg = new_register_transfer_nft_query_msg(
        connection_id,
        update_period,
        min_height,
        recipient,
        sender,
        contract_address,
        token_id,
    )?;
    Ok(Response::new().add_message(query_msg))
}

pub fn remove_interchain_query(query_id: u64) -> NeutronResult<Response<NeutronMsg>> {
    let remove_msg = NeutronMsg::remove_interchain_query(query_id);
    Ok(Response::new().add_message(remove_msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<NeutronQuery>, env: Env, msg: QueryMsg) -> NeutronResult<Binary> {
    match msg {
        QueryMsg::TransferNft { query_id } => {
            Ok(to_binary(&query_transfer_nft(deps, env, query_id)?)?)
        }
        QueryMsg::GetRegisteredQuery { query_id } => {
            Ok(to_binary(&get_registered_query(deps, query_id)?)?)
        }
    }
}

fn query_transfer_nft(deps: Deps<NeutronQuery>, env: Env, query_id: u64) -> NeutronResult<TransferNftResponse> {
    todo!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    deps.api.debug("WASMDEBUG: migrate");
    Ok(Response::default())
}

#[entry_point]
pub fn sudo(deps: DepsMut<NeutronQuery>, env: Env, msg: SudoMsg) -> NeutronResult<Response> {
    match msg {
        // For handling tx query result
        SudoMsg::TxQueryResult {
            query_id,
            height,
            data,
        } => sudo_tx_query_result(deps, env, query_id, height, data),

        // For handling kv query result
        SudoMsg::KVQueryResult { query_id } => sudo_kv_query_result(deps, env, query_id),
        _ => Ok(Response::default()),
    }
}

/// sudo_check_tx_query_result is an example callback for transaction query results that stores the
/// deposits received as a result on the registered query in the contract's state.
pub fn sudo_tx_query_result(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    query_id: u64,
    _height: Height,
    data: Binary,
) -> NeutronResult<Response> {
    // Decode the transaction data
    let tx: TxRaw = TxRaw::decode(data.as_slice())?;
    let body: TxBody = TxBody::decode(tx.body_bytes.as_slice())?;

    // Get the registered query by ID and retrieve the raw query string
    let registered_query: QueryRegisteredQueryResponse =
        get_registered_query(deps.as_ref(), query_id)?;
    let transactions_filter = registered_query.registered_query.transactions_filter;

    #[allow(clippy::match_single_binding)]
    // Depending of the query type, check the transaction data to see whether is satisfies
    // the original query. If you don't write specific checks for a transaction query type,
    // all submitted results will be treated as valid.
    //
    // TODO: come up with solution to determine transactions filter type
    match registered_query.registered_query.query_type {
        _ => {
            // TODO: implements
            let msg: WasmMsg = body.messages.get(0).unwrap();
            if let WasmMsg::Execute { contract_addr, msg, funds } = msg {
                if let cw721_base::ExecuteMsg::TransferNft { recipient, token_id } = from_binary(msg)? 
                {
                    
                }

            }

            

            // For transfer queries, query data looks like `[{"field:"transfer.recipient", "op":"eq", "value":"some_address"}]`
            let query_data: Vec<TransactionFilterItem> =
                serde_json_wasm::from_str(transactions_filter.as_str())?;

            // let recipient = query_data
            //     .iter()
            //     .find(|x| x.field == RECIPIENT_FIELD && x.op == TransactionFilterOp::Eq)
            //     .map(|x| match &x.value {
            //         TransactionFilterValue::String(v) => v.as_str(),
            //         _ => "",
            //     })
            //     .unwrap_or("");

            // let deposits = recipient_deposits_from_tx_body(body, recipient)?;
            // // If we didn't find a Send message with the correct recipient, return an error, and
            // // this query result will be rejected by Neutron: no data will be saved to state.
            // if deposits.is_empty() {
            //     return Err(NeutronError::Std(StdError::generic_err(
            //         "failed to find a matching transaction message",
            //     )));
            // }

            // let mut stored_transfers: u64 = TRANSFERS.load(deps.storage).unwrap_or_default();
            // stored_transfers += deposits.len() as u64;
            // TRANSFERS.save(deps.storage, &stored_transfers)?;

            // check_deposits_size(&deposits)?;
            // let mut stored_deposits: Vec<NftTransfer> = RECIPIENT_TXS
            //     .load(deps.storage, recipient)
            //     .unwrap_or_default();
            // stored_deposits.extend(deposits);
            // RECIPIENT_TXS.save(deps.storage, recipient, &stored_deposits)?;
            Ok(Response::new())
        }
    }
}

/// parses tx body and retrieves transactions to the given recipient.
fn nft_transfers_from_tx_body(
    tx_body: TxBody,
    recipient: &str,
) -> NeutronResult<Vec<NftTransfer>> {
    let mut transfers: Vec<NftTransfer> = vec![];
    // Only handle up to MAX_ALLOWED_MESSAGES messages, everything else
    // will be ignored to prevent 'out of gas' conditions.
    // Note: in real contracts you will have to somehow save ignored
    // data in order to handle it later.

    for msg in tx_body.messages.iter().take(MAX_ALLOWED_MESSAGES) {
        // Skip all messages in this transaction that are not Send messages.
        if msg.type_url != *WASM_EXECUTE_MSG_TYPE.to_string() {
            continue;
        }

        // Parse a Send message and check that it has the required recipient.
        let transfer_msg: MsgExecuteContract = MsgExecuteContract::(msg.value.as_slice())?;
        if transfer_msg.to_address == recipient {
            for coin in transfer_msg.amount {
                transfers.push(NftTransfer {
                    sender: transfer_msg.from_address.clone(),
                    amount: coin.amount.clone(),
                    denom: coin.denom,
                    recipient: recipient.to_string(),
                });
            }
        }
    }
    Ok(transfers)
}

// checks whether there are deposits that are greater then MAX_ALLOWED_TRANSFER.
fn check_deposits_size(deposits: &Vec<Transfer>) -> StdResult<()> {
    for deposit in deposits {
        match deposit.amount.parse::<u64>() {
            Ok(amount) => {
                if amount > MAX_ALLOWED_TRANSFER {
                    return Err(StdError::generic_err(format!(
                        "maximum allowed transfer is {}",
                        MAX_ALLOWED_TRANSFER
                    )));
                };
            }
            Err(error) => {
                return Err(StdError::generic_err(format!(
                    "failed to cast transfer amount to u64: {}",
                    error
                )));
            }
        };
    }
    Ok(())
}

/// sudo_kv_query_result is the contract's callback for KV query results. Note that only the query
/// id is provided, so you need to read the query result from the state.
pub fn sudo_kv_query_result(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    query_id: u64,
) -> NeutronResult<Response> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_kv_query_result received; query_id: {:?}",
            query_id,
        )
        .as_str(),
    );

    // TODO: provide an actual example. Currently to many things are going to change
    // after @pro0n00gler's PRs to implement this.

    Ok(Response::default())
}
