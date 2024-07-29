use neutron_std_derive::CosmwasmExt;
/// CurrencyPair is the standard representation of a pair of assets, where one
/// (Base) is priced in terms of the other (Quote)
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
#[proto_message(type_url = "/slinky.types.v1.CurrencyPair")]
pub struct CurrencyPair {
    #[prost(string, tag = "1")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub quote: ::prost::alloc::string::String,
}
