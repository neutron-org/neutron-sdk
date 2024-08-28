pub mod v2;
use neutron_std_derive::CosmwasmExt;
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
#[proto_message(type_url = "/neutron.dex.PairID")]
pub struct PairId {
    #[prost(string, tag = "1")]
    pub token0: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token1: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/neutron.dex.TradePairID")]
pub struct TradePairId {
    #[prost(string, tag = "2")]
    pub maker_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub taker_denom: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/neutron.dex.PoolReservesKey")]
pub struct PoolReservesKey {
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "trade_pairID")]
    pub trade_pair_id: ::core::option::Option<TradePairId>,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tick_index_taker_to_maker: i64,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub fee: u64,
}
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
#[proto_message(type_url = "/neutron.dex.PoolReserves")]
pub struct PoolReserves {
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<PoolReservesKey>,
    #[prost(string, tag = "2")]
    pub reserves_maker_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub price_taker_to_maker: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub price_opposite_taker_to_maker: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/neutron.dex.Pool")]
pub struct Pool {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    #[prost(message, optional, tag = "2")]
    pub lower_tick0: ::core::option::Option<PoolReserves>,
    #[prost(message, optional, tag = "3")]
    pub upper_tick1: ::core::option::Option<PoolReserves>,
}
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
#[proto_message(type_url = "/neutron.dex.DepositRecord")]
pub struct DepositRecord {
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "pairID")]
    pub pair_id: ::core::option::Option<PairId>,
    #[prost(string, tag = "2")]
    pub shares_owned: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub center_tick_index: i64,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub lower_tick_index: i64,
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub upper_tick_index: i64,
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub fee: u64,
    #[prost(string, tag = "7")]
    pub total_shares: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "8")]
    pub pool: ::core::option::Option<Pool>,
}
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
#[proto_message(type_url = "/neutron.dex.LimitOrderTrancheKey")]
pub struct LimitOrderTrancheKey {
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "trade_pairID")]
    pub trade_pair_id: ::core::option::Option<TradePairId>,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tick_index_taker_to_maker: i64,
    #[prost(string, tag = "3")]
    pub tranche_key: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/neutron.dex.LimitOrderTranche")]
pub struct LimitOrderTranche {
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LimitOrderTrancheKey>,
    #[prost(string, tag = "2")]
    pub reserves_maker_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub reserves_taker_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub total_maker_denom: ::prost::alloc::string::String,
    /// LimitOrders with expiration_time set are valid as long as blockTime <= expiration_time
    #[prost(string, tag = "5")]
    pub total_taker_denom: ::prost::alloc::string::String,
    /// JIT orders also use expiration_time to handle deletion but represent a special case
    /// All JIT orders have a expiration_time of 0 and an exception is made to still treat these orders as live
    /// Order deletion still functions the same and the orders will be deleted at the end of the block
    #[prost(message, optional, tag = "6")]
    pub expiration_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(string, tag = "7")]
    pub price_taker_to_maker: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/neutron.dex.Params")]
