use cosmwasm_std::Empty;
use cosmwasm_std::Order;
use cosmwasm_std::from_binary;
use cosmwasm_std::to_vec;
use cosmwasm_std::Binary;
use cosmwasm_std::CustomQuery;
use cosmwasm_std::Deps;
use cosmwasm_std::Env;
use cosmwasm_std::StdError;
use cosmwasm_std::StdResult;
use cosmwasm_std::Storage;
use cw721_base::state::TokenInfo;
use cw_storage_plus::{Item, Map};
use neutron_sdk::interchain_txs::helpers::get_port_id;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type Sender = str;

/// contains all transfers mapped by a recipient address observed by the contract.
pub const SENDER_TXS: Map<&Sender, Vec<NftTransfer>> = Map::new("recipient_txs");
/// contains number of transfers to addresses observed by the contract.
pub const TRANSFERS: Item<u64> = Item::new("nft-transfers");
pub const CACHED_TOKEN_ID: Item<String> = Item::new("cached_token_id");
pub const TOKEN_ID_QUERY_PAIRS: Map<String, u64> = Map::new("token_id_query_pairs");

/// not working yet 
pub const TOKEN_INFOS: Map<String, TokenInfo<Empty>> = Map::new("token_infos");

// For each token_id, we need to be able to get the sender of that NFT in the contract
// Don't forget to clear that storage after the nft is indeed transfered to neutron
pub const TOKEN_ID_SENDER: Map<String, String> = Map::new("token_id_sender");

/// This stores the latest id of the token created against the token_id
pub const MINTED_TOKENS: Map<String, u64> = Map::new("minted_tokens");
pub const TOTAL_MINTED_TOKENS: Item<u64> = Item::new("total_minted_tokens");

/// This stores the address of the minted tokenfactory token that is the right one for the token_id
pub const CONFIG: Item<Config> = Item::new("config");

#[cosmwasm_schema::cw_serde]
pub struct Config {
    pub connection_id: String,
    pub nft_contract_address: String, // THis is a contract address on a distant chain, so please don't verify it
    pub update_period: u64,           // This is the update period in blocks
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct NftTransfer {
    /// The address of the sender in the host chain
    pub sender: String,
    /// he address of the specific NFT collection
    pub contract_address: String,
    /// The ID of the NFT
    pub token_id: String,
}



// ICA Related part

/// SudoPayload is a type that stores information about a transaction that we try to execute
/// on the host chain. This is a type introduced for our convenience.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SudoPayload {
    pub message: String,
    pub port_id: String,
}

pub const SUDO_PAYLOAD_REPLY_ID: u64 = 1;

pub const REPLY_ID_STORAGE: Item<Vec<u8>> = Item::new("reply_queue_id");
pub const SUDO_PAYLOAD: Map<(String, u64), Vec<u8>> = Map::new("sudo_payload");

// interchain transaction responses - ack/err/timeout state to query later
pub const ACKNOWLEDGEMENT_RESULTS: Map<(String, u64), AcknowledgementResult> =
    Map::new("acknowledgement_results");

pub const ERRORS_QUEUE: Map<u32, String> = Map::new("errors_queue");

/// Serves for storing acknowledgement calls for interchain transactions
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AcknowledgementResult {
    /// Success - Got success acknowledgement in sudo with array of message item types in it
    Success(Vec<String>),
    /// Error - Got error acknowledgement in sudo with payload message in it and error details
    Error((String, String)),
    /// Timeout - Got timeout acknowledgement in sudo with payload message in it
    Timeout(String),
}

pub fn add_error_to_queue(store: &mut dyn Storage, error_msg: String) -> Option<()> {
    let result = ERRORS_QUEUE
        .keys(store, None, None, Order::Descending)
        .next()
        .and_then(|data| data.ok())
        .map(|c| c + 1)
        .or(Some(0));

    result.and_then(|idx| ERRORS_QUEUE.save(store, idx, &error_msg).ok())
}

pub fn read_errors_from_queue(store: &dyn Storage) -> StdResult<Vec<(Vec<u8>, String)>> {
    ERRORS_QUEUE
        .range_raw(store, None, None, Order::Ascending)
        .collect()
}

pub fn save_reply_payload(store: &mut dyn Storage, payload: SudoPayload) -> StdResult<()> {
    REPLY_ID_STORAGE.save(store, &to_vec(&payload)?)
}

pub fn read_reply_payload(store: &mut dyn Storage) -> StdResult<SudoPayload> {
    let data = REPLY_ID_STORAGE.load(store)?;
    from_binary(&Binary(data))
}

pub fn read_sudo_payload(
    store: &mut dyn Storage,
    channel_id: String,
    seq_id: u64,
) -> StdResult<SudoPayload> {
    let data = SUDO_PAYLOAD.load(store, (channel_id, seq_id))?;
    from_binary(&Binary(data))
}

pub fn save_sudo_payload(
    store: &mut dyn Storage,
    channel_id: String,
    seq_id: u64,
    payload: SudoPayload,
) -> StdResult<()> {
    SUDO_PAYLOAD.save(store, (channel_id, seq_id), &to_vec(&payload)?)
}

pub const INTERCHAIN_ACCOUNTS: Map<String, Option<(String, String)>> =
    Map::new("interchain_accounts");

pub fn get_ica(
    deps: Deps<impl CustomQuery>,
    env: &Env,
    interchain_account_id: &str,
) -> Result<(String, String), StdError> {
    let key = get_port_id(env.contract.address.as_str(), interchain_account_id);

    INTERCHAIN_ACCOUNTS
        .load(deps.storage, key)?
        .ok_or_else(|| StdError::generic_err("Interchain account is not created yet"))
}


