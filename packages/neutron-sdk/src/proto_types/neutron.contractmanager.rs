// @generated
/// Failure message contains information about ACK failures and can be used to
/// replay ACK in case of requirement.
/// Note that Failure means that sudo handler to cosmwasm contract failed for
/// some reason
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Failure {
    /// Address of the failed contract
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Id of the failure under specific address
    #[prost(uint64, tag = "2")]
    pub id: u64,
    /// Serialized MessageSudoCallback with Packet and Ack(if exists)
    #[prost(bytes = "vec", tag = "3")]
    pub sudo_payload: ::prost::alloc::vec::Vec<u8>,
    /// Redacted error response of the sudo call. Full error is emitted as an event
    #[prost(string, tag = "4")]
    pub error: ::prost::alloc::string::String,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, tag = "1")]
    pub sudo_call_gas_limit: u64,
}
/// GenesisState defines the contractmanager module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// List of the contract failures
    ///
    /// this line is used by starport scaffolding # genesis/proto/state
    #[prost(message, repeated, tag = "2")]
    pub failures_list: ::prost::alloc::vec::Vec<Failure>,
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
/// QueryFailuresRequest is request type for the Query/Failures RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFailuresRequest {
    /// address of the contract which Sudo call failed.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// ID of the failure for the given contract.
    #[prost(uint64, tag = "2")]
    pub failure_id: u64,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryFailuresResponse is response type for the Query/Failures RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFailuresResponse {
    #[prost(message, repeated, tag = "1")]
    pub failures: ::prost::alloc::vec::Vec<Failure>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgUpdateParams is the MsgUpdateParams request type.
///
/// Since: 0.47
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/contractmanager parameters to update.
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
