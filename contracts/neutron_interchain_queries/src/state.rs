use cosmwasm_std::from_binary;
use cosmwasm_std::to_vec;
use cosmwasm_std::Binary;
use cosmwasm_std::CustomQuery;
use cosmwasm_std::Deps;
use cosmwasm_std::Env;
use cosmwasm_std::StdError;
use cosmwasm_std::StdResult;
use cosmwasm_std::Storage;
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

pub fn save_reply_payload(store: &mut dyn Storage, payload: SudoPayload) -> StdResult<()> {
    REPLY_ID_STORAGE.save(store, &to_vec(&payload)?)
}

pub fn read_reply_payload(store: &mut dyn Storage) -> StdResult<SudoPayload> {
    let data = REPLY_ID_STORAGE.load(store)?;
    from_binary(&Binary(data))
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

/// This stores the latest id of the token created against the token_id
pub const MINTED_TOKENS: Map<String, u64> = Map::new("minted_tokens");
pub const TOTAL_MINTED_TOKENS: Item<u64> = Item::new("total_minted_tokens");

/// This stores the address of the minted tokenfactory token that is the right one for the token_id
pub const CONFIG: Item<Config> = Item::new("config");

#[cosmwasm_schema::cw_serde]
pub struct Config {
    pub connection_id: String,
    pub nft_contract_address: String, // THis is a contract address on a distant chain, so please don't verify it
    pub update_period: u64, // This is the update period in blocks
}
