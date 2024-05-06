use crate::bindings::oracle::types::{CurrencyPair, QuotePrice};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OracleQuery {
    /// Parameters queries the parameters of the module.
    GetAllCurrencyPairs {},
    GetPrice {
        currency_pair: CurrencyPair,
    },
    GetPrices {
        currency_pair_ids: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetPriceResponse {
    // QuotePrice represents the quote-price for the CurrencyPair given in
    // GetPriceRequest (possibly nil if no update has been made)
    pub price: QuotePrice,
    // nonce represents the nonce for the CurrencyPair if it exists in state
    pub nonce: u64,
    // decimals represents the number of decimals that the quote-price is
    // represented in. For Pairs where ETHEREUM is the quote this will be 18,
    // otherwise it will be 8.
    pub decimals: u64,
    // ID represents the identifier for the CurrencyPair.
    pub id: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetPricesResponse {
    pub prices: Vec<GetPriceResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetAllCurrencyPairsResponse {
    pub currency_pairs: Vec<CurrencyPair>,
}
