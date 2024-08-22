use neutron_std_derive::CosmwasmExt;
/// Market encapsulates a Ticker and its provider-specific configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.Market")]
pub struct Market {
    /// Ticker represents a price feed for a given asset pair i.e. BTC/USD. The
    /// price feed is scaled to a number of decimal places and has a minimum number
    /// of providers required to consider the ticker valid.
    #[prost(message, optional, tag = "1")]
    pub ticker: ::core::option::Option<Ticker>,
    /// ProviderConfigs is the list of provider-specific configs for this Market.
    #[prost(message, repeated, tag = "2")]
    pub provider_configs: ::prost::alloc::vec::Vec<ProviderConfig>,
}
/// Ticker represents a price feed for a given asset pair i.e. BTC/USD. The price
/// feed is scaled to a number of decimal places and has a minimum number of
/// providers required to consider the ticker valid.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.Ticker")]
pub struct Ticker {
    /// CurrencyPair is the currency pair for this ticker.
    #[prost(message, optional, tag = "1")]
    pub currency_pair: ::core::option::Option<super::super::types::v1::CurrencyPair>,
    /// Decimals is the number of decimal places for the ticker. The number of
    /// decimal places is used to convert the price to a human-readable format.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub decimals: u64,
    /// MinProviderCount is the minimum number of providers required to consider
    /// the ticker valid.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub min_provider_count: u64,
    /// Enabled is the flag that denotes if the Ticker is enabled for price
    /// fetching by an oracle.
    #[prost(bool, tag = "14")]
    pub enabled: bool,
    /// MetadataJSON is a string of JSON that encodes any extra configuration
    /// for the given ticker.
    #[prost(string, tag = "15")]
    pub metadata_json: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.ProviderConfig")]
pub struct ProviderConfig {
    /// Name corresponds to the name of the provider for which the configuration is
    /// being set.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// OffChainTicker is the off-chain representation of the ticker i.e. BTC/USD.
    /// The off-chain ticker is unique to a given provider and is used to fetch the
    /// price of the ticker from the provider.
    #[prost(string, tag = "2")]
    pub off_chain_ticker: ::prost::alloc::string::String,
    /// NormalizeByPair is the currency pair for this ticker to be normalized by.
    /// For example, if the desired Ticker is BTC/USD, this market could be reached
    /// using: OffChainTicker = BTC/USDT NormalizeByPair = USDT/USD This field is
    /// optional and nullable.
    #[prost(message, optional, tag = "3")]
    pub normalize_by_pair: ::core::option::Option<super::super::types::v1::CurrencyPair>,
    /// Invert is a boolean indicating if the BASE and QUOTE of the market should
    /// be inverted. i.e. BASE -> QUOTE, QUOTE -> BASE
    #[prost(bool, tag = "4")]
    pub invert: bool,
    /// MetadataJSON is a string of JSON that encodes any extra configuration
    /// for the given provider config.
    #[prost(string, tag = "15")]
    pub metadata_json: ::prost::alloc::string::String,
}
/// MarketMap maps ticker strings to their Markets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.MarketMap")]
pub struct MarketMap {
    /// Markets is the full list of tickers and their associated configurations
    /// to be stored on-chain.
    #[prost(map = "string, message", tag = "1")]
    pub markets: ::std::collections::HashMap<::prost::alloc::string::String, Market>,
}
/// Params defines the parameters for the x/marketmap module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.Params")]
pub struct Params {
    /// MarketAuthorities is the list of authority accounts that are able to
    /// control updating the marketmap.
    #[prost(string, repeated, tag = "1")]
    pub market_authorities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Admin is an address that can remove addresses from the MarketAuthorities
    /// list. Only governance can add to the MarketAuthorities or change the Admin.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
}
/// GenesisState defines the x/marketmap module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.GenesisState")]
pub struct GenesisState {
    /// MarketMap defines the global set of market configurations for all providers
    /// and markets.
    #[prost(message, optional, tag = "1")]
    pub market_map: ::core::option::Option<MarketMap>,
    /// LastUpdated is the last block height that the market map was updated.
    /// This field can be used as an optimization for clients checking if there
    /// is a new update to the map.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_updated: u64,
    /// Params are the parameters for the x/marketmap module.
    #[prost(message, optional, tag = "3")]
    pub params: ::core::option::Option<Params>,
}
/// MarketMapRequest is the query request for the MarketMap query.
/// It takes no arguments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.MarketMapRequest")]
#[proto_query(
    path = "/slinky.marketmap.v1.Query/MarketMap",
    response_type = MarketMapResponse
)]
pub struct MarketMapRequest {}
/// MarketMapResponse is the query response for the MarketMap query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.MarketMapResponse")]
pub struct MarketMapResponse {
    /// MarketMap defines the global set of market configurations for all providers
    /// and markets.
    #[prost(message, optional, tag = "1")]
    pub market_map: ::core::option::Option<MarketMap>,
    /// LastUpdated is the last block height that the market map was updated.
    /// This field can be used as an optimization for clients checking if there
    /// is a new update to the map.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_updated: u64,
    /// ChainId is the chain identifier for the market map.
    #[prost(string, tag = "3")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
}
/// MarketRequest is the query request for the Market query.
/// It takes the currency pair of the market as an argument.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.MarketRequest")]
#[proto_query(
    path = "/slinky.marketmap.v1.Query/Market",
    response_type = MarketResponse
)]
pub struct MarketRequest {
    /// CurrencyPair is the currency pair associated with the market being
    /// requested.
    #[prost(message, optional, tag = "1")]
    pub currency_pair: ::core::option::Option<super::super::types::v1::CurrencyPair>,
}
/// MarketResponse is the query response for the Market query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.MarketResponse")]
pub struct MarketResponse {
    /// Market is the configuration of a single market to be price-fetched for.
    #[prost(message, optional, tag = "1")]
    pub market: ::core::option::Option<Market>,
}
/// ParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.ParamsRequest")]
#[proto_query(
    path = "/slinky.marketmap.v1.Query/Params",
    response_type = ParamsResponse
)]
pub struct ParamsRequest {}
/// ParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.ParamsResponse")]
pub struct ParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// LastUpdatedRequest is the request type for the Query/LastUpdated RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.LastUpdatedRequest")]
#[proto_query(
    path = "/slinky.marketmap.v1.Query/LastUpdated",
    response_type = LastUpdatedResponse
)]
pub struct LastUpdatedRequest {}
/// LastUpdatedResponse is the response type for the Query/LastUpdated RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.LastUpdatedResponse")]
pub struct LastUpdatedResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_updated: u64,
}
/// MsgCreateMarkets defines a message carrying a payload for creating markets in
/// the x/marketmap module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.MsgCreateMarkets")]
pub struct MsgCreateMarkets {
    /// Authority is the signer of this transaction.  This authority must be
    /// authorized by the module to execute the message.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// CreateMarkets is the list of all markets to be created for the given
    /// transaction.
    #[prost(message, repeated, tag = "2")]
    pub create_markets: ::prost::alloc::vec::Vec<Market>,
}
/// MsgUpdateMarketMapResponse is the response message for MsgUpdateMarketMap.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.MsgCreateMarketsResponse")]
pub struct MsgCreateMarketsResponse {}
/// MsgUpdateMarkets defines a message carrying a payload for updating the
/// x/marketmap module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.MsgUpdateMarkets")]
pub struct MsgUpdateMarkets {
    /// Authority is the signer of this transaction.  This authority must be
    /// authorized by the module to execute the message.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// UpdateMarkets is the list of all markets to be updated for the given
    /// transaction.
    #[prost(message, repeated, tag = "2")]
    pub update_markets: ::prost::alloc::vec::Vec<Market>,
}
/// MsgUpdateMarketsResponse is the response message for MsgUpdateMarkets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.MsgUpdateMarketsResponse")]
pub struct MsgUpdateMarketsResponse {}
/// MsgParams defines the Msg/Params request type. It contains the
/// new parameters for the x/marketmap module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.MsgParams")]
pub struct MsgParams {
    /// Params defines the new parameters for the x/marketmap module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// Authority defines the authority that is updating the x/marketmap module
    /// parameters.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgParamsResponse defines the Msg/Params response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.MsgParamsResponse")]
pub struct MsgParamsResponse {}
/// MsgRemoveMarketAuthorities defines the Msg/RemoveMarketAuthoritiesResponse
/// request type. It contains the new addresses to remove from the list of
/// authorities
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.MsgRemoveMarketAuthorities")]
pub struct MsgRemoveMarketAuthorities {
    /// RemoveAddresses is the list of addresses to remove.
    #[prost(string, repeated, tag = "1")]
    pub remove_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Admin defines the authority that is the x/marketmap
    /// Admin account.  This account is set in the module parameters.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
}
/// MsgRemoveMarketAuthoritiesResponse defines the
/// Msg/RemoveMarketAuthoritiesResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/slinky.marketmap.v1.MsgRemoveMarketAuthoritiesResponse")]
pub struct MsgRemoveMarketAuthoritiesResponse {}
pub struct MarketmapQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> MarketmapQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn market_map(&self) -> Result<MarketMapResponse, cosmwasm_std::StdError> {
        MarketMapRequest {}.query(self.querier)
    }
    pub fn market(
        &self,
        currency_pair: ::core::option::Option<super::super::types::v1::CurrencyPair>,
    ) -> Result<MarketResponse, cosmwasm_std::StdError> {
        MarketRequest { currency_pair }.query(self.querier)
    }
    pub fn last_updated(&self) -> Result<LastUpdatedResponse, cosmwasm_std::StdError> {
        LastUpdatedRequest {}.query(self.querier)
    }
    pub fn params(&self) -> Result<ParamsResponse, cosmwasm_std::StdError> {
        ParamsRequest {}.query(self.querier)
    }
}