pub struct Params {
    #[prost(uint64, repeated, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub fee_tiers: ::prost::alloc::vec::Vec<u64>,
    #[prost(bool, tag = "3")]
    pub paused: bool,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_jits_per_block: u64,
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub good_til_purge_allowance: u64,
}
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
#[proto_message(type_url = "/neutron.dex.DepositOptions")]
pub struct DepositOptions {
    #[prost(bool, tag = "1")]
    pub disable_autoswap: bool,
    #[prost(bool, tag = "2")]
    pub fail_tx_on_bel: bool,
}
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
#[proto_message(type_url = "/neutron.dex.MsgDeposit")]
pub struct MsgDeposit {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token_a: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token_b: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub amounts_a: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub amounts_b: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub tick_indexes_a_to_b: ::prost::alloc::vec::Vec<i64>,
    #[prost(uint64, repeated, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub fees: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag = "9")]
    pub options: ::prost::alloc::vec::Vec<DepositOptions>,
}
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
#[proto_message(type_url = "/neutron.dex.FailedDeposit")]
pub struct FailedDeposit {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "depositIDx")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub deposit_idx: u64,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/neutron.dex.MsgDepositResponse")]
pub struct MsgDepositResponse {
    #[prost(string, repeated, tag = "1")]
    pub reserve0_deposited: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub reserve1_deposited: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub failed_deposits: ::prost::alloc::vec::Vec<FailedDeposit>,
    #[prost(message, repeated, tag = "4")]
    pub shares_issued: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/neutron.dex.MsgWithdrawal")]
pub struct MsgWithdrawal {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token_a: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token_b: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub shares_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub tick_indexes_a_to_b: ::prost::alloc::vec::Vec<i64>,
    #[prost(uint64, repeated, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub fees: ::prost::alloc::vec::Vec<u64>,
}
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
#[proto_message(type_url = "/neutron.dex.MsgWithdrawalResponse")]
pub struct MsgWithdrawalResponse {
    #[prost(string, tag = "1")]
    pub reserve0_withdrawn: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub reserve1_withdrawn: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub shares_burned: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/neutron.dex.MsgPlaceLimitOrder")]
pub struct MsgPlaceLimitOrder {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token_out: ::prost::alloc::string::String,
    /// DEPRECATED: tick_index_in_to_out will be removed in future release; limit_sell_price should be used instead.
    #[deprecated]
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tick_index_in_to_out: i64,
    #[prost(string, tag = "7")]
    pub amount_in: ::prost::alloc::string::String,
    #[prost(enumeration = "LimitOrderType", tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_type: i32,
    /// expirationTime is only valid iff orderType == GOOD_TIL_TIME.
    #[prost(message, optional, tag = "9")]
    pub expiration_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(string, tag = "10")]
    pub max_amount_out: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub limit_sell_price: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/neutron.dex.MsgPlaceLimitOrderResponse")]
pub struct MsgPlaceLimitOrderResponse {
    #[prost(string, tag = "1")]
    pub tranche_key: ::prost::alloc::string::String,
    /// Total amount of coin used for the limit order
    #[prost(message, optional, tag = "2")]
    pub coin_in: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// Total amount of coin received from the taker portion of the limit order
    /// This is the amount of coin immediately available in the users account after
    /// executing the limit order. It does not include any future proceeds from the
    /// maker portion which will have withdrawn in the future
    #[prost(message, optional, tag = "3")]
    pub taker_coin_out: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// Total amount of the token in that was immediately swapped for takerOutCoin
    #[prost(message, optional, tag = "4")]
    pub taker_coin_in: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/neutron.dex.MsgWithdrawFilledLimitOrder")]
pub struct MsgWithdrawFilledLimitOrder {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tranche_key: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/neutron.dex.MsgWithdrawFilledLimitOrderResponse")]
pub struct MsgWithdrawFilledLimitOrderResponse {
    /// Total amount of taker reserves that were withdrawn
    #[prost(message, optional, tag = "1")]
    pub taker_coin_out: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// Total amount of maker reserves that were withdrawn --only applies to inactive LimitOrders
    #[prost(message, optional, tag = "2")]
    pub maker_coin_out: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/neutron.dex.MsgCancelLimitOrder")]
pub struct MsgCancelLimitOrder {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tranche_key: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/neutron.dex.MsgCancelLimitOrderResponse")]
pub struct MsgCancelLimitOrderResponse {
    /// Total amount of taker reserves that were withdrawn
    #[prost(message, optional, tag = "1")]
    pub taker_coin_out: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// Total amount of maker reserves that were canceled
    #[prost(message, optional, tag = "2")]
    pub maker_coin_out: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/neutron.dex.MultiHopRoute")]
pub struct MultiHopRoute {
    #[prost(string, repeated, tag = "1")]
    pub hops: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
#[proto_message(type_url = "/neutron.dex.MsgMultiHopSwap")]
pub struct MsgMultiHopSwap {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub routes: ::prost::alloc::vec::Vec<MultiHopRoute>,
    #[prost(string, tag = "4")]
    pub amount_in: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub exit_limit_price: ::prost::alloc::string::String,
    /// If pickBestRoute == true then all routes are run and the route with the
    /// best price is chosen otherwise, the first succesful route is used.
    #[prost(bool, tag = "6")]
    pub pick_best_route: bool,
}
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
#[proto_message(type_url = "/neutron.dex.MsgMultiHopSwapResponse")]
pub struct MsgMultiHopSwapResponse {
    #[prost(message, optional, tag = "1")]
    pub coin_out: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "2")]
    pub route: ::core::option::Option<MultiHopRoute>,
    #[prost(message, repeated, tag = "3")]
    pub dust: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/neutron.dex.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: 0.47
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
#[proto_message(type_url = "/neutron.dex.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum LimitOrderType {
    GoodTilCancelled = 0,
    FillOrKill = 1,
    ImmediateOrCancel = 2,
    JustInTime = 3,
    GoodTilTime = 4,
}
impl LimitOrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LimitOrderType::GoodTilCancelled => "GOOD_TIL_CANCELLED",
            LimitOrderType::FillOrKill => "FILL_OR_KILL",
            LimitOrderType::ImmediateOrCancel => "IMMEDIATE_OR_CANCEL",
            LimitOrderType::JustInTime => "JUST_IN_TIME",
            LimitOrderType::GoodTilTime => "GOOD_TIL_TIME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GOOD_TIL_CANCELLED" => Some(Self::GoodTilCancelled),
            "FILL_OR_KILL" => Some(Self::FillOrKill),
            "IMMEDIATE_OR_CANCEL" => Some(Self::ImmediateOrCancel),
            "JUST_IN_TIME" => Some(Self::JustInTime),
            "GOOD_TIL_TIME" => Some(Self::GoodTilTime),
            _ => None,
        }
    }
}
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
#[proto_message(type_url = "/neutron.dex.LimitOrderTrancheUser")]
pub struct LimitOrderTrancheUser {
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "trade_pairID")]
    pub trade_pair_id: ::core::option::Option<TradePairId>,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tick_index_taker_to_maker: i64,
    #[prost(string, tag = "3")]
    pub tranche_key: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub shares_owned: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub shares_withdrawn: ::prost::alloc::string::String,
    /// TODO: remove this in next release. It is no longer used
    #[prost(string, tag = "7")]
    pub shares_cancelled: ::prost::alloc::string::String,
    #[prost(enumeration = "LimitOrderType", tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_type: i32,
}
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
#[proto_message(type_url = "/neutron.dex.PoolMetadata")]
pub struct PoolMetadata {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tick: i64,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub fee: u64,
    #[prost(message, optional, tag = "4")]
    #[serde(alias = "pairID")]
    pub pair_id: ::core::option::Option<PairId>,
}
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
#[proto_message(type_url = "/neutron.dex.TickLiquidity")]
pub struct TickLiquidity {
    #[prost(oneof = "tick_liquidity::Liquidity", tags = "1, 2")]
    pub liquidity: ::core::option::Option<tick_liquidity::Liquidity>,
}
/// Nested message and enum types in `TickLiquidity`.
pub mod tick_liquidity {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum Liquidity {
        #[prost(message, tag = "1")]
        PoolReserves(super::PoolReserves),
        #[prost(message, tag = "2")]
        LimitOrderTranche(super::LimitOrderTranche),
    }
}
/// GenesisState defines the dex module's genesis state.
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
#[proto_message(type_url = "/neutron.dex.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub tick_liquidity_list: ::prost::alloc::vec::Vec<TickLiquidity>,
    #[prost(message, repeated, tag = "3")]
    pub inactive_limit_order_tranche_list: ::prost::alloc::vec::Vec<LimitOrderTranche>,
    #[prost(message, repeated, tag = "4")]
    pub limit_order_tranche_user_list: ::prost::alloc::vec::Vec<LimitOrderTrancheUser>,
    #[prost(message, repeated, tag = "5")]
    pub pool_metadata_list: ::prost::alloc::vec::Vec<PoolMetadata>,
    /// this line is used by starport scaffolding # genesis/proto/state
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_count: u64,
}
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
#[proto_message(type_url = "/neutron.dex.LimitOrderExpiration")]
pub struct LimitOrderExpiration {
    /// see limitOrderTranche.proto for details on expiration_time
    #[prost(message, optional, tag = "1")]
    pub expiration_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub tranche_ref: ::prost::alloc::vec::Vec<u8>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
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
#[proto_message(type_url = "/neutron.dex.QueryParamsRequest")]
#[proto_query(path = "/neutron.dex.Query/Params", response_type = QueryParamsResponse)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/neutron.dex.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryGetLimitOrderTrancheUserRequest")]
#[proto_query(
    path = "/neutron.dex.Query/LimitOrderTrancheUser",
    response_type = QueryGetLimitOrderTrancheUserResponse
)]
pub struct QueryGetLimitOrderTrancheUserRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tranche_key: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub calc_withdrawable_shares: bool,
}
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
#[proto_message(type_url = "/neutron.dex.QueryGetLimitOrderTrancheUserResponse")]
pub struct QueryGetLimitOrderTrancheUserResponse {
    #[prost(message, optional, tag = "1")]
    pub limit_order_tranche_user: ::core::option::Option<LimitOrderTrancheUser>,
    #[prost(string, tag = "2")]
    pub withdrawable_shares: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllLimitOrderTrancheUserRequest")]
#[proto_query(
    path = "/neutron.dex.Query/LimitOrderTrancheUserAll",
    response_type = QueryAllLimitOrderTrancheUserResponse
)]
pub struct QueryAllLimitOrderTrancheUserRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllLimitOrderTrancheUserResponse")]
pub struct QueryAllLimitOrderTrancheUserResponse {
    #[prost(message, repeated, tag = "1")]
    pub limit_order_tranche_user: ::prost::alloc::vec::Vec<LimitOrderTrancheUser>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryGetLimitOrderTrancheRequest")]
#[proto_query(
    path = "/neutron.dex.Query/LimitOrderTranche",
    response_type = QueryGetLimitOrderTrancheResponse
)]
pub struct QueryGetLimitOrderTrancheRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "pairID")]
    pub pair_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tick_index: i64,
    #[prost(string, tag = "3")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub tranche_key: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/neutron.dex.QueryGetLimitOrderTrancheResponse")]
