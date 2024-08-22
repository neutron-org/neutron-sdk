use neutron_std_derive::CosmwasmExt;
/// Deprecated. Used only for migration purposes.
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
#[proto_message(type_url = "/neutron.contractmanager.v1.Failure")]
pub struct Failure {
    /// ChannelId
    #[prost(string, tag = "1")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// Address of the failed contract
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// id of the failure under specific address
    #[prost(uint64, tag = "3")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// ACK id to restore
    #[prost(uint64, tag = "4")]
    #[serde(alias = "ackID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ack_id: u64,
    /// Acknowledgement type
    #[prost(string, tag = "5")]
    pub ack_type: ::prost::alloc::string::String,
}
