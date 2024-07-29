use neutron_std_derive::CosmwasmExt;
/// OracleVoteExtension defines the vote extension structure for oracle prices.
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
#[proto_message(type_url = "/slinky.abci.v1.OracleVoteExtension")]
pub struct OracleVoteExtension {
    /// Prices defines a map of id(CurrencyPair) -> price.Bytes() . i.e. 1 ->
    /// 0x123.. (bytes). Notice the `id` function is determined by the
    /// `CurrencyPairIDStrategy` used in the VoteExtensionHandler.
    #[prost(map = "uint64, bytes", tag = "1")]
    pub prices: ::std::collections::HashMap<u64, ::prost::alloc::vec::Vec<u8>>,
}
