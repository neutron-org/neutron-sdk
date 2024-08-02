use neutron_std_derive::CosmwasmExt;
/// Module is the config object of the alerts module.
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
#[proto_message(type_url = "/slinky.alerts.module.v1.Module")]
pub struct Module {
    /// Authority defines the custom module authority. The authority will default
    /// to the governance module account if not set. If the authority is set, the
    /// address provided must be a valid bech-32 address
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
}
