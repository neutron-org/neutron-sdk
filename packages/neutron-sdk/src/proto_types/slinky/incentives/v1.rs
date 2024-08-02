use neutron_std_derive::CosmwasmExt;
/// BadPriceIncentive is a message that contains the information about a bad
/// price that was submitted by a validator.
///
/// NOTE: This is an example of a bad price incentive. It is not used in
/// production.
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
#[proto_message(type_url = "/slinky.incentives.v1.BadPriceIncentive")]
pub struct BadPriceIncentive {
    /// Validator is the address of the validator that submitted the bad price.
    #[prost(string, tag = "1")]
    pub validator: ::prost::alloc::string::String,
    /// Amount is the amount to slash.
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// GoodPriceIncentive is a message that contains the information about a good
/// price that was submitted by a validator.
///
/// NOTE: This is an example of a good price incentive. It is not used in
/// production.
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
#[proto_message(type_url = "/slinky.incentives.v1.GoodPriceIncentive")]
pub struct GoodPriceIncentive {
    /// Validator is the address of the validator that submitted the good price.
    #[prost(string, tag = "1")]
    pub validator: ::prost::alloc::string::String,
    /// Amount is the amount to reward.
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// GenesisState is the genesis-state for the x/incentives module.
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
#[proto_message(type_url = "/slinky.incentives.v1.GenesisState")]
pub struct GenesisState {
    /// Registry is a list of incentives by type. The registry defined here
    /// should be a subset of the incentive types defined in the incentive
    /// module (keeper).
    #[prost(message, repeated, tag = "1")]
    pub registry: ::prost::alloc::vec::Vec<IncentivesByType>,
}
/// IncentivesByType encapsulates a list of incentives by type. Each of the
/// entries here must correspond to the same incentive type defined here.
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
#[proto_message(type_url = "/slinky.incentives.v1.IncentivesByType")]
pub struct IncentivesByType {
    /// IncentiveType is the incentive type i.e. (BadPriceIncentiveType,
    /// GoodPriceIncentiveType).
    #[prost(string, tag = "1")]
    pub incentive_type: ::prost::alloc::string::String,
    /// Entries is a list of incentive bytes.
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// GetIncentivesByTypeRequest is the request type for the
/// Query/GetIncentivesByType RPC method.
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
#[proto_message(type_url = "/slinky.incentives.v1.GetIncentivesByTypeRequest")]
#[proto_query(
    path = "/slinky.incentives.v1.Query/GetIncentivesByType",
    response_type = GetIncentivesByTypeResponse
)]
pub struct GetIncentivesByTypeRequest {
    /// IncentiveType is the incentive type i.e. (BadPriceIncentiveType,
    /// GoodPriceIncentiveType).
    #[prost(string, tag = "1")]
    pub incentive_type: ::prost::alloc::string::String,
}
/// GetIncentivesByTypeResponse is the response type for the
/// Query/GetIncentivesByType RPC method.
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
#[proto_message(type_url = "/slinky.incentives.v1.GetIncentivesByTypeResponse")]
pub struct GetIncentivesByTypeResponse {
    /// Entries is the list of incentives of the given type.
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// GetAllIncentivesRequest is the request type for the Query/GetAllIncentives
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
#[proto_message(type_url = "/slinky.incentives.v1.GetAllIncentivesRequest")]
#[proto_query(
    path = "/slinky.incentives.v1.Query/GetAllIncentives",
    response_type = GetAllIncentivesResponse
)]
pub struct GetAllIncentivesRequest {}
/// GetAllIncentivesResponse is the response type for the Query/GetAllIncentives
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
#[proto_message(type_url = "/slinky.incentives.v1.GetAllIncentivesResponse")]
pub struct GetAllIncentivesResponse {
    /// Registry is the list of all incentives, grouped by type.
    #[prost(message, repeated, tag = "1")]
    pub registry: ::prost::alloc::vec::Vec<IncentivesByType>,
}
pub struct IncentivesQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> IncentivesQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn get_incentives_by_type(
        &self,
        incentive_type: ::prost::alloc::string::String,
    ) -> Result<GetIncentivesByTypeResponse, cosmwasm_std::StdError> {
        GetIncentivesByTypeRequest { incentive_type }.query(self.querier)
    }
    pub fn get_all_incentives(&self) -> Result<GetAllIncentivesResponse, cosmwasm_std::StdError> {
        GetAllIncentivesRequest {}.query(self.querier)
    }
}
