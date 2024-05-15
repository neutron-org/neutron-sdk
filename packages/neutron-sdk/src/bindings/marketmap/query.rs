use crate::bindings::marketmap::types::{MarketMap, Params};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarketmapQuery {
    /// Parameters queries the parameters of the module.
    Params {},
    GetLastUpdated {},
    GetMarketMap {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ParamsResponse {
    pub params: Params,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetLastUpdatedResponse {
    pub last_updated: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetMarketMapResponse {
    // MarketMap defines the global set of market configurations for all providers
    // and markets.
    pub market_map: MarketMap,
    // LastUpdated is the last block height that the market map was updated.
    // This field can be used as an optimization for clients checking if there
    // is a new update to the map.
    pub last_updated: u64,
    // ChainId is the chain identifier for the market map.
    pub chain_id: String,
}
