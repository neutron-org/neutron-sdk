use crate::bindings::marketmap::types::{Market, MarketMap, Params};
use crate::bindings::oracle::types::CurrencyPair;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarketMapQuery {
    /// Parameters queries the parameters of the module.
    Params {},
    LastUpdated {},
    MarketMap {},
    Market {
        currency_pair: CurrencyPair,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ParamsResponse {
    pub params: Params,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LastUpdatedResponse {
    pub last_updated: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MarketMapResponse {
    /// **market_map** defines the global set of market configurations for all providers
    /// and markets.
    pub market_map: MarketMap,
    /// **last_updated** is the last block height that the market map was updated.
    /// This field can be used as an optimization for clients checking if there
    /// is a new update to the map.
    pub last_updated: Option<u64>,
    /// **chain_id** is the chain identifier for the market map.
    pub chain_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MarketResponse {
    pub market: Market,
}
