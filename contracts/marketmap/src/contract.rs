use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {}

use neutron_sdk::bindings::marketmap::query::MarketResponse;
use neutron_sdk::bindings::{
    marketmap::query::{LastUpdatedResponse, MarketMapQuery, MarketMapResponse, ParamsResponse},
    msg::NeutronMsg,
    query::NeutronQuery,
};

const CONTRACT_NAME: &str = concat!("crates.io:neutron-contracts__", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    deps.api.debug("WASMDEBUG: instantiate");
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> StdResult<Response<NeutronMsg>> {
    Ok(Default::default())
}

#[entry_point]
pub fn query(deps: Deps<NeutronQuery>, env: Env, msg: MarketMapQuery) -> StdResult<Binary> {
    query_marketmap(deps, env, msg)
}

fn query_marketmap(deps: Deps<NeutronQuery>, _env: Env, msg: MarketMapQuery) -> StdResult<Binary> {
    match msg {
        MarketMapQuery::Params { .. } => {
            let query_response: ParamsResponse = deps.querier.query(&msg.into())?;
            to_json_binary(&query_response)
        }
        MarketMapQuery::LastUpdated { .. } => {
            let query_response: LastUpdatedResponse = deps.querier.query(&msg.into())?;
            to_json_binary(&query_response)
        }
        MarketMapQuery::MarketMap { .. } => {
            let query_response: MarketMapResponse = deps.querier.query(&msg.into())?;
            to_json_binary(&query_response)
        }
        MarketMapQuery::Market { .. } => {
            let query_response: MarketResponse = deps.querier.query(&msg.into())?;
            to_json_binary(&query_response)
        }
    }
}
