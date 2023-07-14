use crate::query_helpers::verify_query;
use crate::ibc::msg_with_sudo_callback;
use crate::state::SudoPayload;
use cosmos_sdk_proto::traits::MessageExt;
use crate::state::get_ica;
use cw0::must_pay;
use crate::ibc::min_ntrn_ibc_fee;
use neutron_sdk::query::min_ibc_fee::query_min_ibc_fee;
use neutron_sdk::bindings::types::ProtobufAny;
use neutron_sdk::interchain_txs::helpers::get_port_id;
use crate::state::MINTED_TOKENS;
use crate::mint::format_token_denom;
use crate::mint::THRESHOLD_BURN_AMOUNT;
use neutron_sdk::NeutronError;
use crate::mint::any_addr_to_neutron;
use crate::mint::mint_native_receipt;
use crate::ibc::execute_register_ica;
use crate::state::Config;
use crate::state::CONFIG;

use cosmos_sdk_proto::cosmos::tx::v1beta1::{TxBody, TxRaw};
use cosmos_sdk_proto::traits::MessageExt;
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Uint128, WasmMsg, from_binary, Empty,
};
use cw2::set_contract_version;
use prost::Message as ProstMessage;

use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, TransferNftResponse,
};
use crate::query_helpers::{new_register_transfer_nft_query_msg, WASM_EXECUTE_MSG_TYPE};
use crate::state::{NftTransfer, RECIPIENT_TXS, TRANSFERS};
use neutron_sdk::bindings::msg::{NeutronMsg, MsgExecuteContractResponse};
use neutron_sdk::bindings::query::{NeutronQuery, QueryRegisteredQueryResponse};
use neutron_sdk::bindings::types::{Height};
use neutron_sdk::interchain_queries::{
    get_registered_query,
};
use neutron_sdk::sudo::msg::SudoMsg;
use neutron_sdk::{NeutronResult};

use neutron_sdk::interchain_queries::types::{
    TransactionFilterItem,
};
use cosmos_sdk_proto::cosmwasm::wasm::v1::{
    MsgExecuteContract
};

use serde_json_wasm;
use cw721::Cw721ExecuteMsg;


// Default timeout for IbcTransfer is 10000000 blocks
const DEFAULT_TIMEOUT_HEIGHT: u64 = 10000000;
const DEFAULT_TIMEOUT_SECONDS: u64 = 60 * 60 * 24 * 7 * 2;


/// defines the incoming transfers limit to make a case of failed callback possible.
const MAX_ALLOWED_TRANSFER: u64 = 20000;
const MAX_ALLOWED_MESSAGES: usize = 20;

const CONTRACT_NAME: &str = concat!("crates.io:neutron-sdk__", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INTERCHAIN_ACCOUNT_ID: &str = "bad-kids-account-id";

#[cfg_attr(feature="interface", cw_orch::interface_entry_point)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    deps.api.debug("WASMDEBUG: instantiate");
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config{
        nft_contract_address: msg.contract_addr
    };
    CONFIG.save(deps.storage, &config)?;

    execute_register_ica(deps, env, msg.connection_id, INTERCHAIN_ACCOUNT_ID.to_string())?;


    Ok(Response::default())
}

#[cfg_attr(feature="interface", cw_orch::interface_entry_point)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    match msg {
        ExecuteMsg::RegisterTransferNftQuery { connection_id, update_period, min_height, recipient, sender, contract_address, token_id } => {
            register_transfer_nft_query(connection_id, update_period, min_height, recipient, sender, contract_address, token_id)
        }
        // todo: add NFT ownership query
        ExecuteMsg::RemoveInterchainQuery { query_id } => remove_interchain_query(query_id),
        ExecuteMsg::UnlockNft {
            token_id,
            destination,
        } => execute_unlock_nft(
            deps,
            env,
            info,
            token_id,
            destination,
        ),
        ExecuteMsg::MintNft {token_id} => execute_mint_nft(
            deps,
            env,
            token_id
        ),
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

fn execute_mint_nft(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    token_id: String,
) -> NeutronResult<Response<NeutronMsg>> {
    
    // We need to verify the query
    let sender_addr = verify_query(token_id.clone())?;

    // Now that we have the address, we can mint our token to the recipient which validates their ownership of the bad kid
    let addr = any_addr_to_neutron(deps.as_ref(), sender_addr)?;
    mint_native_receipt(deps, env, token_id, addr)
}

