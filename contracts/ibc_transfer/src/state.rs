use cosmwasm_std::{from_binary, to_vec, Binary, StdResult, Storage};
use cw_storage_plus::Map;

use crate::contract::SudoPayload;

pub const IBC_SUDO_ID_RANGE_START: u64 = 1_000_000_000;
pub const IBC_SUDO_ID_RANGE_SIZE: u64 = 1_000_000;
pub const IBC_SUDO_ID_RANGE_END: u64 = IBC_SUDO_ID_RANGE_START + IBC_SUDO_ID_RANGE_SIZE;

pub const REPLY_QUEUE_ID: Map<u64, Vec<u8>> = Map::new("reply_queue_id");

pub fn get_next_id(store: &mut dyn Storage) -> StdResult<u64> {
    //TODO ?: ring buffer for range IBC_SUDO_ID_RANGE_START IBC_SUDO_ID_RANGE_END
    // or 2^50 elements is enough for contract lifetime?
    REPLY_QUEUE_ID
        .keys(store, None, None, cosmwasm_std::Order::Descending)
        .next()
        .unwrap_or(Ok(IBC_SUDO_ID_RANGE_START))
        .map(|id| id + 1)
}

pub fn save_reply_payload(store: &mut dyn Storage, payload: SudoPayload) -> StdResult<u64> {
    let id = get_next_id(store)?;
    REPLY_QUEUE_ID.save(store, id, &to_vec(&payload)?)?;
    Ok(id)
}

pub fn read_reply_payload(store: &mut dyn Storage, id: u64) -> StdResult<SudoPayload> {
    let data = REPLY_QUEUE_ID.load(store, id)?;
    from_binary(&Binary(data))
}

//TODO: SUDO_PAYLOAD must be channel specific
pub const SUDO_PAYLOAD: Map<u64, Vec<u8>> = Map::new("sudo_payload");

pub fn save_sudo_payload(
    store: &mut dyn Storage,
    seq_id: u64,
    payload: SudoPayload,
) -> StdResult<()> {
    SUDO_PAYLOAD.save(store, seq_id, &to_vec(&payload)?)
}

pub fn read_sudo_payload(store: &mut dyn Storage, seq_id: u64) -> StdResult<SudoPayload> {
    let data = SUDO_PAYLOAD.load(store, seq_id)?;
    from_binary(&Binary(data))
}
