use cw_storage_plus::Map;

pub const IBC_SUDO_ID_RANGE_START: u64 = 1_000_000_000;
pub const IBC_SUDO_ID_RANGE_SIZE: u64 = 1_000_000;
pub const IBC_SUDO_ID_RANGE_END: u64 = IBC_SUDO_ID_RANGE_START + IBC_SUDO_ID_RANGE_SIZE;

pub const REPLY_QUEUE_ID: Map<u64, Vec<u8>> = Map::new("reply_queue_id");
pub const SUDO_PAYLOAD: Map<(String, u64), Vec<u8>> = Map::new("sudo_payload");
pub const INTERCHAIN_ACCOUNTS: Map<String, Option<(String, String)>> =
    Map::new("interchain_accounts");
