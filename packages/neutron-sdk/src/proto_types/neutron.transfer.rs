// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransfer {
    /// the port on which the packet will be sent
    #[prost(string, tag = "1")]
    pub source_port: ::prost::alloc::string::String,
    /// the channel by which the packet will be sent
    #[prost(string, tag = "2")]
    pub source_channel: ::prost::alloc::string::String,
    /// the tokens to be transferred
    #[prost(message, optional, tag = "3")]
    pub token: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// the sender address
    #[prost(string, tag = "4")]
    pub sender: ::prost::alloc::string::String,
    /// the recipient address on the destination chain
    #[prost(string, tag = "5")]
    pub receiver: ::prost::alloc::string::String,
    /// Timeout height relative to the current block height.
    /// The timeout is disabled when set to 0.
    #[prost(message, optional, tag = "6")]
    pub timeout_height: ::core::option::Option<cosmos_sdk_proto::ibc::core::client::v1::Height>,
    /// Timeout timestamp in absolute nanoseconds since unix epoch.
    /// The timeout is disabled when set to 0.
    #[prost(uint64, tag = "7")]
    pub timeout_timestamp: u64,
    #[prost(string, tag = "8")]
    pub memo: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "9")]
    pub fee: ::core::option::Option<super::feerefunder::Fee>,
}
/// MsgTransferResponse is the modified response type for
/// ibc-go MsgTransfer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferResponse {
    /// channel's sequence_id for outgoing ibc packet. Unique per a channel.
    #[prost(uint64, tag = "1")]
    pub sequence_id: u64,
    /// channel src channel on neutron side trasaction was submitted from
    #[prost(string, tag = "2")]
    pub channel: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
