// @generated
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, repeated, tag = "1")]
    pub fee_tiers: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag = "2")]
    pub max_true_taker_spread: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
