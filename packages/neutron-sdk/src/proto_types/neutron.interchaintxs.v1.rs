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
/// MsgRegisterInterchainAccount is used to register an account on a remote zone.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterInterchainAccount {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub interchain_account_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub register_fee: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// MsgRegisterInterchainAccountResponse is the response type for
/// MsgRegisterInterchainAccount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterInterchainAccountResponse {
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub port_id: ::prost::alloc::string::String,
}
/// MsgSubmitTx defines the payload for Msg/SubmitTx
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitTx {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    /// interchain_account_id is supposed to be the unique identifier, e.g.,
    /// lido/kava. This allows contracts to have more than one interchain accounts
    /// on remote zone This identifier will be a part of the portID that we'll
    /// claim our capability for.
    #[prost(string, tag = "2")]
    pub interchain_account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub msgs: ::prost::alloc::vec::Vec<::prost_types::Any>,
    #[prost(string, tag = "5")]
    pub memo: ::prost::alloc::string::String,
    /// timeout in seconds after which the packet times out
    #[prost(uint64, tag = "6")]
    pub timeout: u64,
    #[prost(message, optional, tag = "7")]
    pub fee: ::core::option::Option<super::super::feerefunder::Fee>,
}
/// MsgSubmitTxResponse defines the response for Msg/SubmitTx
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitTxResponse {
    /// channel's sequence_id for outgoing ibc packet. Unique per a channel.
    #[prost(uint64, tag = "1")]
    pub sequence_id: u64,
    /// channel src channel on neutron side transaction was submitted from
    #[prost(string, tag = "2")]
    pub channel: ::prost::alloc::string::String,
}
/// MsgUpdateParams is the MsgUpdateParams request type.
///
/// Since: 0.47
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/interchaintxs parameters to update.
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
