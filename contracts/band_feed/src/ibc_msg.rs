use cosmwasm_std::Coin;
use cosmwasm_std::ContractResult;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// All IBC acknowledgements are wrapped in `ContractResult`.
/// The success value depends on the PacketMsg variant.
pub type AcknowledgementMsg<T> = ContractResult<T>;

// TODO: improve naming
// https://github.com/bandprotocol/chain/blob/v2.4.1/proto/oracle/v1/oracle.proto#L159
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OracleRequest {
    OracleRequestPacketData {
        client_id: String,
        oracle_script_id: u64,
        calldata: Vec<u8>,
        ask_count: u64,
        min_count: u64,
        fee_limit: Vec<Coin>,
        prepare_gas: u64,
        execute_gas: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct OracleRequestPacketAcknowledgement {
    pub request_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct OracleResponsePacketData {
    pub client_id: String,
    pub oracle_script_id: u64,
    pub calldata: Vec<u8>,
    pub ask_count: u64,
    pub min_count: u64,
    pub request_id: u64,
    pub ans_count: u64,
    pub request_time: i64,
    pub resolve_time: i64,
    pub resolve_status: i32,
    pub result: Vec<u8>,
}

use std::convert::TryFrom;

pub enum ResolveStatus {
    Open = 0,
    Success = 1,
    Failure = 2,
    Expired = 3,
}

impl TryFrom<i32> for ResolveStatus {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == ResolveStatus::Open as i32 => Ok(ResolveStatus::Open),
            x if x == ResolveStatus::Success as i32 => Ok(ResolveStatus::Success),
            x if x == ResolveStatus::Failure as i32 => Ok(ResolveStatus::Failure),
            x if x == ResolveStatus::Expired as i32 => Ok(ResolveStatus::Expired),
            _ => Err(()),
        }
    }
}

// TODO: Result.status constants:
// const (
// // Open - the request is not yet resolved.
// RESOLVE_STATUS_OPEN ResolveStatus = 0
// // Success - the request has been resolved successfully with no errors.
// RESOLVE_STATUS_SUCCESS ResolveStatus = 1
// // Failure - an error occured during the request's resolve call.
// RESOLVE_STATUS_FAILURE ResolveStatus = 2
// // Expired - the request does not get enough reports from validator within the
// // timeframe.
// RESOLVE_STATUS_EXPIRED ResolveStatus = 3
// )
