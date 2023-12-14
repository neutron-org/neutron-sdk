// @generated
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalBurnedNeutronsAmount {
    #[prost(message, optional, tag = "1")]
    pub coin: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the feeburner module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// this line is used by starport scaffolding # genesis/proto/state
    #[prost(message, optional, tag = "2")]
    pub total_burned_neutrons_amount: ::core::option::Option<TotalBurnedNeutronsAmount>,
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
/// QueryTotalBurnedNeutronsAmountRequest is request type for the
/// Query/QueryTotalBurnedNeutronsAmount method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalBurnedNeutronsAmountRequest {}
/// QueryTotalBurnedNeutronsAmountResponse is response type for the
/// Query/QueryTotalBurnedNeutronsAmount method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalBurnedNeutronsAmountResponse {
    #[prost(message, optional, tag = "1")]
    pub total_burned_neutrons_amount: ::core::option::Option<TotalBurnedNeutronsAmount>,
}
/// MsgUpdateParams is the MsgUpdateParams request type.
///
/// Since: 0.47
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
// @@protoc_insertion_point(module)
