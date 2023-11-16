// @generated
/// Deprecated. Used only for migration purposes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Failure {
    /// ChannelId
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    /// Address of the failed contract
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// id of the failure under specific address
    #[prost(uint64, tag = "3")]
    pub id: u64,
    /// ACK id to restore
    #[prost(uint64, tag = "4")]
    pub ack_id: u64,
    /// Acknowledgement type
    #[prost(string, tag = "5")]
    pub ack_type: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
