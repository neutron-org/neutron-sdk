// @generated
/// Fee defines the ICS29 receive, acknowledgement and timeout fees
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fee {
    /// the packet receive fee
    #[prost(message, repeated, tag = "1")]
    pub recv_fee: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// the packet acknowledgement fee
    #[prost(message, repeated, tag = "2")]
    pub ack_fee: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// the packet timeout fee
    #[prost(message, repeated, tag = "3")]
    pub timeout_fee: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PacketId {
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub min_fee: ::core::option::Option<Fee>,
}
/// GenesisState defines the fee module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// this line is used by starport scaffolding # genesis/proto/state
    #[prost(message, repeated, tag = "2")]
    pub fee_infos: ::prost::alloc::vec::Vec<FeeInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeInfo {
    #[prost(string, tag = "1")]
    pub payer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub packet_id: ::core::option::Option<PacketId>,
    #[prost(message, optional, tag = "3")]
    pub fee: ::core::option::Option<Fee>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeInfoRequest {
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub fee_info: ::core::option::Option<FeeInfo>,
}
/// MsgUpdateParams is the MsgUpdateParams request type.
///
/// Since: 0.47
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
// @@protoc_insertion_point(module)
