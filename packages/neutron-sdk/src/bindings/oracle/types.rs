use cosmwasm_std::Int128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CurrencyPair {
    pub base: String,
    pub quote: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QuotePrice {
    pub price: Int128,
    // // BlockTimestamp tracks the block height associated with this price update.
    // // We include block timestamp alongside the price to ensure that smart
    // // contracts and applications are not utilizing stale oracle prices
    // block_timestamp: time.Time,
    // BlockHeight is height of block mentioned above
    pub block_height: u64,
}