fn execute_unlock_nft(
    mut deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    token_id: String,
    destination_addr: String
) -> NeutronResult<Response<NeutronMsg>> {
    
    let config = CONFIG.load(deps.storage)?;

    // We need to make sure, that the client pays enough tokens to unlock their tokens
    let denom_count = MINTED_TOKENS.load(deps.storage, token_id.clone())?;
    let denom = format_token_denom(env.clone(), token_id.clone(), denom_count);

    let amount = must_pay(&info, &denom).map_err(|e| NeutronError::Std(StdError::generic_err(e.to_string())))?;
    if amount < THRESHOLD_BURN_AMOUNT.into(){
        return Err(NeutronError::Std(StdError::generic_err(format!("You need to pay at least{} to unlock your token {}", THRESHOLD_BURN_AMOUNT, token_id))))
    }

    // contract must pay for relaying of acknowledgements
    // See more info here: https://docs.neutron.org/neutron/feerefunder/overview
    let fee = min_ntrn_ibc_fee(query_min_ibc_fee(deps.as_ref())?.min_fee);
    let (account_addr, connection_id) = get_ica(deps.as_ref(), &env, INTERCHAIN_ACCOUNT_ID)?;

    let unlock_message = MsgExecuteContract {
        contract: config.nft_contract_address,
        msg: to_binary(&Cw721ExecuteMsg::TransferNft{
            recipient: destination_addr,
            token_id
        })?.to_vec(),
        funds: vec![],
        sender: account_addr
    };

    let mut buf = Vec::new();
    buf.reserve(unlock_message.encoded_len());

    if let Err(e) = unlock_message.encode(&mut buf) {
        return Err(NeutronError::Std(StdError::generic_err(format!(
            "Encode error: {}",
            e
        ))));
    }

    let any_msg = unlock_message.to_any() // Using the to_any feature to not mess it up
        .map_err(|e| NeutronError::Std(StdError::generic_err(e.to_string())))?;

    let cosmos_msg = NeutronMsg::submit_tx(
        connection_id,
        INTERCHAIN_ACCOUNT_ID.to_string(),
        vec![ProtobufAny{
            value: Binary(any_msg.value),
            type_url: any_msg.type_url
        }],
        "".to_string(),
        DEFAULT_TIMEOUT_SECONDS,
        fee,
    );

    // We use a submessage here because we need the process message reply to save
    // the outgoing IBC packet identifier for later.
    let submsg = msg_with_sudo_callback(
        deps.branch(),
        cosmos_msg,
        SudoPayload {
            port_id: get_port_id(env.contract.address.as_str(), INTERCHAIN_ACCOUNT_ID),
            message: "message".to_string(),
        },
    )?;

    Ok(Response::default().add_submessages(vec![submsg]))
}

#[cfg_attr(feature="interface", cw_orch::interface_entry_point)]
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

#[cfg_attr(feature="interface", cw_orch::interface_entry_point)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    deps.api.debug("WASMDEBUG: migrate");
    Ok(Response::default())
}

#[cfg_attr(feature="interface", cw_orch::interface_entry_point)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut<NeutronQuery>, env: Env, msg: SudoMsg) -> NeutronResult<Response> {
    match msg {
        // For handling tx query result
        SudoMsg::TxQueryResult {
            query_id,
            height,
            data,
        } => sudo_tx_query_result(deps, env, query_id, height, data),

        // For handling kv query result
        SudoMsg::KVQueryResult { query_id } => panic!(),
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
            let msg = body.messages.get(0).unwrap();

            if let WasmMsg::Execute { contract_addr, msg, funds } = from_binary(&msg.to_bytes().into())? {
                if let cw721_base::ExecuteMsg::TransferNft { recipient, token_id } = from_binary(&msg)? 
                { 
                    let sender = deps.api.addr_validate(recipient.as_str())?;
                    let contract_addr = deps.api.addr_validate(contract_addr.as_str())?;
                    let transfer_nft = NftTransfer {
                        sender: todo!(),
                        contract_address: contract_addr.to_string(),
                        token_id,
                    };

                    return Ok(Response::new().add_attribute("transfer_nft", transfer_nft.into()));
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
        let transfer_msg: cw721_base::ExecuteMsg<Empty, Empty> = from_binary(msg.value.as_slice());
        // TODO HOW can i parse this message data into the right interface?


        transfers.push(NftTransfer {
            sender: transfer_msg,
            contract_address: todo!(),
            token_id: todo!(),
        })
    }
    Ok(transfers)

}