pub struct QueryGetLimitOrderTrancheResponse {
    #[prost(message, optional, tag = "1")]
    pub limit_order_tranche: ::core::option::Option<LimitOrderTranche>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllLimitOrderTrancheRequest")]
#[proto_query(
    path = "/neutron.dex.Query/LimitOrderTrancheAll",
    response_type = QueryAllLimitOrderTrancheResponse
)]
pub struct QueryAllLimitOrderTrancheRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "pairID")]
    pub pair_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllLimitOrderTrancheResponse")]
pub struct QueryAllLimitOrderTrancheResponse {
    #[prost(message, repeated, tag = "1")]
    pub limit_order_tranche: ::prost::alloc::vec::Vec<LimitOrderTranche>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllUserDepositsRequest")]
#[proto_query(
    path = "/neutron.dex.Query/UserDepositsAll",
    response_type = QueryAllUserDepositsResponse
)]
pub struct QueryAllUserDepositsRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    #[prost(bool, tag = "3")]
    pub include_pool_data: bool,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllUserDepositsResponse")]
pub struct QueryAllUserDepositsResponse {
    #[prost(message, repeated, tag = "1")]
    pub deposits: ::prost::alloc::vec::Vec<DepositRecord>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllLimitOrderTrancheUserByAddressRequest")]
#[proto_query(
    path = "/neutron.dex.Query/LimitOrderTrancheUserAllByAddress",
    response_type = QueryAllLimitOrderTrancheUserByAddressResponse
)]
pub struct QueryAllLimitOrderTrancheUserByAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllLimitOrderTrancheUserByAddressResponse")]
pub struct QueryAllLimitOrderTrancheUserByAddressResponse {
    #[prost(message, repeated, tag = "1")]
    pub limit_orders: ::prost::alloc::vec::Vec<LimitOrderTrancheUser>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllTickLiquidityRequest")]
#[proto_query(
    path = "/neutron.dex.Query/TickLiquidityAll",
    response_type = QueryAllTickLiquidityResponse
)]
pub struct QueryAllTickLiquidityRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "pairID")]
    pub pair_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllTickLiquidityResponse")]
