use neutron_std_derive::CosmwasmExt;
/// GenesisState defines the sla module's genesis state.
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
#[proto_message(type_url = "/slinky.sla.v1.GenesisState")]
pub struct GenesisState {
    /// SLAs are the SLAs that are currently active.
    #[prost(message, repeated, tag = "1")]
    pub slas: ::prost::alloc::vec::Vec<PriceFeedSla>,
    /// PrceFeeds are the price feeds that are currently active.
    #[prost(message, repeated, tag = "2")]
    pub price_feeds: ::prost::alloc::vec::Vec<PriceFeed>,
    /// Params are the parameters for the sla module.
    #[prost(message, optional, tag = "3")]
    pub params: ::core::option::Option<Params>,
}
/// Params defines the parameters for the sla module.
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
#[proto_message(type_url = "/slinky.sla.v1.Params")]
pub struct Params {
    /// Enabled is a flag to enable or disable the sla module.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// PriceFeedSLA defines the the desired SLA for a given set of price feeds. A
/// price feed is defined to be a set of price prices for the same (currency
/// pair, validator).
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
#[proto_message(type_url = "/slinky.sla.v1.PriceFeedSLA")]
pub struct PriceFeedSla {
    /// MaximumViableWindow is the maximum time window that we are interested
    /// for the SLA. This is used to determine the moving window of blocks that
    /// we are interested in.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub maximum_viable_window: u64,
    /// ExpectedUptime is the expected uptime for the given validator and price
    /// feed.
    #[prost(string, tag = "2")]
    pub expected_uptime: ::prost::alloc::string::String,
    /// SlashConstant is the constant by which we will multiply the deviation from
    /// the expected uptime.
    #[prost(string, tag = "3")]
    pub slash_constant: ::prost::alloc::string::String,
    /// MinimumBlockUpdates is the minimum number of blocks that the
    /// validator had to have voted on in the maximum viable window
    /// in order to be considered for the SLA.
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub minimum_block_updates: u64,
    /// Frequency is the frequency at which we will check the SLA.
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub frequency: u64,
    /// ID is the unique identifier for the SLA.
    #[prost(string, tag = "6")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
}
/// PriceFeed defines the object type that will be utilized to monitor how
/// frequently validators are voting with price updates across the network.
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
#[proto_message(type_url = "/slinky.sla.v1.PriceFeed")]
pub struct PriceFeed {
    /// UpdateMap represents the relevant moving window of price feed updates.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub update_map: ::prost::alloc::vec::Vec<u8>,
    /// InclusionMap represents the relevant moving window of blocks that the
    /// validator has voted on.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub inclusion_map: ::prost::alloc::vec::Vec<u8>,
    /// Index corresponds to the current index into the bitmap.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub index: u64,
    /// Validator represents the validator that this SLA corresponds to.
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub validator: ::prost::alloc::vec::Vec<u8>,
    /// CurrencyPair represents the currency pair that this SLA corresponds to.
    #[prost(message, optional, tag = "5")]
    pub currency_pair: ::core::option::Option<super::super::types::v1::CurrencyPair>,
    /// MaximumViableWindow represents the maximum number of blocks that can be
    /// represented by the bit map.
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub maximum_viable_window: u64,
    /// ID corresponds to the SLA ID that this price feed corresponds to.
    #[prost(string, tag = "7")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
}
/// QueryAllSLAsRequest is the request type for the Query/GetAllSLAs RPC method.
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
#[proto_message(type_url = "/slinky.sla.v1.GetAllSLAsRequest")]
#[proto_query(
    path = "/slinky.sla.v1.Query/GetAllSLAs",
    response_type = GetAllSlAsResponse
)]
pub struct GetAllSlAsRequest {}
/// QueryAllSLAsResponse is the response type for the Query/GetAllSLAs RPC
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
#[proto_message(type_url = "/slinky.sla.v1.GetAllSLAsResponse")]
pub struct GetAllSlAsResponse {
    #[prost(message, repeated, tag = "1")]
    pub slas: ::prost::alloc::vec::Vec<PriceFeedSla>,
}
/// QueryGetPriceFeedsRequest is the request type for the Query/GetPriceFeeds RPC
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
#[proto_message(type_url = "/slinky.sla.v1.GetPriceFeedsRequest")]
#[proto_query(
    path = "/slinky.sla.v1.Query/GetPriceFeeds",
    response_type = GetPriceFeedsResponse
)]
pub struct GetPriceFeedsRequest {
    /// ID defines the SLA to query price feeds for.
    #[prost(string, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
}
/// QueryGetPriceFeedsResponse is the response type for the Query/GetPriceFeeds
/// RPC method.
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
#[proto_message(type_url = "/slinky.sla.v1.GetPriceFeedsResponse")]
pub struct GetPriceFeedsResponse {
    /// PriceFeeds defines the price feeds for the given SLA.
    #[prost(message, repeated, tag = "1")]
    pub price_feeds: ::prost::alloc::vec::Vec<PriceFeed>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
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
#[proto_message(type_url = "/slinky.sla.v1.ParamsRequest")]
#[proto_query(path = "/slinky.sla.v1.Query/Params", response_type = ParamsResponse)]
pub struct ParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/slinky.sla.v1.ParamsResponse")]
pub struct ParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgAddSLAs defines the Msg/AddSLAs request type. It contains the
/// SLAs to be added to the store.
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
#[proto_message(type_url = "/slinky.sla.v1.MsgAddSLAs")]
pub struct MsgAddSlAs {
    /// SLAs defines the SLAs to be added to the store.
    #[prost(message, repeated, tag = "1")]
    pub slas: ::prost::alloc::vec::Vec<PriceFeedSla>,
    /// Authority defines the authority that is adding the SLAs.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgAddSLAsResponse defines the Msg/AddSLAs response type.
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
#[proto_message(type_url = "/slinky.sla.v1.MsgAddSLAsResponse")]
pub struct MsgAddSlAsResponse {}
/// MsgRemoveSLAs defines the Msg/RemoveSLAs request type. It contains the
/// IDs of the SLAs to be removed from the store.
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
#[proto_message(type_url = "/slinky.sla.v1.MsgRemoveSLAs")]
pub struct MsgRemoveSlAs {
    /// IDs defines the IDs of the SLAs to be removed from the store.
    #[prost(string, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Authority defines the authority that is removing the SLAs.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgRemoveSLAsResponse defines the Msg/RemoveSLAs response type.
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
#[proto_message(type_url = "/slinky.sla.v1.MsgRemoveSLAsResponse")]
pub struct MsgRemoveSlAsResponse {}
/// MsgParams defines the Msg/Params request type. It contains the
/// new parameters for the SLA module.
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
#[proto_message(type_url = "/slinky.sla.v1.MsgParams")]
pub struct MsgParams {
    /// Params defines the new parameters for the SLA module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// Authority defines the authority that is updating the SLA module parameters.
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
#[proto_message(type_url = "/slinky.sla.v1.MsgParamsResponse")]
pub struct MsgParamsResponse {}
pub struct SlaQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> SlaQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn get_all_sl_as(&self) -> Result<GetAllSlAsResponse, cosmwasm_std::StdError> {
        GetAllSlAsRequest {}.query(self.querier)
    }
    pub fn get_price_feeds(
        &self,
        id: ::prost::alloc::string::String,
    ) -> Result<GetPriceFeedsResponse, cosmwasm_std::StdError> {
        GetPriceFeedsRequest { id }.query(self.querier)
    }
    pub fn params(&self) -> Result<ParamsResponse, cosmwasm_std::StdError> {
        ParamsRequest {}.query(self.querier)
    }
}
