use cosmwasm_std::{Deps, StdError, StdResult};
use neutron_sdk::{
    bindings::query::{
        NeutronQuery, QueryRegisteredQueryResponse, QueryRegisteredQueryResultResponse,
    },
    query::min_ibc_fee::MinIbcFeeResponse,
    NeutronResult,
};
use prost::Message;

/// Queries raw interchain query result (raw KV storage values or transactions) from Interchain Queries Module.
/// Usually it is better to implement [KVReconstruct] for your own type and then use [query_kv_result],
/// but in cases when Rust forbids to implement foreign trait [KVReconstruct] for some foreign type,
/// it is possible to use [get_raw_interchain_query_result] and reconstruct query result manually.
pub fn get_raw_interchain_query_result(
    deps: Deps,
    interchain_query_id: u64,
) -> NeutronResult<QueryRegisteredQueryResultResponse> {
    let interchain_query = NeutronQuery::InterchainQueryResult {
        query_id: interchain_query_id,
    };
    let res = deps.querier.query(&interchain_query.into())?;
    Ok(res)
}

/// Queries registered query info
pub fn get_registered_query(
    deps: Deps,
    interchain_query_id: u64,
) -> NeutronResult<QueryRegisteredQueryResponse> {
    let query = cosmos_anybuf::types::neutron::icq_query::QueryRegisteredQueryRequest {
        query_id: interchain_query_id,
    };

    let res: QueryRegisteredQueryResponse = deps.querier.query(&query.into())?;
    Ok(res)
}

pub fn query_min_ibc_fee(deps: Deps) -> NeutronResult<MinIbcFeeResponse> {
    let query = NeutronQuery::MinIbcFee {};
    Ok(deps.querier.query(&query.into())?)
}

/// Decodes protobuf any item into T structure
pub fn decode_message_response<T: Message + Default>(item: &Vec<u8>) -> StdResult<T> {
    let res = T::decode(item.as_slice());
    match res {
        Err(e) => Err(StdError::generic_err(format!("Can't decode item: {}", e))),
        Ok(data) => Ok(data),
    }
}
