#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
// use cw_band::

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:band_feed";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    deps.api.debug("WASMDEBUG: instantiate");
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
      ExecuteMsg::AskPrice{} => execute_ask_price(deps, env)
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> NeutronResult<Binary>{
    match msg {
        QueryMsg::GetPrice{} => query_price(deps, env),
    }
}

fn execute_ask_price(deps: DepsMut, env: Env) -> NeutronResult<Response<NeutronMsg>> {

}

fn query_price(deps: Deps<InterchainQueries>, env: Env) -> NeutronResult<Binary>{

}

// https://docs.rs/cw-band/0.1.0-alpha.2/cw_band/struct.OracleRequestPacketData.html
// pub struct OracleRequestPacketData {
//     pub client_id: String,
//     pub oracle_script_id: Uint64,
//     pub calldata: Vec<u8>,
//     pub ask_count: Uint64,
//     pub min_count: Uint64,
//     pub fee_limit: Vec<Coin>,
//     pub prepare_gas: Uint64,
//     pub execute_gas: Uint64,
// }
// fn ask_price_msg() -> NeutronMsg {
    // TODO
    // cw_band::OracleRequestPacketData{

    // }
    // return NeutronMsg
// }

#[cfg(test)]
mod tests {}
