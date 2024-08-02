use neutron_std_derive::CosmwasmExt;
/// QueryPricesRequest defines the request type for the the Prices method.
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
#[proto_message(type_url = "/slinky.service.v1.QueryPricesRequest")]
pub struct QueryPricesRequest {}
/// QueryPricesResponse defines the response type for the Prices method.
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
#[proto_message(type_url = "/slinky.service.v1.QueryPricesResponse")]
pub struct QueryPricesResponse {
    /// prices defines the list of prices.
    #[prost(map = "string, string", tag = "1")]
    pub prices:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
}
/// QueryMarketMapRequest defines the request type for the MarketMap method.
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
#[proto_message(type_url = "/slinky.service.v1.QueryMarketMapRequest")]
pub struct QueryMarketMapRequest {}
/// QueryMarketMapResponse defines the response type for the MarketMap method.
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
#[proto_message(type_url = "/slinky.service.v1.QueryMarketMapResponse")]
pub struct QueryMarketMapResponse {
    /// market_map defines the current market map configuration.
    #[prost(message, optional, tag = "1")]
    pub market_map: ::core::option::Option<super::super::marketmap::v1::MarketMap>,
}
