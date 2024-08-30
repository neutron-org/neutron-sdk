pub mod v1;
use neutron_std_derive::CosmwasmExt;
/// Defines the parameters for the module.
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
#[proto_message(type_url = "/neutron.cron.Params")]
pub struct Params {
    /// Security address that can remove schedules
    #[prost(string, tag = "1")]
    pub security_address: ::prost::alloc::string::String,
    /// Limit of schedules executed in one block
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub limit: u64,
}
/// Defines the schedule for execution
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
#[proto_message(type_url = "/neutron.cron.Schedule")]
pub struct Schedule {
    /// Name of schedule
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Period in blocks
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub period: u64,
    /// Msgs that will be executed every certain number of blocks, specified in the `period` field
    #[prost(message, repeated, tag = "3")]
    pub msgs: ::prost::alloc::vec::Vec<MsgExecuteContract>,
    /// Last execution's block height
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_execute_height: u64,
    /// Stage when messages will be executed
    #[prost(enumeration = "ExecutionStage", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub execution_stage: i32,
}
/// Defines the contract and the message to pass
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
#[proto_message(type_url = "/neutron.cron.MsgExecuteContract")]
pub struct MsgExecuteContract {
    /// The address of the smart contract
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    /// JSON encoded message to be passed to the contract
    #[prost(string, tag = "2")]
    pub msg: ::prost::alloc::string::String,
}
/// Defines the number of current schedules
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
#[proto_message(type_url = "/neutron.cron.ScheduleCount")]
pub struct ScheduleCount {
    /// The number of current schedules
    #[prost(int32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub count: i32,
}
/// Defines when messages will be executed in the block
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum ExecutionStage {
    /// Execution at the end of the block
    EndBlocker = 0,
    /// Execution at the beginning of the block
    BeginBlocker = 1,
}
impl ExecutionStage {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionStage::EndBlocker => "EXECUTION_STAGE_END_BLOCKER",
            ExecutionStage::BeginBlocker => "EXECUTION_STAGE_BEGIN_BLOCKER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_STAGE_END_BLOCKER" => Some(Self::EndBlocker),
            "EXECUTION_STAGE_BEGIN_BLOCKER" => Some(Self::BeginBlocker),
            _ => None,
        }
    }
}
/// Defines the cron module's genesis state.
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
#[proto_message(type_url = "/neutron.cron.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "2")]
    pub schedule_list: ::prost::alloc::vec::Vec<Schedule>,
    /// this line is used by starport scaffolding # genesis/proto/state
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// The request type for the Query/Params RPC method.
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
#[proto_message(type_url = "/neutron.cron.QueryParamsRequest")]
#[proto_query(path = "/neutron.cron.Query/Params", response_type = QueryParamsResponse)]
pub struct QueryParamsRequest {}
/// The response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/neutron.cron.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// The request type for the Query/Schedule RPC method.
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
#[proto_message(type_url = "/neutron.cron.QueryGetScheduleRequest")]
#[proto_query(
    path = "/neutron.cron.Query/Schedule",
    response_type = QueryGetScheduleResponse
)]
pub struct QueryGetScheduleRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/neutron.cron.QueryGetScheduleResponse")]
pub struct QueryGetScheduleResponse {
    #[prost(message, optional, tag = "1")]
    pub schedule: ::core::option::Option<Schedule>,
}
/// The request type for the Query/Schedules RPC method.
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
#[proto_message(type_url = "/neutron.cron.QuerySchedulesRequest")]
#[proto_query(
    path = "/neutron.cron.Query/Schedules",
    response_type = QuerySchedulesResponse
)]
pub struct QuerySchedulesRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// The response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/neutron.cron.QuerySchedulesResponse")]
pub struct QuerySchedulesResponse {
    #[prost(message, repeated, tag = "1")]
    pub schedules: ::prost::alloc::vec::Vec<Schedule>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// The MsgAddSchedule request type.
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
#[proto_message(type_url = "/neutron.cron.MsgAddSchedule")]
pub struct MsgAddSchedule {
    /// The address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Name of the schedule
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Period in blocks
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub period: u64,
    /// Msgs that will be executed every certain number of blocks, specified in the `period` field
    #[prost(message, repeated, tag = "4")]
    pub msgs: ::prost::alloc::vec::Vec<MsgExecuteContract>,
    /// Stage when messages will be executed
    #[prost(enumeration = "ExecutionStage", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub execution_stage: i32,
}
/// Defines the response structure for executing a MsgAddSchedule message.
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
#[proto_message(type_url = "/neutron.cron.MsgAddScheduleResponse")]
pub struct MsgAddScheduleResponse {}
/// The MsgRemoveSchedule request type.
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
#[proto_message(type_url = "/neutron.cron.MsgRemoveSchedule")]
pub struct MsgRemoveSchedule {
    /// The address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Name of the schedule
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// Defines the response structure for executing a MsgRemoveSchedule message.
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
#[proto_message(type_url = "/neutron.cron.MsgRemoveScheduleResponse")]
pub struct MsgRemoveScheduleResponse {}
/// The MsgUpdateParams request type.
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
#[proto_message(type_url = "/neutron.cron.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// The address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Defines the x/cron parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// Defines the response structure for executing a MsgUpdateParams message.
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
#[proto_message(type_url = "/neutron.cron.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct CronQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> CronQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn schedule(
        &self,
        name: ::prost::alloc::string::String,
    ) -> Result<QueryGetScheduleResponse, cosmwasm_std::StdError> {
        QueryGetScheduleRequest { name }.query(self.querier)
    }
    pub fn schedules(
        &self,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QuerySchedulesResponse, cosmwasm_std::StdError> {
        QuerySchedulesRequest { pagination }.query(self.querier)
    }
}
