use schemars::{JsonSchema, Map};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Params {
    pub version: u64,
    pub market_authority: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MarketMap {
    // Tickers is the full list of tickers and their associated configurations
    // to be stored on-chain.
    pub tickers: Map<String, Ticker>,
    // Paths is a map from CurrencyPair to all paths that resolve to that pair
    pub paths: Map<String, Paths>,
    // Providers is a map from CurrencyPair to each of to provider-specific
    // configs associated with it.
    pub providers: Map<String, Providers>,
    // AggregationType is the type of aggregation that will be used to aggregate
    // the prices of the tickers.
    pub aggregation_type: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Providers {
    // Providers is the list of provider configurations for the given ticker.
    pub providers: Vec<ProviderConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProviderConfig {
    // Name corresponds to the name of the provider for which the configuration is
    // being set.
    pub name: String,
    // OffChainTicker is the off-chain representation of the ticker i.e. BTC/USD.
    // The off-chain ticker is unique to a given provider and is used to fetch the
    // price of the ticker from the provider.
    pub off_chain_ticker: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Paths {
    // Paths is the list of convertable markets that will be used to convert the
    // prices of a set of tickers to a common ticker.
    pub paths: Vec<Path>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Path {
    // Operations is an ordered list of operations that will be taken. These must
    // be topologically sorted to ensure that the conversion is possible i.e. DAG.
    pub operations: Vec<Operation>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Operation {
    // CurrencyPair is the on-chain currency pair for this ticker.
    pub currency_pair: CurrencyPair,
    // Invert is a boolean that indicates whether the price of the ticker should
    // be inverted.
    pub invert: bool,
    // Provider is the name of the provider that will be used to fetch the price
    // of the ticker.
    pub provider: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CurrencyPair {
    #[serde(rename(serialize = "Base", deserialize = "Base"))]
    pub base: String,
    #[serde(rename(serialize = "Quote", deserialize = "Quote"))]
    pub quote: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Ticker {
    // CurrencyPair is the currency pair for this ticker.
    pub currency_pair: CurrencyPair,
    // Decimals is the number of decimal places for the ticker. The number of
    // decimal places is used to convert the price to a human-readable format.
    pub decimals: u64,
    // MinProviderCount is the minimum number of providers required to consider
    // the ticker valid.
    pub min_provider_count: u64,
    // Enabled is the flag that denotes if the Ticker is enabled for price
    // fetching by an oracle.
    pub enabled: bool,
    // MetadataJSON is a string of JSON that encodes any extra configuration
    // for the given ticker.
    pub metadata_json: String,
}
