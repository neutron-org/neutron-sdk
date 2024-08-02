use neutron_std_derive::CosmwasmExt;
/// QuotePrice is the representation of the aggregated prices for a CurrencyPair,
/// where price represents the price of Base in terms of Quote
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
#[proto_message(type_url = "/slinky.oracle.v1.QuotePrice")]
pub struct QuotePrice {
    #[prost(string, tag = "1")]
    pub price: ::prost::alloc::string::String,
    /// BlockTimestamp tracks the block height associated with this price update.
    /// We include block timestamp alongside the price to ensure that smart
    /// contracts and applications are not utilizing stale oracle prices
    #[prost(message, optional, tag = "2")]
    pub block_timestamp: ::core::option::Option<crate::shim::Timestamp>,
    /// BlockHeight is height of block mentioned above
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_height: u64,
}
/// CurrencyPairState represents the stateful information tracked by the x/oracle
/// module per-currency-pair.
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
#[proto_message(type_url = "/slinky.oracle.v1.CurrencyPairState")]
pub struct CurrencyPairState {
    /// QuotePrice is the latest price for a currency-pair, notice this value can
    /// be null in the case that no price exists for the currency-pair
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<QuotePrice>,
    /// Nonce is the number of updates this currency-pair has received
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nonce: u64,
    /// ID is the ID of the CurrencyPair
    #[prost(uint64, tag = "3")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
/// CurrencyPairGenesis is the information necessary for initialization of a
/// CurrencyPair.
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
#[proto_message(type_url = "/slinky.oracle.v1.CurrencyPairGenesis")]
pub struct CurrencyPairGenesis {
    /// The CurrencyPair to be added to module state
    #[prost(message, optional, tag = "1")]
    pub currency_pair: ::core::option::Option<super::super::types::v1::CurrencyPair>,
    /// A genesis price if one exists (note this will be empty, unless it results
    /// from forking the state of this module)
    #[prost(message, optional, tag = "2")]
    pub currency_pair_price: ::core::option::Option<QuotePrice>,
    /// nonce is the nonce (number of updates) for the CP (same case as above,
    /// likely 0 unless it results from fork of module)
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nonce: u64,
    /// id is the ID of the CurrencyPair
    #[prost(uint64, tag = "4")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
/// GenesisState is the genesis-state for the x/oracle module, it takes a set of
/// predefined CurrencyPairGeneses
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
#[proto_message(type_url = "/slinky.oracle.v1.GenesisState")]
pub struct GenesisState {
    /// CurrencyPairGenesis is the set of CurrencyPairGeneses for the module. I.e
    /// the starting set of CurrencyPairs for the module + information regarding
    /// their latest update.
    #[prost(message, repeated, tag = "1")]
    pub currency_pair_genesis: ::prost::alloc::vec::Vec<CurrencyPairGenesis>,
    /// NextID is the next ID to be used for a CurrencyPair
    #[prost(uint64, tag = "2")]
    #[serde(alias = "nextID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_id: u64,
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
#[proto_message(type_url = "/slinky.oracle.v1.GetAllCurrencyPairsRequest")]
#[proto_query(
    path = "/slinky.oracle.v1.Query/GetAllCurrencyPairs",
    response_type = GetAllCurrencyPairsResponse
)]
pub struct GetAllCurrencyPairsRequest {}
/// GetAllCurrencyPairsResponse returns all CurrencyPairs that the module is
/// currently tracking.
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
#[proto_message(type_url = "/slinky.oracle.v1.GetAllCurrencyPairsResponse")]
pub struct GetAllCurrencyPairsResponse {
    #[prost(message, repeated, tag = "1")]
    pub currency_pairs: ::prost::alloc::vec::Vec<super::super::types::v1::CurrencyPair>,
}
/// GetPriceRequest either takes a CurrencyPair, or an identifier for the
/// CurrencyPair in the format base/quote.
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
#[proto_message(type_url = "/slinky.oracle.v1.GetPriceRequest")]
#[proto_query(
    path = "/slinky.oracle.v1.Query/GetPrice",
    response_type = GetPriceResponse
)]
pub struct GetPriceRequest {
    /// CurrencyPair represents the pair that the user wishes to query.
    #[prost(message, optional, tag = "1")]
    pub currency_pair: ::core::option::Option<super::super::types::v1::CurrencyPair>,
}
/// GetPriceResponse is the response from the GetPrice grpc method exposed from
/// the x/oracle query service.
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
#[proto_message(type_url = "/slinky.oracle.v1.GetPriceResponse")]
pub struct GetPriceResponse {
    /// QuotePrice represents the quote-price for the CurrencyPair given in
    /// GetPriceRequest (possibly nil if no update has been made)
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<QuotePrice>,
    /// nonce represents the nonce for the CurrencyPair if it exists in state
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nonce: u64,
    /// decimals represents the number of decimals that the quote-price is
    /// represented in. For Pairs where ETHEREUM is the quote this will be 18,
    /// otherwise it will be 8.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub decimals: u64,
    /// ID represents the identifier for the CurrencyPair.
    #[prost(uint64, tag = "4")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
/// GetPricesRequest takes an identifier for the CurrencyPair
/// in the format base/quote.
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
#[proto_message(type_url = "/slinky.oracle.v1.GetPricesRequest")]
#[proto_query(
    path = "/slinky.oracle.v1.Query/GetPrices",
    response_type = GetPricesResponse
)]
pub struct GetPricesRequest {
    #[prost(string, repeated, tag = "1")]
    #[serde(alias = "currency_pairIDs")]
    pub currency_pair_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetPricesResponse is the response from the GetPrices grpc method exposed from
/// the x/oracle query service.
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
#[proto_message(type_url = "/slinky.oracle.v1.GetPricesResponse")]
pub struct GetPricesResponse {
    #[prost(message, repeated, tag = "1")]
    pub prices: ::prost::alloc::vec::Vec<GetPriceResponse>,
}
/// GetCurrencyPairMappingRequest is the GetCurrencyPairMapping request type.
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
#[proto_message(type_url = "/slinky.oracle.v1.GetCurrencyPairMappingRequest")]
#[proto_query(
    path = "/slinky.oracle.v1.Query/GetCurrencyPairMapping",
    response_type = GetCurrencyPairMappingResponse
)]
pub struct GetCurrencyPairMappingRequest {}
/// GetCurrencyPairMappingResponse is the GetCurrencyPairMapping response type.
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
#[proto_message(type_url = "/slinky.oracle.v1.GetCurrencyPairMappingResponse")]
pub struct GetCurrencyPairMappingResponse {
    /// currency_pair_mapping is a mapping of the id representing the currency pair
    /// to the currency pair itself.
    #[prost(map = "uint64, message", tag = "1")]
    pub currency_pair_mapping:
        ::std::collections::HashMap<u64, super::super::types::v1::CurrencyPair>,
}
/// Given an authority + a set of CurrencyPairs, the x/oracle module will
/// check to see that the authority has permissions to update the set of
/// CurrencyPairs tracked in the oracle, and add the given CurrencyPairs to be
/// tracked in each VoteExtension
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
#[proto_message(type_url = "/slinky.oracle.v1.MsgAddCurrencyPairs")]
pub struct MsgAddCurrencyPairs {
    /// authority is the address of the account that is authorized to update the
    /// x/oracle's CurrencyPairs
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// set of CurrencyPairs to be added to the module (+ prices if they are to be
    /// set)
    #[prost(message, repeated, tag = "2")]
    pub currency_pairs: ::prost::alloc::vec::Vec<super::super::types::v1::CurrencyPair>,
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
#[proto_message(type_url = "/slinky.oracle.v1.MsgAddCurrencyPairsResponse")]
pub struct MsgAddCurrencyPairsResponse {}
/// Given an authority + a set of CurrencyPairIDs, the x/oracle module's message
/// service will remove all of the CurrencyPairs identified by each
/// CurrencyPairID in the request from state. Notice, if a given currency-pair
/// does not exist in state, the module ignores that currency-pair and continues
/// removing the rest.
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
#[proto_message(type_url = "/slinky.oracle.v1.MsgRemoveCurrencyPairs")]
pub struct MsgRemoveCurrencyPairs {
    /// authority is the address of the account that is authorized to update the
    /// x/oracle's CurrencyPairs
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// currency_pair_ids are the stringified representation of a currency-pairs
    /// (base/quote) to be removed from the module's state
    #[prost(string, repeated, tag = "2")]
    #[serde(alias = "currency_pairIDs")]
    pub currency_pair_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/slinky.oracle.v1.MsgRemoveCurrencyPairsResponse")]
pub struct MsgRemoveCurrencyPairsResponse {}
pub struct OracleQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> OracleQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn get_all_currency_pairs(
        &self,
    ) -> Result<GetAllCurrencyPairsResponse, cosmwasm_std::StdError> {
        GetAllCurrencyPairsRequest {}.query(self.querier)
    }
    pub fn get_price(
        &self,
        currency_pair: ::core::option::Option<super::super::types::v1::CurrencyPair>,
    ) -> Result<GetPriceResponse, cosmwasm_std::StdError> {
        GetPriceRequest { currency_pair }.query(self.querier)
    }
    pub fn get_prices(
        &self,
        currency_pair_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ) -> Result<GetPricesResponse, cosmwasm_std::StdError> {
        GetPricesRequest { currency_pair_ids }.query(self.querier)
    }
    pub fn get_currency_pair_mapping(
        &self,
    ) -> Result<GetCurrencyPairMappingResponse, cosmwasm_std::StdError> {
        GetCurrencyPairMappingRequest {}.query(self.querier)
    }
}