pub struct QueryAllTickLiquidityResponse {
    #[prost(message, repeated, tag = "1")]
    pub tick_liquidity: ::prost::alloc::vec::Vec<TickLiquidity>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryGetInactiveLimitOrderTrancheRequest")]
#[proto_query(
    path = "/neutron.dex.Query/InactiveLimitOrderTranche",
    response_type = QueryGetInactiveLimitOrderTrancheResponse
)]
pub struct QueryGetInactiveLimitOrderTrancheRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "pairID")]
    pub pair_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tick_index: i64,
    #[prost(string, tag = "4")]
    pub tranche_key: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/neutron.dex.QueryGetInactiveLimitOrderTrancheResponse")]
pub struct QueryGetInactiveLimitOrderTrancheResponse {
    #[prost(message, optional, tag = "1")]
    pub inactive_limit_order_tranche: ::core::option::Option<LimitOrderTranche>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllInactiveLimitOrderTrancheRequest")]
#[proto_query(
    path = "/neutron.dex.Query/InactiveLimitOrderTrancheAll",
    response_type = QueryAllInactiveLimitOrderTrancheResponse
)]
pub struct QueryAllInactiveLimitOrderTrancheRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllInactiveLimitOrderTrancheResponse")]
pub struct QueryAllInactiveLimitOrderTrancheResponse {
    #[prost(message, repeated, tag = "1")]
    pub inactive_limit_order_tranche: ::prost::alloc::vec::Vec<LimitOrderTranche>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllPoolReservesRequest")]
#[proto_query(
    path = "/neutron.dex.Query/PoolReservesAll",
    response_type = QueryAllPoolReservesResponse
)]
pub struct QueryAllPoolReservesRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "pairID")]
    pub pair_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllPoolReservesResponse")]
