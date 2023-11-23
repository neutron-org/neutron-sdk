use crate::proto_types::neutron::dex::{QueryParamsRequest, QueryParamsResponse};
use cosmwasm_std::{
    Binary, ContractResult, Deps, Empty, QueryRequest, StdError, StdResult, SystemResult,
};
use prost::bytes::Bytes;
use prost::Message;
use serde_json_wasm::to_vec;

const PARAMS_QUERY_PATH: &str = "/neutron.dex.Query/Params";
const LIMIT_ORDER_TRANCHE_USER_QUERY_PATH: &str = "/neutron.dex.Query/LimitOrderTrancheUser";
const LIMIT_ORDER_TRANCHE_QUERY_PATH: &str = "/neutron.dex.Query/LimitOrderTranche";
const USER_DEPOSITS_ALL_QUERY_PATH: &str = "/neutron.dex.Query/UserDepositsAll";
const USER_LIMIT_ORDERS_ALL_QUERY_PATH: &str = "/neutron.dex.Query/UserLimitOrdersAll";
const INACTIVE_LIMIT_ORDER_TRANCHE_QUERY_PATH: &str =
    "/neutron.dex.Query/InactiveLimitOrderTranche";
const POOL_RESERVES_QUERY_PATH: &str = "/neutron.dex.Query/PoolReserves";
const ESTIMATE_MULTI_HOP_SWAP_QUERY_PATH: &str = "/neutron.dex.Query/EstimateMultiHopSwap";
const ESTIMATE_PLACE_LIMIT_ORDER_QUERY_PATH: &str = "/neutron.dex.Query/EstimatePlaceLimitOrder";

// FOR MSG: cosmwasm_std::CosmosMsg::Stargate { type_url: (), value: () }

pub fn get_neutron_dex_params(deps: Deps) -> StdResult<QueryParamsResponse> {
    let payload = QueryParamsRequest {};
    let resp = make_stargate_query(deps, PARAMS_QUERY_PATH.to_string(), payload.encode_to_vec())?;
    QueryParamsResponse::decode(Bytes::copy_from_slice(&resp))
        .map_err(|e| StdError::generic_err(e.to_string()))
}

fn make_stargate_query(deps: Deps, path: String, encoded_query_data: Vec<u8>) -> StdResult<Binary> {
    let raw = to_vec::<QueryRequest<Empty>>(&QueryRequest::Stargate {
        path,
        data: encoded_query_data.into(),
    })
    .map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    })?;
    match deps.querier.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier system error: {}",
            system_err
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            contract_err
        ))),
        // response(value) is base64 encoded bytes
        SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
    }
}
