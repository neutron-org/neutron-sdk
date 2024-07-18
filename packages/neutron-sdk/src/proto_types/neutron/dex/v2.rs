use neutron_std_derive::CosmwasmExt;
/// Params defines the parameters for the module.
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
#[proto_message(type_url = "/neutron.dex.v2.Params")]
pub struct Params {
    #[prost(uint64, repeated, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub fee_tiers: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag = "2")]
    pub max_true_taker_spread: ::prost::alloc::string::String,
}
