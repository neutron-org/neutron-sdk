pub mod v1;
use neutron_std_derive::CosmwasmExt;
/// Failure message contains information about ACK failures and can be used to
/// replay ACK in case of requirement.
/// Note that Failure means that sudo handler to cosmwasm contract failed for
/// some reason
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
#[proto_message(type_url = "/neutron.contractmanager.Failure")]
pub struct Failure {
    /// Address of the failed contract
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Id of the failure under specific address
    #[prost(uint64, tag = "2")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// Serialized MessageSudoCallback with Packet and Ack(if exists)
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub sudo_payload: ::prost::alloc::vec::Vec<u8>,
    /// Redacted error response of the sudo call. Full error is emitted as an event
    #[prost(string, tag = "4")]
    pub error: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/neutron.contractmanager.Params")]
pub struct Params {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sudo_call_gas_limit: u64,
}
/// GenesisState defines the contractmanager module's genesis state.
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
#[proto_message(type_url = "/neutron.contractmanager.GenesisState")]
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
#[proto_message(type_url = "/neutron.contractmanager.QueryParamsRequest")]
#[proto_query(
    path = "/neutron.contractmanager.Query/Params",
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
#[proto_message(type_url = "/neutron.contractmanager.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryFailuresRequest is request type for the Query/Failures RPC method.
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
#[proto_message(type_url = "/neutron.contractmanager.QueryFailuresRequest")]
#[proto_query(
    path = "/neutron.contractmanager.Query/AddressFailures",
    response_type = QueryFailuresResponse
)]
pub struct QueryFailuresRequest {
    /// address of the contract which Sudo call failed.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryFailureRequest is request type for the Query/Failures RPC method.
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
#[proto_message(type_url = "/neutron.contractmanager.QueryFailureRequest")]
#[proto_query(
    path = "/neutron.contractmanager.Query/AddressFailure",
    response_type = QueryFailureResponse
)]
pub struct QueryFailureRequest {
    /// address of the contract which Sudo call failed.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// ID of the failure for the given contract.
    #[prost(uint64, tag = "2")]
    #[serde(alias = "failureID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub failure_id: u64,
}
/// QueryFailureResponse is response type for the Query/Failure RPC method.
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
#[proto_message(type_url = "/neutron.contractmanager.QueryFailureResponse")]
pub struct QueryFailureResponse {
    #[prost(message, optional, tag = "1")]
    pub failure: ::core::option::Option<Failure>,
}
/// QueryFailuresResponse is response type for the Query/Failures RPC method.
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
#[proto_message(type_url = "/neutron.contractmanager.QueryFailuresResponse")]
pub struct QueryFailuresResponse {
    #[prost(message, repeated, tag = "1")]
    pub failures: ::prost::alloc::vec::Vec<Failure>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
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
#[proto_message(type_url = "/neutron.contractmanager.MsgUpdateParams")]
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
#[proto_message(type_url = "/neutron.contractmanager.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct ContractmanagerQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ContractmanagerQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn address_failure(
        &self,
        address: ::prost::alloc::string::String,
        failure_id: u64,
    ) -> Result<QueryFailureResponse, cosmwasm_std::StdError> {
        QueryFailureRequest {
            address,
            failure_id,
        }
        .query(self.querier)
    }
    pub fn address_failures(
        &self,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryFailuresResponse, cosmwasm_std::StdError> {
        QueryFailuresRequest {
            address,
            pagination,
        }
        .query(self.querier)
    }
    pub fn failures(
        &self,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryFailuresResponse, cosmwasm_std::StdError> {
        QueryFailuresRequest {
            address,
            pagination,
        }
        .query(self.querier)
    }
}
