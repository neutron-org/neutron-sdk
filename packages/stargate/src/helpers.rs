use crate::interchain::interchainaccounts_tx::MsgSubmitTx;
use cosmwasm_std::{
    to_vec, Binary, ContractResult, CosmosMsg, Deps, Empty, QueryRequest, StdError, StdResult,
    SystemResult,
};
use protobuf::Message;

pub const SUBMIT_INTERCHAIN_TX_PATH: &str =
    "/neutron_org.interchainadapter.interchaintxs.v1.MsgSubmitTx";

/// Helper to make a stargate query.
/// * path - is a protobuf path of any Cosmos-SDK query.
/// * encoded_query_data - is a protobuf encoded query data
///
/// Usage example:
/// ```rust,ignore
///     let mut interchain_query = QueryRegisteredQueryResultRequest::new();
///     interchain_query.query_id = 1;
///
///     let encoded_query_bytes = interchain_query.write_to_bytes()?;
///
///     let interchain_query_result: QueryRegisteredQueryResultResponse = make_stargate_query(
///         deps,
///         QUERY_REGISTERED_QUERY_RESULT_PATH.to_string(),
///         encoded_query_bytes,
///     )?;
/// ```
pub fn make_stargate_query<T: Message>(
    deps: Deps,
    path: String,
    encoded_query_data: Vec<u8>,
) -> StdResult<T> {
    let raw = to_vec::<QueryRequest<Empty>>(&QueryRequest::Stargate {
        path,
        data: Binary::from(encoded_query_data),
    })
    .map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    })?;
    match deps.querier.raw_query(&raw) {
        SystemResult::Err(system_err) => {
            return Err(StdError::generic_err(format!(
                "Querier system error: {}",
                system_err
            )))
        }
        SystemResult::Ok(ContractResult::Err(contract_err)) => {
            return Err(StdError::generic_err(format!(
                "Querier contract error: {}",
                contract_err
            )))
        }
        // response(value) is base64 encoded bytes
        SystemResult::Ok(ContractResult::Ok(value)) => Message::parse_from_bytes(value.as_slice())
            .map_err(|e| StdError::generic_err(format!("Protobuf parsing error: {}", e))),
    }
}

/// Composes a protobuf encoded message (to be sent via stargate) for submitting an interchain transaction
pub fn make_stargate_tx(message: MsgSubmitTx) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Stargate {
        type_url: SUBMIT_INTERCHAIN_TX_PATH.to_string(),
        value: Binary::from(
            message
                .write_to_bytes()
                .map_err(|e| StdError::generic_err(e.to_string()))?,
        ),
    })
}
