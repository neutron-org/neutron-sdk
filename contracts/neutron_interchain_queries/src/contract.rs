use crate::query_helpers::new_register_nft_owned_query_msg;
use crate::reply::SUDO_PAYLOAD_REPLY_ID;
use crate::state::TOKEN_ID_SENDER;
use crate::state::TOKEN_INFOS;
use crate::sudo::prepare_sudo_payload;
use crate::reply::QUERY_REGISTER_REPLY_ID;

use crate::ibc::execute_register_ica;
use crate::ibc::min_ntrn_ibc_fee;
use crate::ibc::msg_with_sudo_callback;
use crate::mint::any_addr_to_neutron;
use crate::mint::format_token_denom;
use crate::mint::mint_native_receipt;
use crate::mint::THRESHOLD_BURN_AMOUNT;
use crate::query_helpers::verify_query;

use crate::state::get_ica;
use crate::state::Config;
use crate::state::SudoPayload;
use crate::state::CACHED_TOKEN_ID;
use crate::state::CONFIG;
use crate::state::MINTED_TOKENS;
use crate::state::SENDER_TXS;
use crate::state::TOKEN_ID_QUERY_PAIRS;
use crate::sudo::sudo_error;
use crate::sudo::sudo_open_ack;
use crate::sudo::sudo_response;
use crate::sudo::sudo_timeout;

use cosmos_sdk_proto::traits::MessageExt;

use cosmwasm_std::Addr;
use cosmwasm_std::BankMsg;
use cosmwasm_std::Empty;
use cosmwasm_std::SubMsg;
use cosmwasm_std::coin;
use cosmwasm_std::from_slice;
use cosmwasm_std::{Reply};

use cw0::must_pay;
use cw721_base::state::TokenInfo;
use cw_storage_plus::KeyDeserialize;
use neutron_sdk::bindings::msg::MsgRegisterInterchainQueryResponse;
use neutron_sdk::bindings::msg::MsgSubmitTxResponse;
use neutron_sdk::bindings::types::ProtobufAny;
use neutron_sdk::bindings::types::StorageValue;
use neutron_sdk::interchain_queries::queries::get_raw_interchain_query_result;
use neutron_sdk::interchain_queries::query_kv_result;
use neutron_sdk::interchain_queries::types::KVReconstruct;
use neutron_sdk::interchain_txs::helpers::get_port_id;
use neutron_sdk::query::min_ibc_fee::query_min_ibc_fee;
use neutron_sdk::NeutronError;


use cosmos_sdk_proto::cosmos::tx::v1beta1::{TxBody, TxRaw};

use cosmwasm_std::{
    entry_point, from_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult,
};
use cw2::set_contract_version;
use prost::Message as ProstMessage;

use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, NftTransfersResponse, QueryMsg};
use crate::query_helpers::new_register_transfer_nft_query_msg;
use crate::state::{NftTransfer, TRANSFERS};
use neutron_sdk::bindings::msg::NeutronMsg;
use neutron_sdk::bindings::query::{NeutronQuery, QueryRegisteredQueryResponse};
use neutron_sdk::bindings::types::Height;
use neutron_sdk::interchain_queries::get_registered_query;
use neutron_sdk::sudo::msg::SudoMsg;
use neutron_sdk::NeutronResult;

use cosmos_sdk_proto::cosmwasm::wasm::v1::MsgExecuteContract;

use cw721::Cw721ExecuteMsg;
use serde_json_wasm;

// Default timeout for IbcTransfer is 10000000 blocks
const DEFAULT_TIMEOUT_HEIGHT: u64 = 10000000;
const DEFAULT_TIMEOUT_SECONDS: u64 = 60 * 60 * 24 * 7 * 2;

/// defines the incoming transfers limit to make a case of failed callback possible.
const MAX_ALLOWED_TRANSFER: u64 = 20000;
const MAX_ALLOWED_MESSAGES: usize = 20;

