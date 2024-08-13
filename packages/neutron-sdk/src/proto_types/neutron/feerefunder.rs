use neutron_std_derive::CosmwasmExt;
/// Fee defines the ICS29 receive, acknowledgement and timeout fees
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
#[proto_message(type_url = "/neutron.feerefunder.Fee")]
pub struct Fee {
    /// the packet receive fee
    #[prost(message, repeated, tag = "1")]
    pub recv_fee: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// the packet acknowledgement fee
    #[prost(message, repeated, tag = "2")]
    pub ack_fee: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// the packet timeout fee
    #[prost(message, repeated, tag = "3")]
    pub timeout_fee: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/neutron.feerefunder.PacketID")]
pub struct PacketId {
    #[prost(string, tag = "1")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
}
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
#[proto_message(type_url = "/neutron.feerefunder.Params")]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub min_fee: ::core::option::Option<Fee>,
}
/// GenesisState defines the fee module's genesis state.
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
#[proto_message(type_url = "/neutron.feerefunder.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// this line is used by starport scaffolding # genesis/proto/state
    #[prost(message, repeated, tag = "2")]
    pub fee_infos: ::prost::alloc::vec::Vec<FeeInfo>,
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
#[proto_message(type_url = "/neutron.feerefunder.FeeInfo")]
pub struct FeeInfo {
    #[prost(string, tag = "1")]
    pub payer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    #[serde(alias = "packetID")]
    pub packet_id: ::core::option::Option<PacketId>,
    #[prost(message, optional, tag = "3")]
    pub fee: ::core::option::Option<Fee>,
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
#[proto_message(type_url = "/neutron.feerefunder.QueryParamsRequest")]
#[proto_query(
    path = "/neutron.feerefunder.Query/Params",
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
#[proto_message(type_url = "/neutron.feerefunder.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
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
#[proto_message(type_url = "/neutron.feerefunder.FeeInfoRequest")]
#[proto_query(
    path = "/neutron.feerefunder.Query/FeeInfo",
    response_type = FeeInfoResponse
)]
pub struct FeeInfoRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
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
#[proto_message(type_url = "/neutron.feerefunder.FeeInfoResponse")]
pub struct FeeInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub fee_info: ::core::option::Option<FeeInfo>,
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
#[proto_message(type_url = "/neutron.feerefunder.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/feerefunder parameters to update.
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
#[proto_message(type_url = "/neutron.feerefunder.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct FeerefunderQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> FeerefunderQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn fee_info(
        &self,
        channel_id: ::prost::alloc::string::String,
        port_id: ::prost::alloc::string::String,
        sequence: u64,
    ) -> Result<FeeInfoResponse, cosmwasm_std::StdError> {
        FeeInfoRequest {
            channel_id,
            port_id,
            sequence,
        }
        .query(self.querier)
    }
}
