// @generated
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// Defines maximum amount of messages to be passed in MsgSubmitTx
    #[prost(uint64, tag = "1")]
    pub msg_submit_tx_max_messages: u64,
    /// Defines a minimum fee required to register interchain account
    #[prost(message, repeated, tag = "2")]
    pub register_fee: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the interchaintxs module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
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
pub struct QueryInterchainAccountAddressRequest {
    /// owner_address is the owner of the interchain account on the controller
    /// chain
    #[prost(string, tag = "1")]
    pub owner_address: ::prost::alloc::string::String,
    /// interchain_account_id is an identifier of your interchain account from
    /// which you want to execute msgs
    #[prost(string, tag = "2")]
    pub interchain_account_id: ::prost::alloc::string::String,
    /// connection_id is an IBC connection identifier between Neutron and remote
    /// chain
    #[prost(string, tag = "3")]
    pub connection_id: ::prost::alloc::string::String,
}
/// Query response for an interchain account address
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInterchainAccountAddressResponse {
    /// The corresponding interchain account address on the host chain
    #[prost(string, tag = "1")]
    pub interchain_account_address: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