const CONTRACT_NAME: &str = concat!("crates.io:neutron-sdk__", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const INTERCHAIN_ACCOUNT_ID: &str = "hub";

#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    deps.api.debug("WASMDEBUG: instantiate");
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        nft_contract_address: msg.contract_addr,
        connection_id: msg.connection_id,
        update_period: 10,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    match msg {
        ExecuteMsg::RegisterICA {} => {
            let connection_id = CONFIG.load(deps.storage)?.connection_id;
            execute_register_ica(deps, env, connection_id, INTERCHAIN_ACCOUNT_ID.to_string())
        }
        ExecuteMsg::RegisterTransferNftQuery {
            min_height,
            sender,
            token_id,
        } => register_transfer_nft_query(deps, env, min_height, sender, token_id),
        // todo: add NFT ownership query
        ExecuteMsg::RemoveInterchainQuery { query_id } => remove_interchain_query(query_id, info.sender),
        ExecuteMsg::UnlockNft {
            token_id,
            destination,
        } => execute_unlock_nft(deps, env, info, token_id, destination),
        ExecuteMsg::MintNft { token_id } => execute_mint_nft(deps, env,info, token_id),
        ExecuteMsg::UpdateConfig { update_period, nft_contract_address } => {
            execute_update_config(deps, env, info, update_period, nft_contract_address)
        }
    }
}

