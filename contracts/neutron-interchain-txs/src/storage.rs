use cw_storage_plus::Map;
use interchain_txs::msg::AcknowledgementResult;

pub const IBC_SUDO_ID_RANGE_START: u64 = 1_000_000_000;
pub const IBC_SUDO_ID_RANGE_SIZE: u64 = 1_000_000;
pub const IBC_SUDO_ID_RANGE_END: u64 = IBC_SUDO_ID_RANGE_START + IBC_SUDO_ID_RANGE_SIZE;

pub const REPLY_QUEUE_ID: Map<u64, Vec<u8>> = Map::new("reply_queue_id");

// temporary storage for transferring state to ack sudo callbacks when making interchain txs
pub const SUDO_PAYLOAD: Map<u64, Vec<u8>> = Map::new("sudo_payload");

// stores each registered ICA as Map<port_id, (ica_address, controller_connection_id))
pub const INTERCHAIN_ACCOUNTS: Map<String, Option<(String, String)>> =
    Map::new("interchain_accounts");

// interchain transaction responses - ack/err/timeout state to query later
pub const ACKNOWLEDGEMENT_RESULTS: Map<String, AcknowledgementResult> =
    Map::new("acknowledgement_results");
