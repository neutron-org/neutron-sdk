use crate::types::QueryType;
use cosmwasm_std::StdError;
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

    #[error("Protobuf error")]
    Protobuf(String),

    #[error("balance query with query_id '{query_id:?}' not found")]
    BalanceNotFound { query_id: u64 },

    #[error("Serde JSON (Wasm) error")]
    SerdeJSONWasm(String),

    #[error("interchain query for {zone_id:?} {query_type:?} {query_data_json_encoded:?} is not registered")]
    InterchainQueryIsNotRegistered {
        zone_id: String,
        query_type: String,
        query_data_json_encoded: String,
    },

    #[error("address length should be max {max:?} bytes, got {actual:?}")]
    MaxAddrLength { max: usize, actual: usize },

    #[error("no result data in register interchain query response")]
    EmptyInterchainQueryResult,

    #[error("register interchain query failed: {0}")]
    RegisterInterchainQueryFailed(String),

    #[error("invalid reply id: {0}")]
    InvalidReplyID(u64),

    #[error("invalid query type: expected '{expected:?}' got {actual:?}")]
    InvalidQueryType { expected: QueryType, actual: String },
}

impl From<serde_json_wasm::de::Error> for ContractError {
    fn from(e: serde_json_wasm::de::Error) -> Self {
        ContractError::SerdeJSONWasm(e.to_string())
    }
}
