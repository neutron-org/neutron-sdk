// @generated
/// Params defines the set of module parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// minimum_gas_prices stores the minimum gas price(s) for all TX on the chain.
    /// When multiple coins are defined then they are accepted alternatively.
    /// The list must be sorted by denoms asc. No duplicate denoms or zero amount
    /// values allowed. For more information see
    /// <https://docs.cosmos.network/main/modules/auth#concepts>
    #[prost(message, repeated, tag = "1")]
    pub minimum_gas_prices:
        ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::DecCoin>,
    /// bypass_min_fee_msg_types defines a list of message type urls
    /// that are free of fee charge.
    #[prost(string, repeated, tag = "2")]
    pub bypass_min_fee_msg_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// max_total_bypass_min_fee_msg_gas_usage defines the total maximum gas usage
    /// allowed for a transaction containing only messages of types in bypass_min_fee_msg_types
    /// to bypass fee charge.
    #[prost(uint64, tag = "3")]
    pub max_total_bypass_min_fee_msg_gas_usage: u64,
}
/// GenesisState - initial state of module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// Params of this module
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryMinimumGasPricesRequest is the request type for the
/// Query/MinimumGasPrices RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryMinimumGasPricesResponse is the response type for the
/// Query/MinimumGasPrices RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
// this line is used by starport scaffolding # proto/tx/message

/// MsgUpdateParams is the MsgUpdateParams request type.
///
/// Since: 0.47
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/globalfee parameters to update.
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
