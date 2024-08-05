// @generated
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// List of asset prices by the NTRN
    /// it's used in cooperation with feemarket module
    /// ntrn_prices is a data source to convert gas_price from feemarket's base_denom (untrn)
    /// into a given asset
    #[prost(message, repeated, tag = "1")]
    pub ntrn_prices: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::DecCoin>,
}
/// GenesisState defines the dynamicfees module's genesis state.
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
/// MsgUpdateParams is the MsgUpdateParams request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/dynamicfees parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
// @@protoc_insertion_point(module)
