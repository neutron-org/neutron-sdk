use neutron_std_derive::CosmwasmExt;
/// Params defines the parameters for the module.
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
#[proto_message(type_url = "/neutron.feeburner.Params")]
pub struct Params {
    /// Defines Neutron denom, which will be burned during fee processing, any
    /// other denom will be sent to Treasury
    #[prost(string, tag = "1")]
    pub neutron_denom: ::prost::alloc::string::String,
    /// Deprecated in v0.4.4. Is not used anymore
    #[prost(string, tag = "2")]
    pub reserve_address: ::prost::alloc::string::String,
    /// Defines treasury address
    #[prost(string, tag = "3")]
    pub treasury_address: ::prost::alloc::string::String,
}
/// TotalBurnedNeutronsAmount defines total amount of burned neutron fees
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
#[proto_message(type_url = "/neutron.feeburner.TotalBurnedNeutronsAmount")]
pub struct TotalBurnedNeutronsAmount {
    #[prost(message, optional, tag = "1")]
    pub coin: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the feeburner module's genesis state.
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
#[proto_message(type_url = "/neutron.feeburner.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// this line is used by starport scaffolding # genesis/proto/state
    #[prost(message, optional, tag = "2")]
    pub total_burned_neutrons_amount: ::core::option::Option<TotalBurnedNeutronsAmount>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
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
#[proto_message(type_url = "/neutron.feeburner.QueryParamsRequest")]
#[proto_query(
    path = "/neutron.feeburner.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/neutron.feeburner.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryTotalBurnedNeutronsAmountRequest is request type for the
/// Query/QueryTotalBurnedNeutronsAmount method.
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
#[proto_message(type_url = "/neutron.feeburner.QueryTotalBurnedNeutronsAmountRequest")]
#[proto_query(
    path = "/neutron.feeburner.Query/TotalBurnedNeutronsAmount",
    response_type = QueryTotalBurnedNeutronsAmountResponse
)]
pub struct QueryTotalBurnedNeutronsAmountRequest {}
/// QueryTotalBurnedNeutronsAmountResponse is response type for the
/// Query/QueryTotalBurnedNeutronsAmount method.
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
#[proto_message(type_url = "/neutron.feeburner.QueryTotalBurnedNeutronsAmountResponse")]
pub struct QueryTotalBurnedNeutronsAmountResponse {
    #[prost(message, optional, tag = "1")]
    pub total_burned_neutrons_amount: ::core::option::Option<TotalBurnedNeutronsAmount>,
}
/// MsgUpdateParams is the MsgUpdateParams request type.
///
/// Since: 0.47
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
#[proto_message(type_url = "/neutron.feeburner.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/feeburner parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: 0.47
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
#[proto_message(type_url = "/neutron.feeburner.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct FeeburnerQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> FeeburnerQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn total_burned_neutrons_amount(
        &self,
    ) -> Result<QueryTotalBurnedNeutronsAmountResponse, cosmwasm_std::StdError> {
        QueryTotalBurnedNeutronsAmountRequest {}.query(self.querier)
    }
}
