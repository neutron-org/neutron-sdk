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
#[proto_message(type_url = "/neutron.interchaintxs.v1.Params")]
pub struct Params {
    /// Defines maximum amount of messages to be passed in MsgSubmitTx
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub msg_submit_tx_max_messages: u64,
    /// Defines a minimum fee required to register interchain account
    #[prost(message, repeated, tag = "2")]
    pub register_fee: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the interchaintxs module's genesis state.
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
#[proto_message(type_url = "/neutron.interchaintxs.v1.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
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
#[proto_message(type_url = "/neutron.interchaintxs.v1.QueryParamsRequest")]
#[proto_query(
    path = "/neutron.interchaintxs.v1.Query/Params",
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
#[proto_message(type_url = "/neutron.interchaintxs.v1.QueryParamsResponse")]
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
#[proto_message(type_url = "/neutron.interchaintxs.v1.QueryInterchainAccountAddressRequest")]
#[proto_query(
    path = "/neutron.interchaintxs.v1.Query/InterchainAccountAddress",
    response_type = QueryInterchainAccountAddressResponse
)]
pub struct QueryInterchainAccountAddressRequest {
    /// owner_address is the owner of the interchain account on the controller
    /// chain
    #[prost(string, tag = "1")]
    pub owner_address: ::prost::alloc::string::String,
    /// interchain_account_id is an identifier of your interchain account from
    /// which you want to execute msgs
    #[prost(string, tag = "2")]
    #[serde(alias = "interchain_accountID")]
    pub interchain_account_id: ::prost::alloc::string::String,
    /// connection_id is an IBC connection identifier between Neutron and remote
    /// chain
    #[prost(string, tag = "3")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
}
/// Query response for an interchain account address
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
#[proto_message(type_url = "/neutron.interchaintxs.v1.QueryInterchainAccountAddressResponse")]
pub struct QueryInterchainAccountAddressResponse {
    /// The corresponding interchain account address on the host chain
    #[prost(string, tag = "1")]
    pub interchain_account_address: ::prost::alloc::string::String,
}
/// MsgRegisterInterchainAccount is used to register an account on a remote zone.
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
#[proto_message(type_url = "/neutron.interchaintxs.v1.MsgRegisterInterchainAccount")]
pub struct MsgRegisterInterchainAccount {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    #[serde(alias = "interchain_accountID")]
    pub interchain_account_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub register_fee: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(
        enumeration = "super::super::super::ibc::core::channel::v1::Order",
        tag = "5"
    )]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ordering: i32,
}
/// MsgRegisterInterchainAccountResponse is the response type for
/// MsgRegisterInterchainAccount.
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
#[proto_message(type_url = "/neutron.interchaintxs.v1.MsgRegisterInterchainAccountResponse")]
pub struct MsgRegisterInterchainAccountResponse {
    #[prost(string, tag = "1")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
}
/// MsgSubmitTx defines the payload for Msg/SubmitTx
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
#[proto_message(type_url = "/neutron.interchaintxs.v1.MsgSubmitTx")]
pub struct MsgSubmitTx {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    /// interchain_account_id is supposed to be the unique identifier, e.g.,
    /// lido/kava. This allows contracts to have more than one interchain accounts
    /// on remote zone This identifier will be a part of the portID that we'll
    /// claim our capability for.
    #[prost(string, tag = "2")]
    #[serde(alias = "interchain_accountID")]
    pub interchain_account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub msgs: ::prost::alloc::vec::Vec<crate::shim::Any>,
    #[prost(string, tag = "5")]
    pub memo: ::prost::alloc::string::String,
    /// timeout in seconds after which the packet times out
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timeout: u64,
    #[prost(message, optional, tag = "7")]
    pub fee: ::core::option::Option<super::super::feerefunder::Fee>,
}
/// MsgSubmitTxResponse defines the response for Msg/SubmitTx
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
#[proto_message(type_url = "/neutron.interchaintxs.v1.MsgSubmitTxResponse")]
pub struct MsgSubmitTxResponse {
    /// channel's sequence_id for outgoing ibc packet. Unique per a channel.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "sequenceID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence_id: u64,
    /// channel src channel on neutron side transaction was submitted from
    #[prost(string, tag = "2")]
    pub channel: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/neutron.interchaintxs.v1.MsgUpdateParams")]
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
#[proto_message(type_url = "/neutron.interchaintxs.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct InterchaintxsQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> InterchaintxsQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn interchain_account_address(
        &self,
        owner_address: ::prost::alloc::string::String,
        interchain_account_id: ::prost::alloc::string::String,
        connection_id: ::prost::alloc::string::String,
    ) -> Result<QueryInterchainAccountAddressResponse, cosmwasm_std::StdError> {
        QueryInterchainAccountAddressRequest {
            owner_address,
            interchain_account_id,
            connection_id,
        }
        .query(self.querier)
    }
}
