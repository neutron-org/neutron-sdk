use neutron_std_derive::CosmwasmExt;
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
#[proto_message(type_url = "/neutron.cron.v1.Schedule")]
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
#[proto_message(type_url = "/neutron.cron.v1.MsgExecuteContract")]
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
#[proto_message(type_url = "/neutron.cron.v1.ScheduleCount")]
pub struct ScheduleCount {
    /// The number of current schedules
    #[prost(int32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub count: i32,
}
