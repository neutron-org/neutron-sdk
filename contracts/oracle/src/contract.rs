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

use neutron_sdk::bindings::{
    msg::NeutronMsg,
    oracle::query::{
        GetAllCurrencyPairsResponse, GetPriceResponse, GetPricesResponse, OracleQuery,
    },
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
pub fn query(deps: Deps<NeutronQuery>, env: Env, msg: OracleQuery) -> StdResult<Binary> {
    query_oracle(deps, env, msg)
}

fn query_oracle(deps: Deps<NeutronQuery>, _env: Env, msg: OracleQuery) -> StdResult<Binary> {
    match msg {
        OracleQuery::GetPrice { .. } => {
            let query_response: GetPriceResponse = deps.querier.query(&msg.into())?;
            to_json_binary(&query_response)
        }
        OracleQuery::GetPrices { .. } => {
            let query_response: GetPricesResponse = deps.querier.query(&msg.into())?;
            to_json_binary(&query_response)
        }
        OracleQuery::GetAllCurrencyPairs { .. } => {
            let query_response: GetAllCurrencyPairsResponse = deps.querier.query(&msg.into())?;
            to_json_binary(&query_response)
        }
    }
}