pub struct QueryAllPoolReservesResponse {
    #[prost(message, repeated, tag = "1")]
    pub pool_reserves: ::prost::alloc::vec::Vec<PoolReserves>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryGetPoolReservesRequest")]
#[proto_query(
    path = "/neutron.dex.Query/PoolReserves",
    response_type = QueryGetPoolReservesResponse
)]
pub struct QueryGetPoolReservesRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "pairID")]
    pub pair_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tick_index: i64,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub fee: u64,
}
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
#[proto_message(type_url = "/neutron.dex.QueryGetPoolReservesResponse")]
pub struct QueryGetPoolReservesResponse {
    #[prost(message, optional, tag = "1")]
    pub pool_reserves: ::core::option::Option<PoolReserves>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryEstimateMultiHopSwapRequest")]
#[proto_query(
    path = "/neutron.dex.Query/EstimateMultiHopSwap",
    response_type = QueryEstimateMultiHopSwapResponse
)]
pub struct QueryEstimateMultiHopSwapRequest {
    /// DEPRECATED: Use QuerySimulateMultiHopSwap
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub routes: ::prost::alloc::vec::Vec<MultiHopRoute>,
    #[prost(string, tag = "4")]
    pub amount_in: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub exit_limit_price: ::prost::alloc::string::String,
    /// If pickBestRoute == true then all routes are run and the route with the
    /// best price is chosen otherwise, the first succesful route is used.
    #[prost(bool, tag = "6")]
    pub pick_best_route: bool,
}
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
#[proto_message(type_url = "/neutron.dex.QueryEstimateMultiHopSwapResponse")]
pub struct QueryEstimateMultiHopSwapResponse {
    #[prost(message, optional, tag = "1")]
    pub coin_out: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryEstimatePlaceLimitOrderRequest")]
#[proto_query(
    path = "/neutron.dex.Query/EstimatePlaceLimitOrder",
    response_type = QueryEstimatePlaceLimitOrderResponse
)]
pub struct QueryEstimatePlaceLimitOrderRequest {
    /// DEPRECATED: Use QuerySimulatePlaceLimitOrder
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token_out: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tick_index_in_to_out: i64,
    #[prost(string, tag = "6")]
    pub amount_in: ::prost::alloc::string::String,
    #[prost(enumeration = "LimitOrderType", tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_type: i32,
    /// expirationTime is only valid iff orderType == GOOD_TIL_TIME.
    #[prost(message, optional, tag = "8")]
    pub expiration_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(string, tag = "9")]
    pub max_amount_out: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/neutron.dex.QueryEstimatePlaceLimitOrderResponse")]
