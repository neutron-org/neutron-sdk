// @generated
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// Security address that can remove schedules
    #[prost(string, tag = "1")]
    pub security_address: ::prost::alloc::string::String,
    /// Limit of schedules executed in one block
    #[prost(uint64, tag = "2")]
    pub limit: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schedule {
    /// Name of schedule
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Period in blocks
    #[prost(uint64, tag = "2")]
    pub period: u64,
    /// Msgs that will be executed every period amount of time
    #[prost(message, repeated, tag = "3")]
    pub msgs: ::prost::alloc::vec::Vec<MsgExecuteContract>,
    /// Last execution's block height
    #[prost(uint64, tag = "4")]
    pub last_execute_height: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteContract {
    /// Contract is the address of the smart contract
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    /// Msg is json encoded message to be passed to the contract
    #[prost(string, tag = "2")]
    pub msg: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleCount {
    /// Count is the number of current schedules
    #[prost(int32, tag = "1")]
    pub count: i32,
}
/// GenesisState defines the cron module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "2")]
    pub schedule_list: ::prost::alloc::vec::Vec<Schedule>,
    /// this line is used by starport scaffolding # genesis/proto/state
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetScheduleRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetScheduleResponse {
    #[prost(message, optional, tag = "1")]
    pub schedule: ::core::option::Option<Schedule>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySchedulesRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySchedulesResponse {
    #[prost(message, repeated, tag = "1")]
    pub schedules: ::prost::alloc::vec::Vec<Schedule>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
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
    /// params defines the x/cron parameters to update.
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
