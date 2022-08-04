use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub connection_id: String,
    pub interchain_account_id: String,
    pub denom: String,
}

pub const IBC_SUDO_ID_RANGE_START: u64 = 1_000_000_000;
pub const IBC_SUDO_ID_RANGE_SIZE: u64 = 1_000_000;
pub const IBC_SUDO_ID_RANGE_END: u64 = IBC_SUDO_ID_RANGE_START + IBC_SUDO_ID_RANGE_SIZE;

pub const REPLY_QUEUE_ID: Map<u64, Vec<u8>> = Map::new("reply_queue_id");
pub const SUDO_PAYLOAD: Map<u64, Vec<u8>> = Map::new("sudo_payload");
pub const CONFIG: Item<Config> = Item::new("config");