pub struct QueryEstimatePlaceLimitOrderResponse {
    /// Total amount of coin used for the limit order
    /// You can derive makerLimitInCoin using the equation: totalInCoin =
    /// swapInCoin + makerLimitInCoin
    #[prost(message, optional, tag = "1")]
    pub total_in_coin: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// Total amount of the token in that was immediately swapped for swapOutCoin
    #[prost(message, optional, tag = "2")]
    pub swap_in_coin: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// Total amount of coin received from the taker portion of the limit order
    /// This is the amount of coin immediately available in the users account after
    /// executing the limit order. It does not include any future proceeds from the
    /// maker portion which will have withdrawn in the future
    #[prost(message, optional, tag = "3")]
    pub swap_out_coin: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryPoolRequest")]
#[proto_query(path = "/neutron.dex.Query/Pool", response_type = QueryPoolResponse)]
pub struct QueryPoolRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "pairID")]
    pub pair_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tick_index: i64,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub fee: u64,
}
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
#[proto_message(type_url = "/neutron.dex.QueryPoolByIDRequest")]
#[proto_query(path = "/neutron.dex.Query/PoolByID", response_type = QueryPoolResponse)]
pub struct QueryPoolByIdRequest {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
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
#[proto_message(type_url = "/neutron.dex.QueryPoolResponse")]
pub struct QueryPoolResponse {
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<Pool>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryGetPoolMetadataRequest")]
#[proto_query(
    path = "/neutron.dex.Query/PoolMetadata",
    response_type = QueryGetPoolMetadataResponse
)]
pub struct QueryGetPoolMetadataRequest {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
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
#[proto_message(type_url = "/neutron.dex.QueryGetPoolMetadataResponse")]
pub struct QueryGetPoolMetadataResponse {
    #[prost(message, optional, tag = "1")]
    pub pool_metadata: ::core::option::Option<PoolMetadata>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllPoolMetadataRequest")]
#[proto_query(
    path = "/neutron.dex.Query/PoolMetadataAll",
    response_type = QueryAllPoolMetadataResponse
)]
pub struct QueryAllPoolMetadataRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
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
#[proto_message(type_url = "/neutron.dex.QueryAllPoolMetadataResponse")]
pub struct QueryAllPoolMetadataResponse {
    #[prost(message, repeated, tag = "1")]
    pub pool_metadata: ::prost::alloc::vec::Vec<PoolMetadata>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
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
#[proto_message(type_url = "/neutron.dex.QuerySimulateDepositRequest")]
#[proto_query(
    path = "/neutron.dex.Query/SimulateDeposit",
    response_type = QuerySimulateDepositResponse
)]
pub struct QuerySimulateDepositRequest {
    #[prost(message, optional, tag = "1")]
    pub msg: ::core::option::Option<MsgDeposit>,
}
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
#[proto_message(type_url = "/neutron.dex.QuerySimulateDepositResponse")]
pub struct QuerySimulateDepositResponse {
    #[prost(message, optional, tag = "1")]
    pub resp: ::core::option::Option<MsgDepositResponse>,
}
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
#[proto_message(type_url = "/neutron.dex.QuerySimulateWithdrawalRequest")]
#[proto_query(
    path = "/neutron.dex.Query/SimulateWithdrawal",
    response_type = QuerySimulateWithdrawalResponse
)]
pub struct QuerySimulateWithdrawalRequest {
    #[prost(message, optional, tag = "1")]
    pub msg: ::core::option::Option<MsgWithdrawal>,
}
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
#[proto_message(type_url = "/neutron.dex.QuerySimulateWithdrawalResponse")]
pub struct QuerySimulateWithdrawalResponse {
    #[prost(message, optional, tag = "1")]
    pub resp: ::core::option::Option<MsgWithdrawalResponse>,
}
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
#[proto_message(type_url = "/neutron.dex.QuerySimulatePlaceLimitOrderRequest")]
#[proto_query(
    path = "/neutron.dex.Query/SimulatePlaceLimitOrder",
    response_type = QuerySimulatePlaceLimitOrderResponse
)]
pub struct QuerySimulatePlaceLimitOrderRequest {
    #[prost(message, optional, tag = "1")]
    pub msg: ::core::option::Option<MsgPlaceLimitOrder>,
}
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
#[proto_message(type_url = "/neutron.dex.QuerySimulatePlaceLimitOrderResponse")]
pub struct QuerySimulatePlaceLimitOrderResponse {
    #[prost(message, optional, tag = "1")]
    pub resp: ::core::option::Option<MsgPlaceLimitOrderResponse>,
}
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
#[proto_message(type_url = "/neutron.dex.QuerySimulateWithdrawFilledLimitOrderRequest")]
#[proto_query(
    path = "/neutron.dex.Query/SimulateWithdrawFilledLimitOrder",
    response_type = QuerySimulateWithdrawFilledLimitOrderResponse
)]
pub struct QuerySimulateWithdrawFilledLimitOrderRequest {
    #[prost(message, optional, tag = "1")]
    pub msg: ::core::option::Option<MsgWithdrawFilledLimitOrder>,
}
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
#[proto_message(type_url = "/neutron.dex.QuerySimulateWithdrawFilledLimitOrderResponse")]
pub struct QuerySimulateWithdrawFilledLimitOrderResponse {
    #[prost(message, optional, tag = "1")]
    pub resp: ::core::option::Option<MsgWithdrawFilledLimitOrderResponse>,
}
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
#[proto_message(type_url = "/neutron.dex.QuerySimulateCancelLimitOrderRequest")]
#[proto_query(
    path = "/neutron.dex.Query/SimulateCancelLimitOrder",
    response_type = QuerySimulateCancelLimitOrderResponse
)]
pub struct QuerySimulateCancelLimitOrderRequest {
    #[prost(message, optional, tag = "1")]
    pub msg: ::core::option::Option<MsgCancelLimitOrder>,
}
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
#[proto_message(type_url = "/neutron.dex.QuerySimulateCancelLimitOrderResponse")]
pub struct QuerySimulateCancelLimitOrderResponse {
    #[prost(message, optional, tag = "1")]
    pub resp: ::core::option::Option<MsgCancelLimitOrderResponse>,
}
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
#[proto_message(type_url = "/neutron.dex.QuerySimulateMultiHopSwapRequest")]
#[proto_query(
    path = "/neutron.dex.Query/SimulateMultiHopSwap",
    response_type = QuerySimulateMultiHopSwapResponse
)]
pub struct QuerySimulateMultiHopSwapRequest {
    #[prost(message, optional, tag = "1")]
    pub msg: ::core::option::Option<MsgMultiHopSwap>,
}
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
#[proto_message(type_url = "/neutron.dex.QuerySimulateMultiHopSwapResponse")]
pub struct QuerySimulateMultiHopSwapResponse {
    #[prost(message, optional, tag = "1")]
    pub resp: ::core::option::Option<MsgMultiHopSwapResponse>,
}
pub struct DexQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> DexQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn limit_order_tranche_user(
        &self,
        address: ::prost::alloc::string::String,
        tranche_key: ::prost::alloc::string::String,
        calc_withdrawable_shares: bool,
    ) -> Result<QueryGetLimitOrderTrancheUserResponse, cosmwasm_std::StdError> {
        QueryGetLimitOrderTrancheUserRequest {
            address,
            tranche_key,
            calc_withdrawable_shares,
        }
        .query(self.querier)
    }
    pub fn limit_order_tranche_user_all(
        &self,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryAllLimitOrderTrancheUserResponse, cosmwasm_std::StdError> {
        QueryAllLimitOrderTrancheUserRequest { pagination }.query(self.querier)
    }
    pub fn limit_order_tranche_user_all_by_address(
        &self,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryAllLimitOrderTrancheUserByAddressResponse, cosmwasm_std::StdError> {
        QueryAllLimitOrderTrancheUserByAddressRequest {
            address,
            pagination,
        }
        .query(self.querier)
    }
    pub fn limit_order_tranche(
        &self,
        pair_id: ::prost::alloc::string::String,
        tick_index: i64,
        token_in: ::prost::alloc::string::String,
        tranche_key: ::prost::alloc::string::String,
    ) -> Result<QueryGetLimitOrderTrancheResponse, cosmwasm_std::StdError> {
        QueryGetLimitOrderTrancheRequest {
            pair_id,
            tick_index,
            token_in,
            tranche_key,
        }
        .query(self.querier)
    }
    pub fn limit_order_tranche_all(
        &self,
        pair_id: ::prost::alloc::string::String,
        token_in: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryAllLimitOrderTrancheResponse, cosmwasm_std::StdError> {
        QueryAllLimitOrderTrancheRequest {
            pair_id,
            token_in,
            pagination,
        }
        .query(self.querier)
    }
    pub fn user_deposits_all(
        &self,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
        include_pool_data: bool,
    ) -> Result<QueryAllUserDepositsResponse, cosmwasm_std::StdError> {
        QueryAllUserDepositsRequest {
            address,
            pagination,
            include_pool_data,
        }
        .query(self.querier)
    }
    pub fn tick_liquidity_all(
        &self,
        pair_id: ::prost::alloc::string::String,
        token_in: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryAllTickLiquidityResponse, cosmwasm_std::StdError> {
        QueryAllTickLiquidityRequest {
            pair_id,
            token_in,
            pagination,
        }
        .query(self.querier)
    }
    pub fn inactive_limit_order_tranche(
        &self,
        pair_id: ::prost::alloc::string::String,
        token_in: ::prost::alloc::string::String,
        tick_index: i64,
        tranche_key: ::prost::alloc::string::String,
    ) -> Result<QueryGetInactiveLimitOrderTrancheResponse, cosmwasm_std::StdError> {
        QueryGetInactiveLimitOrderTrancheRequest {
            pair_id,
            token_in,
            tick_index,
            tranche_key,
        }
        .query(self.querier)
    }
    pub fn inactive_limit_order_tranche_all(
        &self,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryAllInactiveLimitOrderTrancheResponse, cosmwasm_std::StdError> {
        QueryAllInactiveLimitOrderTrancheRequest { pagination }.query(self.querier)
    }
    pub fn pool_reserves_all(
        &self,
        pair_id: ::prost::alloc::string::String,
        token_in: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryAllPoolReservesResponse, cosmwasm_std::StdError> {
        QueryAllPoolReservesRequest {
            pair_id,
            token_in,
            pagination,
        }
        .query(self.querier)
    }
    pub fn pool_reserves(
        &self,
        pair_id: ::prost::alloc::string::String,
        token_in: ::prost::alloc::string::String,
        tick_index: i64,
        fee: u64,
    ) -> Result<QueryGetPoolReservesResponse, cosmwasm_std::StdError> {
        QueryGetPoolReservesRequest {
            pair_id,
            token_in,
            tick_index,
            fee,
        }
        .query(self.querier)
    }
    #[deprecated]
    pub fn estimate_multi_hop_swap(
        &self,
        creator: ::prost::alloc::string::String,
        receiver: ::prost::alloc::string::String,
        routes: ::prost::alloc::vec::Vec<MultiHopRoute>,
        amount_in: ::prost::alloc::string::String,
        exit_limit_price: ::prost::alloc::string::String,
        pick_best_route: bool,
    ) -> Result<QueryEstimateMultiHopSwapResponse, cosmwasm_std::StdError> {
        QueryEstimateMultiHopSwapRequest {
            creator,
            receiver,
            routes,
            amount_in,
            exit_limit_price,
            pick_best_route,
        }
        .query(self.querier)
    }
    #[deprecated]
    #[allow(clippy::too_many_arguments)]
    pub fn estimate_place_limit_order(
        &self,
        creator: ::prost::alloc::string::String,
        receiver: ::prost::alloc::string::String,
        token_in: ::prost::alloc::string::String,
        token_out: ::prost::alloc::string::String,
        tick_index_in_to_out: i64,
        amount_in: ::prost::alloc::string::String,
        order_type: i32,
        expiration_time: ::core::option::Option<crate::shim::Timestamp>,
        max_amount_out: ::prost::alloc::string::String,
    ) -> Result<QueryEstimatePlaceLimitOrderResponse, cosmwasm_std::StdError> {
        QueryEstimatePlaceLimitOrderRequest {
            creator,
            receiver,
            token_in,
            token_out,
            tick_index_in_to_out,
            amount_in,
            order_type,
            expiration_time,
            max_amount_out,
        }
        .query(self.querier)
    }
    pub fn pool(
        &self,
        pair_id: ::prost::alloc::string::String,
        tick_index: i64,
        fee: u64,
    ) -> Result<QueryPoolResponse, cosmwasm_std::StdError> {
        QueryPoolRequest {
            pair_id,
            tick_index,
            fee,
        }
        .query(self.querier)
    }
    pub fn pool_by_id(&self, pool_id: u64) -> Result<QueryPoolResponse, cosmwasm_std::StdError> {
        QueryPoolByIdRequest { pool_id }.query(self.querier)
    }
    pub fn pool_metadata(
        &self,
        id: u64,
    ) -> Result<QueryGetPoolMetadataResponse, cosmwasm_std::StdError> {
        QueryGetPoolMetadataRequest { id }.query(self.querier)
    }
    pub fn pool_metadata_all(
        &self,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryAllPoolMetadataResponse, cosmwasm_std::StdError> {
        QueryAllPoolMetadataRequest { pagination }.query(self.querier)
    }
    pub fn simulate_deposit(
        &self,
        msg: ::core::option::Option<MsgDeposit>,
    ) -> Result<QuerySimulateDepositResponse, cosmwasm_std::StdError> {
        QuerySimulateDepositRequest { msg }.query(self.querier)
    }
    pub fn simulate_withdrawal(
        &self,
        msg: ::core::option::Option<MsgWithdrawal>,
    ) -> Result<QuerySimulateWithdrawalResponse, cosmwasm_std::StdError> {
        QuerySimulateWithdrawalRequest { msg }.query(self.querier)
    }
    pub fn simulate_place_limit_order(
        &self,
        msg: ::core::option::Option<MsgPlaceLimitOrder>,
    ) -> Result<QuerySimulatePlaceLimitOrderResponse, cosmwasm_std::StdError> {
        QuerySimulatePlaceLimitOrderRequest { msg }.query(self.querier)
    }
    pub fn simulate_withdraw_filled_limit_order(
        &self,
        msg: ::core::option::Option<MsgWithdrawFilledLimitOrder>,
    ) -> Result<QuerySimulateWithdrawFilledLimitOrderResponse, cosmwasm_std::StdError> {
        QuerySimulateWithdrawFilledLimitOrderRequest { msg }.query(self.querier)
    }
    pub fn simulate_cancel_limit_order(
        &self,
        msg: ::core::option::Option<MsgCancelLimitOrder>,
    ) -> Result<QuerySimulateCancelLimitOrderResponse, cosmwasm_std::StdError> {
        QuerySimulateCancelLimitOrderRequest { msg }.query(self.querier)
    }
    pub fn simulate_multi_hop_swap(
        &self,
        msg: ::core::option::Option<MsgMultiHopSwap>,
    ) -> Result<QuerySimulateMultiHopSwapResponse, cosmwasm_std::StdError> {
        QuerySimulateMultiHopSwapRequest { msg }.query(self.querier)
    }
}
