use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, PriceResponse, QueryMsg};
use crate::{ibc_msg, obi_calldata};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo,
    Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use neutron_sdk::NeutronResult;

use crate::state::{CHANNEL, PRICES};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:band_feed";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// OracleRequest params
// TODO: what is REQUEST_CLIENT_ID?
const REQUEST_CLIENT_ID: &str = "TODO: client-1234";
const ORACLE_SCRIPT_ID: u64 = 111; // https://laozi-testnet6.cosmoscan.io/oracle-script/111
const ASK_COUNT: u64 = 3;
const MIN_COUNT: u64 = 3;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
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
) -> StdResult<Response<CosmosMsg>> {
    deps.api.debug("WASMDEBUG: execute");
    match msg {
        ExecuteMsg::SubscribePriceFeed {} => execute_subscribe_price_feed(deps, env),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> NeutronResult<Binary> {
    match msg {
        QueryMsg::GetPrice {} => Ok(to_binary(&query_price(deps, env)?)?),
    }
}

fn execute_subscribe_price_feed(deps: DepsMut, env: Env) -> StdResult<Response<CosmosMsg>> {
    let channel_id = CHANNEL.load(deps.storage)?;
    let oracle_request = ibc_msg::OracleRequest::OracleRequestPacketData {
        client_id: REQUEST_CLIENT_ID.to_string(),
        oracle_script_id: ORACLE_SCRIPT_ID,
        calldata: obi_calldata::calldata(),
        ask_count: ASK_COUNT,
        min_count: MIN_COUNT,
        fee_limit: vec![Coin {
            denom: "uband".to_string(),
            amount: Uint128::new(6000000),
        }],
        prepare_gas: 10000,
        execute_gas: 1000,
    };
    let msg = CosmosMsg::Ibc(IbcMsg::SendPacket {
        channel_id,
        data: to_binary(&oracle_request)?,
        timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(10 * 60)),
    });
    Ok(Response::new().add_message(msg))
}

fn query_price(deps: Deps, _env: Env) -> NeutronResult<PriceResponse> {
    let prices = PRICES.load(deps.storage)?;

    let prices_str = if let (_resolve_status, Some(prices)) = prices {
        prices
            .into_iter()
            .map(|maybe_price| maybe_price.to_string())
            .collect::<Vec<String>>()
    } else {
        // TODO: error show
        vec!["".to_string()]
    };

    Ok(PriceResponse { prices: prices_str })
}

#[cfg(test)]
mod tests {}
