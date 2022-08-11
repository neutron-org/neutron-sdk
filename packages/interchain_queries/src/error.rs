use crate::types::QueryType;
use cosmwasm_std::{DecimalRangeExceeded, OverflowError, StdError};
use serde_json_wasm;
use thiserror::Error;

pub type ContractResult<T> = Result<T, ContractError>;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Fmt(#[from] std::fmt::Error),

    #[error("Bech32 error")]
    Bech32(#[from] bech32::Error),

    #[error("Prost protobuf error")]
    ProstProtobuf(#[from] prost::DecodeError),

    #[error("Serde JSON (Wasm) error")]
    SerdeJSONWasm(String),

    #[error("address length should be max {max:?} bytes, got {actual:?}")]
    MaxAddrLength { max: usize, actual: usize },

    #[error("invalid reply id: {0}")]
    InvalidReplyID(u64),

    #[error("invalid query type: expected '{expected:?}' got {actual:?}")]
    InvalidQueryType { expected: QueryType, actual: String },

    #[error("Decimal range exceeded")]
    DecimalRangeExceeded(#[from] DecimalRangeExceeded),

    #[error("Overflow error")]
    OverflowError(#[from] OverflowError),
}

impl From<serde_json_wasm::de::Error> for ContractError {
    fn from(e: serde_json_wasm::de::Error) -> Self {
        ContractError::SerdeJSONWasm(e.to_string())
    }
}