fn execute_update_config(
    deps: DepsMut<'_, NeutronQuery>,
    env: Env,
    info: MessageInfo,
    update_period: Option<u64>,
    nft_contract_address: Option<String>,
) -> Result<Response<NeutronMsg>, NeutronError> {
    let mut config = CONFIG.load(deps.storage)?;

    if let Some(update_period) = update_period {
        config.update_period = update_period;
    }

    if let Some(nft_contract_address) = nft_contract_address {
        config.nft_contract_address = nft_contract_address;
    }

    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn register_transfer_nft_query(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    min_height: u64,
    sender: String,
    token_id: String,
) -> NeutronResult<Response<NeutronMsg>> {
    let config = CONFIG.load(deps.storage)?;

    let (ica_account, connection_id) = get_ica(deps.as_ref(), &env, INTERCHAIN_ACCOUNT_ID)?;
    CACHED_TOKEN_ID.save(deps.storage, &token_id)?;

    let tx_query_msg = new_register_transfer_nft_query_msg(
        connection_id.clone(),
        config.update_period,
        min_height,
        ica_account,
        sender,
        config.nft_contract_address.clone(),
        token_id.clone(),
    )?;

    // let kv_query_msg = new_register_nft_owned_query_msg(connection_id, config.update_period, config.nft_contract_address, token_id)?;

    Ok(Response::new()
       // .add_message(kv_query_msg)
    .add_submessage(SubMsg::reply_on_success(tx_query_msg, QUERY_REGISTER_REPLY_ID)))
}

pub fn remove_interchain_query(query_id: u64, sender: Addr) -> NeutronResult<Response<NeutronMsg>> {
    let remove_msg = NeutronMsg::remove_interchain_query(query_id);
    let transfer_msg = BankMsg::Send {
        to_address: sender.into(),
        amount: vec![coin(100000u128, "untrn")],
    };
    Ok(Response::new()
    .add_message(remove_msg)
    .add_message(transfer_msg))
}

fn execute_mint_nft(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    token_id: String,
) -> NeutronResult<Response<NeutronMsg>> {
    // We need to verify the query
    let query_id = TOKEN_ID_QUERY_PAIRS.load(deps.storage, token_id.clone())?;
    let sender_addr = verify_query(deps.as_ref(),&env, token_id.clone(), info.sender.clone())?;

    // Now that we have the address, we can mint our token to the recipient which validates their ownership of the bad kid
    let addr = any_addr_to_neutron(deps.as_ref(), sender_addr)?; 

    // close the query (gets back some funds)
    let resp = remove_interchain_query(query_id, info.sender)?;

    // Mint the new cw20 tokens (costs some funds - on testnet it's the same)
    let resp = resp.add_submessages(mint_native_receipt(deps, env, token_id, addr)?.messages);

    Ok(resp) 
}

fn execute_unlock_nft(
    mut deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    token_id: String,
    destination_addr: String,
) -> NeutronResult<Response<NeutronMsg>> {
    let config = CONFIG.load(deps.storage)?;

    let denom = get_token_denom(deps.as_ref(), env.clone(), token_id.clone())?;

    let amount = info.funds.iter().find(|c| c.denom == denom).ok_or(StdError::generic_err(format!(
            "You need to pay at least{} to unlock your token {}",
            THRESHOLD_BURN_AMOUNT, token_id
        )))?.amount;

    if amount < THRESHOLD_BURN_AMOUNT.into() {
        return Err(NeutronError::Std(StdError::generic_err(format!(
            "You need to pay at least{} to unlock your token {}",
            THRESHOLD_BURN_AMOUNT, token_id
        ))));
    }

    // contract must pay for relaying of acknowledgements
    // See more info here: https://docs.neutron.org/neutron/feerefunder/overview
    let fee = min_ntrn_ibc_fee(query_min_ibc_fee(deps.as_ref())?.min_fee);
    let (account_addr, connection_id) = get_ica(deps.as_ref(), &env, INTERCHAIN_ACCOUNT_ID)?;

    let unlock_message = MsgExecuteContract {
        contract: config.nft_contract_address,
        msg: to_binary(&Cw721ExecuteMsg::TransferNft {
            recipient: destination_addr,
            token_id,
        })?
        .to_vec(),
        funds: vec![],
        sender: account_addr,
    };

    let mut buf = Vec::new();
    buf.reserve(unlock_message.encoded_len());

    if let Err(e) = unlock_message.encode(&mut buf) {
        return Err(NeutronError::Std(StdError::generic_err(format!(
            "Encode error: {}",
            e
        ))));
    }

    let any_msg = unlock_message
        .to_any() // Using the to_any feature to not mess it up
        .map_err(|e| NeutronError::Std(StdError::generic_err(e.to_string())))?;

    let cosmos_msg = NeutronMsg::submit_tx(
        connection_id,
        INTERCHAIN_ACCOUNT_ID.to_string(),
        vec![ProtobufAny {
            value: Binary(any_msg.value),
            type_url: any_msg.type_url,
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

#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<NeutronQuery>, env: Env, msg: QueryMsg) -> NeutronResult<Binary> {
    match msg {
        QueryMsg::IcaAccount {} => query_ica_account(deps, env),
        QueryMsg::TokenDenom { token_id } => query_token_denom(deps, env, token_id),
        QueryMsg::NftTransfers { sender } => {
            Ok(to_binary(&query_nft_transfers(deps, env, sender)?)?)
        }
        QueryMsg::GetRegisteredQuery { query_id } => {
            Ok(to_binary(&get_registered_query(deps, query_id)?)?)
        },
        QueryMsg::GetQueryId { token_id } => {
            Ok(to_binary(&get_query_id(deps, token_id)?)?)
        },
    }
}

fn query_ica_account(
    deps: Deps<NeutronQuery>,
    env: Env,
) -> NeutronResult<Binary> {

    let (account, _connection_id) = get_ica(deps, &env, INTERCHAIN_ACCOUNT_ID)?;

    Ok(to_binary(&account)?)
}

fn query_token_denom(deps: Deps<NeutronQuery>, env: Env, token_id: String) -> NeutronResult<Binary> {
    let (account, connection_id) = get_ica(deps, &env, INTERCHAIN_ACCOUNT_ID)?;

    Ok(to_binary(&get_token_denom(deps, env, token_id)?)?)
}

fn get_token_denom(deps: Deps<NeutronQuery>, env: Env, token_id: String) -> NeutronResult<String>{

    // We need to make sure, that the client pays enough tokens to unlock their tokens
    let denom_count = MINTED_TOKENS.load(deps.storage, token_id.clone())?;
    let denom = format_token_denom(env.clone(), token_id.clone(), denom_count);

    Ok(denom)
}


fn query_nft_transfers(
    deps: Deps<NeutronQuery>,
    _env: Env,
    sender: String,
) -> NeutronResult<NftTransfersResponse> {
    let nft_transfers = SENDER_TXS.load(deps.storage, sender.as_str())?;
    return Ok(NftTransfersResponse {
        transfers: nft_transfers,
    });
}

#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    deps.api.debug("WASMDEBUG: migrate");
    Ok(Response::default())
}

#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
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
        SudoMsg::KVQueryResult { query_id } => sudo_kv_query_result(deps, env, query_id),

        // For handling successful (non-error) acknowledgements.
        SudoMsg::Response { request, data } => Ok(sudo_response(deps, request, data)?),

        // For handling error acknowledgements.
        SudoMsg::Error { request, details } => Ok(sudo_error(deps, request, details)?),

        // For handling error timeouts.
        SudoMsg::Timeout { request } => Ok(sudo_timeout(deps, env, request)?),

        // For handling successful registering of ICA
        SudoMsg::OpenAck {
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_version,
        } => Ok(sudo_open_ack(
            deps,
            env,
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_version,
        )?),
        _ => Ok(Response::default()),
    }
}


fn get_query_id(deps: Deps<NeutronQuery>, token_id: String) -> NeutronResult<u64> {
    let query_id = TOKEN_ID_QUERY_PAIRS
        .may_load(deps.storage, token_id)?
        .ok_or_else(|| NeutronError::Std(StdError::generic_err("No query id found for token id")))?;
    Ok(query_id)
}

/// sudo_kv_query_result is an example callback for key-value query results that stores the
fn sudo_kv_query_result(deps: DepsMut<NeutronQuery>, _env: Env, query_id: u64) -> NeutronResult<Response> {
    let response = get_raw_interchain_query_result(deps.as_ref(), query_id)?;
    let store_value = response.result.kv_results[0].value.clone();
    let token_info: TokenInfo<Empty> = from_binary(&store_value)?;
    let (token_id, _ ) = TOKEN_ID_QUERY_PAIRS.range(deps.storage, None, None, cosmwasm_std::Order::Ascending).into_iter()
        .map(|r| r.unwrap())
        .find(|kv| kv.1 == query_id)
        .unwrap();

    TOKEN_INFOS.save(deps.storage, token_id.clone(), &token_info)?;
    Ok(Response::new().add_attribute("kv_query_store", token_id))
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
    let _transactions_filter = registered_query.registered_query.transactions_filter;

    #[allow(clippy::match_single_binding)]
    // Depending of the query type, check the transaction data to see whether is satisfies
    // the original query. If you don't write specific checks for a transaction query type,
    // all submitted results will be treated as valid.

    // TODO: come up with solution to determine transactions filter type
    match registered_query.registered_query.query_type {
        _ => {
            if let Some(msg) = body.messages.get(0) {
                let contract_msg = MsgExecuteContract::decode(msg.value.as_slice()).unwrap();

                let transfer_msg: Cw721ExecuteMsg = from_binary(&contract_msg.msg.into())?;

                match transfer_msg {
                    Cw721ExecuteMsg::TransferNft {
                        token_id,
                        recipient,
                    } => {
                        let sender = contract_msg.sender;
                        let receiver_addr = recipient;

                        let (ica_address, _) =
                            get_ica(deps.as_ref(), &_env, INTERCHAIN_ACCOUNT_ID)?;
                        if receiver_addr != ica_address {
                            return Err(NeutronError::Std(StdError::generic_err(format!(
                                "Receiver is not the ica account: {}, should be '{}' ",
                                receiver_addr,
                                ica_address
                            ))));
                        }

                        let contract_address = contract_msg.contract;

                        let transfer_nft = NftTransfer {
                            sender: sender.clone(),
                            contract_address,
                            token_id: token_id.clone(),
                        };

                        let mut stored_transfers: u64 =
                            TRANSFERS.load(deps.storage).unwrap_or_default();
                        stored_transfers += 1u64;
                        TRANSFERS.save(deps.storage, &stored_transfers)?;

                        let mut stored_deposits: Vec<NftTransfer> = SENDER_TXS
                            .load(deps.storage, sender.as_str())
                            .unwrap_or_default();

                        stored_deposits.push(transfer_nft.clone());
                        SENDER_TXS.save(deps.storage, sender.as_str(), &stored_deposits)?;

                        // We save the sender for each token id, because we need it to be able to mint the Tokens to the right person
                        TOKEN_ID_SENDER.save(deps.storage, token_id, &sender)?;

                        return Ok(Response::new().add_attribute(
                            "transfer_nft",
                            serde_json_wasm::to_string(&transfer_nft)
                                .map_err(|e| NeutronError::SerdeJSONWasm(e.to_string()))?,
                        ));
                    }
                    // message type is different from SendNft -> Contract broken
                    _ => panic!("message is not SendNft"),
                }
            } else {
                // messages are empty, no updates to state
                return Ok(Response::default());
            }
        }
    }
}

#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut<NeutronQuery>, env: Env, reply: Reply) -> NeutronResult<Response> {
    match reply.id {
        QUERY_REGISTER_REPLY_ID => {
            let resp: MsgRegisterInterchainQueryResponse = serde_json_wasm::from_slice(
                reply
                    .result
                    .into_result()
                    .map_err(StdError::generic_err)?
                    .data
                    .ok_or_else(|| StdError::generic_err("no result"))?
                    .as_slice(),
            )?;

            let query_id = resp.id;

            let token_id = CACHED_TOKEN_ID.load(deps.storage)?;
            CACHED_TOKEN_ID.remove(deps.storage);
            TOKEN_ID_QUERY_PAIRS.save(deps.storage, token_id, &query_id)?;

            Ok(Response::new().add_attribute("query_id", query_id.to_string()))
        }
        SUDO_PAYLOAD_REPLY_ID => Ok(prepare_sudo_payload(deps, env, reply)?),
        _ => Err(NeutronError::Std(StdError::generic_err("Wrong reply id"))),
    }
}
