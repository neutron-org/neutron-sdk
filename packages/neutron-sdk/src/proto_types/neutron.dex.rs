// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PairId {
    #[prost(string, tag = "1")]
    pub token0: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token1: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradePairId {
    #[prost(string, tag = "2")]
    pub maker_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub taker_denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolReservesKey {
    #[prost(message, optional, tag = "1")]
    pub trade_pair_id: ::core::option::Option<TradePairId>,
    #[prost(int64, tag = "2")]
    pub tick_index_taker_to_maker: i64,
    #[prost(uint64, tag = "3")]
    pub fee: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
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
// NOTE: This struct is never actually stored in the KV store. It is merely a
// convenience wrapper for holding both sides of a pool.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(message, optional, tag = "2")]
    pub lower_tick0: ::core::option::Option<PoolReserves>,
    #[prost(message, optional, tag = "3")]
    pub upper_tick1: ::core::option::Option<PoolReserves>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositRecord {
    #[prost(message, optional, tag = "1")]
    pub pair_id: ::core::option::Option<PairId>,
    #[prost(string, tag = "2")]
    pub shares_owned: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub center_tick_index: i64,
    #[prost(int64, tag = "4")]
    pub lower_tick_index: i64,
    #[prost(int64, tag = "5")]
    pub upper_tick_index: i64,
    #[prost(uint64, tag = "6")]
    pub fee: u64,
    #[prost(string, tag = "7")]
    pub total_shares: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "8")]
    pub pool: ::core::option::Option<Pool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LimitOrderTrancheKey {
    #[prost(message, optional, tag = "1")]
    pub trade_pair_id: ::core::option::Option<TradePairId>,
    #[prost(int64, tag = "2")]
    pub tick_index_taker_to_maker: i64,
    #[prost(string, tag = "3")]
    pub tranche_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
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
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "7")]
    pub price_taker_to_maker: ::prost::alloc::string::String,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, repeated, tag = "1")]
    pub fee_tiers: ::prost::alloc::vec::Vec<u64>,
    #[prost(bool, tag = "3")]
    pub paused: bool,
    #[prost(uint64, tag = "4")]
    pub max_jits_per_block: u64,
    #[prost(uint64, tag = "5")]
    pub good_til_purge_allowance: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositOptions {
    #[prost(bool, tag = "1")]
    pub disable_autoswap: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
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
    pub tick_indexes_a_to_b: ::prost::alloc::vec::Vec<i64>,
    #[prost(uint64, repeated, tag = "8")]
    pub fees: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag = "9")]
    pub options: ::prost::alloc::vec::Vec<DepositOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositResponse {
    #[prost(string, repeated, tag = "1")]
    pub reserve0_deposited: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub reserve1_deposited: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
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
    pub tick_indexes_a_to_b: ::prost::alloc::vec::Vec<i64>,
    #[prost(uint64, repeated, tag = "7")]
    pub fees: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawalResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
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
    pub tick_index_in_to_out: i64,
    #[prost(string, tag = "7")]
    pub amount_in: ::prost::alloc::string::String,
    #[prost(enumeration = "LimitOrderType", tag = "8")]
    pub order_type: i32,
    /// expirationTime is only valid iff orderType == GOOD_TIL_TIME.
    #[prost(message, optional, tag = "9")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "10")]
    pub max_amount_out: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub limit_sell_price: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPlaceLimitOrderResponse {
    #[prost(string, tag = "1")]
    pub tranche_key: ::prost::alloc::string::String,
    /// Total amount of coin used for the limit order
    #[prost(message, optional, tag = "2")]
    pub coin_in: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// Total amount of coin received from the taker portion of the limit order
    /// This is the amount of coin immediately available in the users account after
    /// executing the limit order. It does not include any future proceeds from the
    /// maker portion which will have withdrawn in the future
    #[prost(message, optional, tag = "3")]
    pub taker_coin_out: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawFilledLimitOrder {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tranche_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawFilledLimitOrderResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelLimitOrder {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tranche_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelLimitOrderResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiHopRoute {
    #[prost(string, repeated, tag = "1")]
    pub hops: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMultiHopSwapResponse {
    #[prost(message, optional, tag = "1")]
    pub coin_out: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
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
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LimitOrderTrancheUser {
    #[prost(message, optional, tag = "1")]
    pub trade_pair_id: ::core::option::Option<TradePairId>,
    #[prost(int64, tag = "2")]
    pub tick_index_taker_to_maker: i64,
    #[prost(string, tag = "3")]
    pub tranche_key: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub shares_owned: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub shares_withdrawn: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub shares_cancelled: ::prost::alloc::string::String,
    #[prost(enumeration = "LimitOrderType", tag = "8")]
    pub order_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolMetadata {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(int64, tag = "2")]
    pub tick: i64,
    #[prost(uint64, tag = "3")]
    pub fee: u64,
    #[prost(message, optional, tag = "4")]
    pub pair_id: ::core::option::Option<PairId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickLiquidity {
    #[prost(oneof = "tick_liquidity::Liquidity", tags = "1, 2")]
    pub liquidity: ::core::option::Option<tick_liquidity::Liquidity>,
}
/// Nested message and enum types in `TickLiquidity`.
pub mod tick_liquidity {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Liquidity {
        #[prost(message, tag = "1")]
        PoolReserves(super::PoolReserves),
        #[prost(message, tag = "2")]
        LimitOrderTranche(super::LimitOrderTranche),
    }
}
/// GenesisState defines the dex module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
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
    pub pool_count: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LimitOrderExpiration {
    /// see limitOrderTranche.proto for details on expiration_time
    #[prost(message, optional, tag = "1")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes = "vec", tag = "2")]
    pub tranche_ref: ::prost::alloc::vec::Vec<u8>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetLimitOrderTrancheUserRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tranche_key: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub calc_withdrawable_shares: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetLimitOrderTrancheUserResponse {
    #[prost(message, optional, tag = "1")]
    pub limit_order_tranche_user: ::core::option::Option<LimitOrderTrancheUser>,
    #[prost(string, tag = "2")]
    pub withdrawable_shares: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllLimitOrderTrancheUserRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllLimitOrderTrancheUserResponse {
    #[prost(message, repeated, tag = "1")]
    pub limit_order_tranche_user: ::prost::alloc::vec::Vec<LimitOrderTrancheUser>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetLimitOrderTrancheRequest {
    #[prost(string, tag = "1")]
    pub pair_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub tick_index: i64,
    #[prost(string, tag = "3")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub tranche_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetLimitOrderTrancheResponse {
    #[prost(message, optional, tag = "1")]
    pub limit_order_tranche: ::core::option::Option<LimitOrderTranche>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllLimitOrderTrancheRequest {
    #[prost(string, tag = "1")]
    pub pair_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllLimitOrderTrancheResponse {
    #[prost(message, repeated, tag = "1")]
    pub limit_order_tranche: ::prost::alloc::vec::Vec<LimitOrderTranche>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllUserDepositsRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
    #[prost(bool, tag = "3")]
    pub include_pool_data: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllUserDepositsResponse {
    #[prost(message, repeated, tag = "1")]
    pub deposits: ::prost::alloc::vec::Vec<DepositRecord>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllUserLimitOrdersRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllUserLimitOrdersResponse {
    #[prost(message, repeated, tag = "1")]
    pub limit_orders: ::prost::alloc::vec::Vec<LimitOrderTrancheUser>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllTickLiquidityRequest {
    #[prost(string, tag = "1")]
    pub pair_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllTickLiquidityResponse {
    #[prost(message, repeated, tag = "1")]
    pub tick_liquidity: ::prost::alloc::vec::Vec<TickLiquidity>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetInactiveLimitOrderTrancheRequest {
    #[prost(string, tag = "1")]
    pub pair_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub tick_index: i64,
    #[prost(string, tag = "4")]
    pub tranche_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetInactiveLimitOrderTrancheResponse {
    #[prost(message, optional, tag = "1")]
    pub inactive_limit_order_tranche: ::core::option::Option<LimitOrderTranche>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllInactiveLimitOrderTrancheRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllInactiveLimitOrderTrancheResponse {
    #[prost(message, repeated, tag = "1")]
    pub inactive_limit_order_tranche: ::prost::alloc::vec::Vec<LimitOrderTranche>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllPoolReservesRequest {
    #[prost(string, tag = "1")]
    pub pair_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllPoolReservesResponse {
    #[prost(message, repeated, tag = "1")]
    pub pool_reserves: ::prost::alloc::vec::Vec<PoolReserves>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPoolReservesRequest {
    #[prost(string, tag = "1")]
    pub pair_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub tick_index: i64,
    #[prost(uint64, tag = "4")]
    pub fee: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPoolReservesResponse {
    #[prost(message, optional, tag = "1")]
    pub pool_reserves: ::core::option::Option<PoolReserves>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimateMultiHopSwapRequest {
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimateMultiHopSwapResponse {
    #[prost(message, optional, tag = "1")]
    pub coin_out: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimatePlaceLimitOrderRequest {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token_out: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub tick_index_in_to_out: i64,
    #[prost(string, tag = "6")]
    pub amount_in: ::prost::alloc::string::String,
    #[prost(enumeration = "LimitOrderType", tag = "7")]
    pub order_type: i32,
    /// expirationTime is only valid iff orderType == GOOD_TIL_TIME.
    #[prost(message, optional, tag = "8")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "9")]
    pub max_amount_out: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimatePlaceLimitOrderResponse {
    /// Total amount of coin used for the limit order
    /// You can derive makerLimitInCoin using the equation: totalInCoin =
    /// swapInCoin + makerLimitInCoin
    #[prost(message, optional, tag = "1")]
    pub total_in_coin: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// Total amount of the token in that was immediately swapped for swapOutCoin
    #[prost(message, optional, tag = "2")]
    pub swap_in_coin: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// Total amount of coin received from the taker portion of the limit order
    /// This is the amount of coin immediately available in the users account after
    /// executing the limit order. It does not include any future proceeds from the
    /// maker portion which will have withdrawn in the future
    #[prost(message, optional, tag = "3")]
    pub swap_out_coin: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolRequest {
    #[prost(string, tag = "1")]
    pub pair_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub tick_index: i64,
    #[prost(uint64, tag = "3")]
    pub fee: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolByIdRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolResponse {
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<Pool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPoolMetadataRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPoolMetadataResponse {
    #[prost(message, optional, tag = "1")]
    pub pool_metadata: ::core::option::Option<PoolMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllPoolMetadataRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllPoolMetadataResponse {
    #[prost(message, repeated, tag = "1")]
    pub pool_metadata: ::prost::alloc::vec::Vec<PoolMetadata>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
// @@protoc_insertion_point(module)
