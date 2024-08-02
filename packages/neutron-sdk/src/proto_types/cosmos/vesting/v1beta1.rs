use neutron_std_derive::CosmwasmExt;
/// BaseVestingAccount implements the VestingAccount interface. It contains all
/// the necessary fields needed for any vesting account implementation.
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
#[proto_message(type_url = "/cosmos.vesting.v1beta1.BaseVestingAccount")]
pub struct BaseVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account: ::core::option::Option<super::super::auth::v1beta1::BaseAccount>,
    #[prost(message, repeated, tag = "2")]
    pub original_vesting: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "3")]
    pub delegated_free: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "4")]
    pub delegated_vesting: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// Vesting end time, as unix timestamp (in seconds).
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub end_time: i64,
}
/// ContinuousVestingAccount implements the VestingAccount interface. It
/// continuously vests by unlocking coins linearly with respect to time.
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
#[proto_message(type_url = "/cosmos.vesting.v1beta1.ContinuousVestingAccount")]
pub struct ContinuousVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
    /// Vesting start time, as unix timestamp (in seconds).
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub start_time: i64,
}
/// DelayedVestingAccount implements the VestingAccount interface. It vests all
/// coins after a specific time, but non prior. In other words, it keeps them
/// locked until a specified time.
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
#[proto_message(type_url = "/cosmos.vesting.v1beta1.DelayedVestingAccount")]
pub struct DelayedVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
}
/// Period defines a length of time and amount of coins that will vest.
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
#[proto_message(type_url = "/cosmos.vesting.v1beta1.Period")]
pub struct Period {
    /// Period duration in seconds.
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub length: i64,
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// PeriodicVestingAccount implements the VestingAccount interface. It
/// periodically vests by unlocking coins during each specified period.
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
#[proto_message(type_url = "/cosmos.vesting.v1beta1.PeriodicVestingAccount")]
pub struct PeriodicVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub start_time: i64,
    #[prost(message, repeated, tag = "3")]
    pub vesting_periods: ::prost::alloc::vec::Vec<Period>,
}
/// PermanentLockedAccount implements the VestingAccount interface. It does
/// not ever release coins, locking them indefinitely. Coins in this account can
/// still be used for delegating and for governance votes even while locked.
///
/// Since: cosmos-sdk 0.43
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
#[proto_message(type_url = "/cosmos.vesting.v1beta1.PermanentLockedAccount")]
pub struct PermanentLockedAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
}
/// MsgCreateVestingAccount defines a message that enables creating a vesting
/// account.
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
#[proto_message(type_url = "/cosmos.vesting.v1beta1.MsgCreateVestingAccount")]
pub struct MsgCreateVestingAccount {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// end of vesting as unix time (in seconds).
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub end_time: i64,
    #[prost(bool, tag = "5")]
    pub delayed: bool,
}
/// MsgCreateVestingAccountResponse defines the Msg/CreateVestingAccount response type.
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
#[proto_message(type_url = "/cosmos.vesting.v1beta1.MsgCreateVestingAccountResponse")]
pub struct MsgCreateVestingAccountResponse {}
/// MsgCreatePermanentLockedAccount defines a message that enables creating a permanent
/// locked account.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.vesting.v1beta1.MsgCreatePermanentLockedAccount")]
pub struct MsgCreatePermanentLockedAccount {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// MsgCreatePermanentLockedAccountResponse defines the Msg/CreatePermanentLockedAccount response type.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.vesting.v1beta1.MsgCreatePermanentLockedAccountResponse")]
pub struct MsgCreatePermanentLockedAccountResponse {}
/// MsgCreateVestingAccount defines a message that enables creating a vesting
/// account.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.vesting.v1beta1.MsgCreatePeriodicVestingAccount")]
pub struct MsgCreatePeriodicVestingAccount {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    /// start of vesting as unix time (in seconds).
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub start_time: i64,
    #[prost(message, repeated, tag = "4")]
    pub vesting_periods: ::prost::alloc::vec::Vec<Period>,
}
/// MsgCreateVestingAccountResponse defines the Msg/CreatePeriodicVestingAccount
/// response type.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.vesting.v1beta1.MsgCreatePeriodicVestingAccountResponse")]
pub struct MsgCreatePeriodicVestingAccountResponse {}
