use crate::bindings::query::PageRequest;
use crate::proto_types::neutron::dex::{
    DepositOptions as DepositOptionsGen, MsgCancelLimitOrder, MsgDeposit, MsgMultiHopSwap,
    MsgPlaceLimitOrder, MsgWithdrawFilledLimitOrder, MsgWithdrawal, MultiHopRoute,
    QueryAllInactiveLimitOrderTrancheRequest, QueryAllLimitOrderTrancheRequest,
    QueryAllLimitOrderTrancheUserRequest, QueryAllPoolMetadataRequest, QueryAllPoolReservesRequest,
    QueryAllTickLiquidityRequest, QueryAllUserDepositsRequest, QueryAllUserLimitOrdersRequest,
    QueryEstimateMultiHopSwapRequest, QueryEstimatePlaceLimitOrderRequest,
    QueryGetInactiveLimitOrderTrancheRequest, QueryGetLimitOrderTrancheRequest,
    QueryGetLimitOrderTrancheUserRequest, QueryGetPoolMetadataRequest, QueryGetPoolReservesRequest,
    QueryParamsRequest, QueryPoolByIdRequest, QueryPoolRequest,
};
use crate::stargate::aux::proto_timestamp_from_i64;
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest as PageRequestGen;
use cosmwasm_std::{Binary, Coin, Int128, Int64, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use speedate::DateTime;

/// JIT_LIMIT_ORDER_TYPE_EXP_DATE_TIME is the default golang time.Time value used for JIT limit
/// order type in the dex module.
pub const JIT_LIMIT_ORDER_TYPE_EXP_DATE_TIME: &str = "0001-01-01T00:00:00Z";
/// JIT_LIMIT_ORDER_TYPE_EXP_TIMESTAMP is a mock unix timestamp value used to replace timestamp
/// calc for JIT_LIMIT_ORDER_TYPE_EXP_DATE_TIME because the timestamp for this date time is invalid.
pub const JIT_LIMIT_ORDER_TYPE_EXP_TIMESTAMP: i64 = 0;

// Deposit message

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DepositRequest {
    /// The account from which deposit Tokens will be debited.
    pub sender: String,
    /// The account to which PoolShares will be issued.
    pub receiver: String,
    /// Denom for one side of the deposit.
    pub token_a: String,
    /// Denom for the opposing side of the deposit.
    pub token_b: String,
    /// Amounts of token_a to deposit.
    pub amounts_a: Vec<String>,
    /// Amounts of token_b to deposit.
    pub amounts_b: Vec<String>,
    /// Tick indexes to deposit at defined in terms of token_a to token_b (i.e. token_a is on the left).
    pub tick_indexes_a_to_b: Vec<i64>,
    /// Fees to use for each deposit.
    pub fees: Vec<u64>,
    /// Additional deposit options.
    pub options: Vec<DepositOptions>,
}

impl From<DepositRequest> for MsgDeposit {
    fn from(v: DepositRequest) -> MsgDeposit {
        MsgDeposit {
            creator: v.sender,
            receiver: v.receiver,
            token_a: v.token_a,
            token_b: v.token_b,
            amounts_a: v.amounts_a,
            amounts_b: v.amounts_b,
            tick_indexes_a_to_b: v.tick_indexes_a_to_b,
            fees: v.fees,
            options: v.options.into_iter().map(|o| o.into()).collect(),
        }
    }
}

// Withdrawal message

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct WithdrawalRequest {
    /// The account from which the PoolShares are removed.
    pub sender: String,
    /// The account to which the tokens are credited.
    pub receiver: String,
    /// Denom for one side of the deposit.
    pub token_a: String,
    /// Denom for the opposing side of the deposit.
    pub token_b: String,
    /// Amount of shares to remove from each pool.
    pub shares_to_remove: Vec<String>,
    /// Tick indexes of the target LiquidityPools defined in terms of tokan_a to token_b (i.e.
    /// token_a is on the left).
    pub tick_indexes_a_to_b: Vec<i64>,
    /// Fee for the target LiquidityPools.
    pub fees: Vec<u64>,
}

impl From<WithdrawalRequest> for MsgWithdrawal {
    fn from(v: WithdrawalRequest) -> MsgWithdrawal {
        MsgWithdrawal {
            creator: v.sender,
            receiver: v.receiver,
            token_a: v.token_a,
            token_b: v.token_b,
            shares_to_remove: v.shares_to_remove,
            tick_indexes_a_to_b: v.tick_indexes_a_to_b,
            fees: v.fees,
        }
    }
}

// PlaceLimitOrder message

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PlaceLimitOrderRequest {
    /// Account from which token_in is debited.
    pub sender: String,
    /// Account to which token_out is credited or that will be allowed to withdraw or cancel a
    /// maker order.
    pub receiver: String,
    /// Token being “sold”.
    pub token_in: String,
    /// Token being “bought”.
    pub token_out: String,
    /// Limit tick for a limit order, specified in terms of token_in to token_out.
    pub tick_index_in_to_out: i64,
    /// limit sell price when selling token_in.
    /// Accepts standard decimals and decimals with scientific notation (ie. 1234.23E-7)
    pub limit_sell_price: String,
    /// Amount of TokenIn to be traded.
    pub amount_in: String,
    /// Type of limit order to be used.
    pub order_type: LimitOrderType,
    /// Expiration time for order. Only valid for GoodTilTime limit orders.
    pub expiration_time: Option<i64>,
    pub max_amount_out: Option<String>,
}

const PREC_DEC_PRECISION: usize = 27;

fn serialize_prec_dec(decimal_str: String) -> String {
    // The proto marshaller expects the decimal to come as an integer that will be divided by 10^PREC_DEC_PRECISION to produce a PrecDec
    // There is no available decimal type that can hold 27 decimals of precision. So instead we use string manipulation to serialize the PrecDec into an integer
    let parts: Vec<&str> = decimal_str.split('.').collect();
    let integer_part = parts[0];
    let mut fractional_part = if parts.len() > 1 {
        String::from(parts[1])
    } else {
        String::new()
    };
    // Remove trailing zeros from the fractional_part
    fractional_part = fractional_part.trim_end_matches('0').to_string();

    // Remove leading zeros from the integer_part
    let mut result = integer_part.trim_start_matches('0').to_string();

    // combine integer part and fractional part
    result.push_str(&fractional_part.to_owned());

    // Add zeros to the end. This is the equivalent of multiplying by 10^PREC_DEC_PRECISION
    let zeros_to_add = PREC_DEC_PRECISION
        .checked_sub(fractional_part.len())
        .expect("Cannot retain precision when serializing PrecDec");
    for _ in 0..zeros_to_add {
        result.push('0');
    }

    result
}

impl From<PlaceLimitOrderRequest> for MsgPlaceLimitOrder {
    fn from(v: PlaceLimitOrderRequest) -> MsgPlaceLimitOrder {
        #[allow(deprecated)] // tick_index_in_to_out will be removed in the next release
        MsgPlaceLimitOrder {
            creator: v.sender,
            receiver: v.receiver,
            token_in: v.token_in,
            token_out: v.token_out,
            tick_index_in_to_out: v.tick_index_in_to_out,
            amount_in: v.amount_in,
            order_type: v.order_type as i32,
            expiration_time: v.expiration_time.map(proto_timestamp_from_i64),
            max_amount_out: v.max_amount_out.unwrap_or_default(),
            limit_sell_price: serialize_prec_dec(v.limit_sell_price),
        }
    }
}

// WithdrawFilledLimitOrder message

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct WithdrawFilledLimitOrderRequest {
    /// Account which controls the limit order and to which proceeds are credited.
    pub sender: String,
    /// Tranche key for the target limit order.
    pub tranche_key: String,
}

impl From<WithdrawFilledLimitOrderRequest> for MsgWithdrawFilledLimitOrder {
    fn from(v: WithdrawFilledLimitOrderRequest) -> MsgWithdrawFilledLimitOrder {
        MsgWithdrawFilledLimitOrder {
            creator: v.sender,
            tranche_key: v.tranche_key,
        }
    }
}

// CancelLimitOrder message

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CancelLimitOrderRequest {
    /// Account which controls the limit order and to which any untraded amount is credited.
    pub sender: String,
    /// Tranche key for the target limit order.
    pub tranche_key: String,
}

impl From<CancelLimitOrderRequest> for MsgCancelLimitOrder {
    fn from(v: CancelLimitOrderRequest) -> MsgCancelLimitOrder {
        MsgCancelLimitOrder {
            creator: v.sender,
            tranche_key: v.tranche_key,
        }
    }
}

// MultiHopSwap message

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MultiHopSwapRequest {
    /// Account from which token_in is debited.
    pub sender: String,
    /// Account to which token_out is credited.
    pub receiver: String,
    /// Array of possible routes. E.g. [[“token_a”, “token_c”, “token_d”, “token_b”]]. The complete
    /// amount of specified by `amount_in` will always be used. If there is insufficient liquidity
    /// in a route to swap 100% of the `amount_in` the route will fail. The first route that does
    /// not run out of liquidity, hit the `exit_limit_price` or return an error will be used.
    pub routes: Vec<Vec<String>>,
    /// Amount of token_in to swap.
    pub amount_in: String,
    /// Minimum price that that must be satisfied for a route to succeed.
    pub exit_limit_price: String,
    /// If true all routes are run and the route with the best price is used.
    pub pick_best_route: bool,
}

impl From<MultiHopSwapRequest> for MsgMultiHopSwap {
    fn from(v: MultiHopSwapRequest) -> MsgMultiHopSwap {
        MsgMultiHopSwap {
            creator: v.sender,
            receiver: v.receiver,
            routes: v
                .routes
                .into_iter()
                .map(|r| MultiHopRoute { hops: r })
                .collect(),
            amount_in: v.amount_in,
            exit_limit_price: v.exit_limit_price,
            pick_best_route: v.pick_best_route,
        }
    }
}

// Params query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ParamsRequest {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ParamsResponse {
    pub params: Params,
}

impl From<ParamsRequest> for QueryParamsRequest {
    fn from(_: ParamsRequest) -> QueryParamsRequest {
        QueryParamsRequest {}
    }
}

// LimitOrderTrancheUser query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LimitOrderTrancheUserRequest {
    pub address: String,
    pub tranche_key: String,
    pub calc_withdrawable_shares: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LimitOrderTrancheUserResponse {
    pub limit_order_tranche_user: Option<LimitOrderTrancheUser>,
    pub withdrawable_shares: Option<String>,
}

impl From<LimitOrderTrancheUserRequest> for QueryGetLimitOrderTrancheUserRequest {
    fn from(v: LimitOrderTrancheUserRequest) -> QueryGetLimitOrderTrancheUserRequest {
        QueryGetLimitOrderTrancheUserRequest {
            address: v.address,
            tranche_key: v.tranche_key,
            calc_withdrawable_shares: v.calc_withdrawable_shares,
        }
    }
}

// LimitOrderTrancheUserAll query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LimitOrderTrancheUserAllRequest {
    pub pagination: Option<PageRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LimitOrderTrancheUserAllResponse {
    pub limit_order_tranche_user: Vec<LimitOrderTrancheUser>,
    pub pagination: Option<PageResponse>,
}

impl From<LimitOrderTrancheUserAllRequest> for QueryAllLimitOrderTrancheUserRequest {
    fn from(v: LimitOrderTrancheUserAllRequest) -> QueryAllLimitOrderTrancheUserRequest {
        QueryAllLimitOrderTrancheUserRequest {
            pagination: convert_page_request(v.pagination),
        }
    }
}

// LimitOrderTrancheUserAllByAddress query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllUserLimitOrdersRequest {
    pub address: String,
    pub pagination: Option<PageRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllUserLimitOrdersResponse {
    pub limit_orders: Vec<LimitOrderTrancheUser>,
    pub pagination: Option<PageResponse>,
}

impl From<AllUserLimitOrdersRequest> for QueryAllUserLimitOrdersRequest {
    fn from(v: AllUserLimitOrdersRequest) -> QueryAllUserLimitOrdersRequest {
        QueryAllUserLimitOrdersRequest {
            address: v.address,
            pagination: convert_page_request(v.pagination),
        }
    }
}

// LimitOrderTranche query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetLimitOrderTrancheRequest {
    pub pair_id: String,
    pub tick_index: i64,
    pub token_in: String,
    pub tranche_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetLimitOrderTrancheResponse {
    pub limit_order_tranche: Option<LimitOrderTranche>,
}

impl From<GetLimitOrderTrancheRequest> for QueryGetLimitOrderTrancheRequest {
    fn from(v: GetLimitOrderTrancheRequest) -> QueryGetLimitOrderTrancheRequest {
        QueryGetLimitOrderTrancheRequest {
            pair_id: v.pair_id,
            tick_index: v.tick_index,
            token_in: v.token_in,
            tranche_key: v.tranche_key,
        }
    }
}

// LimitOrderTrancheAll query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllLimitOrderTrancheRequest {
    pub pair_id: String,
    pub token_in: String,
    pub pagination: Option<PageRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllLimitOrderTrancheResponse {
    pub limit_order_tranche: Vec<LimitOrderTranche>,
    pub pagination: Option<PageResponse>,
}

impl From<AllLimitOrderTrancheRequest> for QueryAllLimitOrderTrancheRequest {
    fn from(v: AllLimitOrderTrancheRequest) -> QueryAllLimitOrderTrancheRequest {
        QueryAllLimitOrderTrancheRequest {
            pair_id: v.pair_id,
            token_in: v.token_in,
            pagination: convert_page_request(v.pagination),
        }
    }
}

// UserDepositsAll query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllUserDepositsRequest {
    pub address: String,
    pub include_pool_data: bool,
    pub pagination: Option<PageRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllUserDepositsResponse {
    pub deposits: Vec<DepositRecord>,
    pub pagination: Option<PageResponse>,
}

impl From<AllUserDepositsRequest> for QueryAllUserDepositsRequest {
    fn from(v: AllUserDepositsRequest) -> QueryAllUserDepositsRequest {
        QueryAllUserDepositsRequest {
            address: v.address,
            pagination: convert_page_request(v.pagination),
            include_pool_data: v.include_pool_data,
        }
    }
}

// TickLiquidityAll query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllTickLiquidityRequest {
    pub pair_id: String,
    pub token_in: String,
    pub pagination: Option<PageRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllTickLiquidityResponse {
    pub tick_liquidity: Vec<TickLiquidity>,
    pub pagination: Option<PageResponse>,
}

impl From<AllTickLiquidityRequest> for QueryAllTickLiquidityRequest {
    fn from(v: AllTickLiquidityRequest) -> QueryAllTickLiquidityRequest {
        QueryAllTickLiquidityRequest {
            pair_id: v.pair_id,
            token_in: v.token_in,
            pagination: convert_page_request(v.pagination),
        }
    }
}

// InactiveLimitOrderTranche query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetInactiveLimitOrderTrancheRequest {
    pub pair_id: String,
    pub token_in: String,
    pub tick_index: i64,
    pub tranche_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetInactiveLimitOrderTrancheResponse {
    pub inactive_limit_order_tranche: LimitOrderTranche,
}

impl From<GetInactiveLimitOrderTrancheRequest> for QueryGetInactiveLimitOrderTrancheRequest {
    fn from(v: GetInactiveLimitOrderTrancheRequest) -> QueryGetInactiveLimitOrderTrancheRequest {
        QueryGetInactiveLimitOrderTrancheRequest {
            pair_id: v.pair_id,
            token_in: v.token_in,
            tick_index: v.tick_index,
            tranche_key: v.tranche_key,
        }
    }
}

// InactiveLimitOrderTrancheAll query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllInactiveLimitOrderTrancheRequest {
    pub pagination: Option<PageRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllInactiveLimitOrderTrancheResponse {
    pub inactive_limit_order_tranche: Vec<LimitOrderTranche>,
    pub pagination: Option<PageResponse>,
}

impl From<AllInactiveLimitOrderTrancheRequest> for QueryAllInactiveLimitOrderTrancheRequest {
    fn from(v: AllInactiveLimitOrderTrancheRequest) -> QueryAllInactiveLimitOrderTrancheRequest {
        QueryAllInactiveLimitOrderTrancheRequest {
            pagination: convert_page_request(v.pagination),
        }
    }
}

// PoolReservesAll query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllPoolReservesRequest {
    pub pair_id: String,
    pub token_in: String,
    pub pagination: Option<PageRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllPoolReservesResponse {
    pub pool_reserves: Vec<PoolReserves>,
    pub pagination: Option<PageResponse>,
}

impl From<AllPoolReservesRequest> for QueryAllPoolReservesRequest {
    fn from(v: AllPoolReservesRequest) -> QueryAllPoolReservesRequest {
        QueryAllPoolReservesRequest {
            pair_id: v.pair_id,
            token_in: v.token_in,
            pagination: convert_page_request(v.pagination),
        }
    }
}

// PoolReserves query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetPoolReservesRequest {
    pub pair_id: String,
    pub token_in: String,
    pub tick_index: i64,
    pub fee: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetPoolReservesResponse {
    pub pool_reserves: PoolReserves,
}

impl From<GetPoolReservesRequest> for QueryGetPoolReservesRequest {
    fn from(v: GetPoolReservesRequest) -> QueryGetPoolReservesRequest {
        QueryGetPoolReservesRequest {
            pair_id: v.pair_id,
            token_in: v.token_in,
            tick_index: v.tick_index,
            fee: v.fee,
        }
    }
}

// EstimateMultiHopSwap query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct EstimateMultiHopSwapRequest {
    pub creator: String,
    pub receiver: String,
    pub routes: Vec<Vec<String>>,
    pub amount_in: String,
    pub exit_limit_price: String,
    pub pick_best_route: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct EstimateMultiHopSwapResponse {
    pub coin_out: Coin,
}

impl From<EstimateMultiHopSwapRequest> for QueryEstimateMultiHopSwapRequest {
    fn from(v: EstimateMultiHopSwapRequest) -> QueryEstimateMultiHopSwapRequest {
        QueryEstimateMultiHopSwapRequest {
            creator: v.creator,
            receiver: v.receiver,
            routes: v
                .routes
                .into_iter()
                .map(|r| MultiHopRoute { hops: r })
                .collect(),
            amount_in: v.amount_in,
            exit_limit_price: v.exit_limit_price,
            pick_best_route: v.pick_best_route,
        }
    }
}

// EstimatePlaceLimitOrder query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct EstimatePlaceLimitOrderRequest {
    pub creator: String,
    pub receiver: String,
    pub token_in: String,
    pub token_out: String,
    pub tick_index_in_to_out: i64,
    pub amount_in: String,
    pub order_type: LimitOrderType,
    pub expiration_time: Option<i64>,
    pub max_amount_out: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct EstimatePlaceLimitOrderResponse {
    // Total amount of coin used for the limit order
    // You can derive makerLimitInCoin using the equation: totalInCoin = swapInCoin + makerLimitInCoin
    pub total_in_coin: Coin,
    // Total amount of the token in that was immediately swapped for swapOutCoin
    pub swap_in_coin: Coin,
    // Total amount of coin received from the taker portion of the limit order
    // This is the amount of coin immediately available in the users account after executing the
    // limit order. It does not include any future proceeds from the maker portion which will have withdrawn in the future
    pub swap_out_coin: Coin,
}

impl From<EstimatePlaceLimitOrderRequest> for QueryEstimatePlaceLimitOrderRequest {
    fn from(v: EstimatePlaceLimitOrderRequest) -> QueryEstimatePlaceLimitOrderRequest {
        QueryEstimatePlaceLimitOrderRequest {
            creator: v.creator,
            receiver: v.receiver,
            token_in: v.token_in,
            token_out: v.token_out,
            tick_index_in_to_out: v.tick_index_in_to_out,
            amount_in: v.amount_in,
            order_type: v.order_type as i32,
            expiration_time: v.expiration_time.map(proto_timestamp_from_i64),
            max_amount_out: v.max_amount_out.unwrap_or_default(),
        }
    }
}

// Pool query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PoolRequest {
    pub pair_id: String,
    pub tick_index: i64,
    pub fee: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PoolResponse {
    pub pool: Pool,
}

impl From<PoolRequest> for QueryPoolRequest {
    fn from(v: PoolRequest) -> QueryPoolRequest {
        QueryPoolRequest {
            pair_id: v.pair_id,
            tick_index: v.tick_index,
            fee: v.fee,
        }
    }
}

// PoolByID query

pub struct PoolByIdRequest {
    pub pool_id: u64,
}

impl From<PoolByIdRequest> for QueryPoolByIdRequest {
    fn from(v: PoolByIdRequest) -> QueryPoolByIdRequest {
        QueryPoolByIdRequest { pool_id: v.pool_id }
    }
}

// PoolMetadata query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetPoolMetadataRequest {
    pub id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetPoolMetadataResponse {
    #[serde(rename(deserialize = "Pool_metadata"))]
    pub pool_metadata: PoolMetadata,
}

impl From<GetPoolMetadataRequest> for QueryGetPoolMetadataRequest {
    fn from(v: GetPoolMetadataRequest) -> QueryGetPoolMetadataRequest {
        QueryGetPoolMetadataRequest { id: v.id }
    }
}

// PoolMetadataAll query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllPoolMetadataRequest {
    pub pagination: Option<PageRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllPoolMetadataResponse {
    pub pool_metadata: Vec<PoolMetadata>,
    pub pagination: Option<PageResponse>,
}

impl From<AllPoolMetadataRequest> for QueryAllPoolMetadataRequest {
    fn from(v: AllPoolMetadataRequest) -> QueryAllPoolMetadataRequest {
        QueryAllPoolMetadataRequest {
            pagination: convert_page_request(v.pagination),
        }
    }
}

// Common

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DepositOptions {
    /// Autoswap provides a mechanism for users to deposit the entirety of their specified deposit
    /// amounts by paying a small fee. By default the `autoswap` option is enabled.
    pub disable_autoswap: bool,
}

impl From<DepositOptions> for DepositOptionsGen {
    fn from(o: DepositOptions) -> DepositOptionsGen {
        DepositOptionsGen {
            disable_autoswap: o.disable_autoswap,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Params {
    pub fee_tiers: Vec<Uint64>,
    pub paused: bool,
    pub max_jits_per_block: Uint64,
    pub good_til_purge_allowance: Uint64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[schemars(with = "String")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LimitOrderType {
    GoodTilCancelled = 0,
    FillOrKill = 1,
    ImmediateOrCancel = 2,
    JustInTime = 3,
    GoodTilTime = 4,
}

impl TryFrom<i32> for LimitOrderType {
    type Error = String;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(LimitOrderType::GoodTilCancelled),
            1 => Ok(LimitOrderType::FillOrKill),
            2 => Ok(LimitOrderType::ImmediateOrCancel),
            3 => Ok(LimitOrderType::JustInTime),
            4 => Ok(LimitOrderType::GoodTilTime),
            _ => Err(format!(
                "invalid numeric value for LimitOrderType {}: expected 0-4",
                v
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LimitOrderTrancheUser {
    pub trade_pair_id: TradePairID,
    pub tick_index_taker_to_maker: Int64,
    pub tranche_key: String,
    pub address: String,
    pub shares_owned: Int128,
    pub shares_withdrawn: Int128,
    pub shares_cancelled: Int128,
    pub order_type: LimitOrderType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LimitOrderTrancheKey {
    pub trade_pair_id: TradePairID,
    pub tick_index_taker_to_maker: Int64,
    pub tranche_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LimitOrderTranche {
    pub key: LimitOrderTrancheKey,
    pub reserves_maker_denom: Int128,
    pub reserves_taker_denom: Int128,
    pub total_maker_denom: Int128,
    pub total_taker_denom: Int128,
    /// unix timestamp in seconds;
    #[serde(deserialize_with = "deserialize_expiration_time")]
    pub expiration_time: Option<Int64>,
    /// a decimal with precision equal to 26
    pub price_taker_to_maker: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DepositRecord {
    pub pair_id: PairID,
    pub shares_owned: Int128,
    pub center_tick_index: Int64,
    pub lower_tick_index: Int64,
    pub upper_tick_index: Int64,
    pub fee: Option<Int64>,
    total_shares: Option<Int128>,
    pool: Option<Pool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
pub struct PairID {
    pub token0: String,
    pub token1: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TickLiquidity {
    PoolReserves(PoolReserves),
    LimitOrderTranche(LimitOrderTranche),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PoolReserves {
    pub key: PoolReservesKey,
    pub reserves_maker_denom: Int128,
    /// a decimal with precision equal to 26
    pub price_taker_to_maker: String,
    /// a decimal with precision equal to 26
    pub price_opposite_taker_to_maker: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PoolReservesKey {
    pub trade_pair_id: TradePairID,
    pub tick_index_taker_to_maker: Int64,
    pub fee: Option<Uint64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TradePairID {
    pub maker_denom: String,
    pub taker_denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Pool {
    pub id: Uint64,
    pub lower_tick0: Option<PoolReserves>,
    pub lower_tick1: Option<PoolReserves>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
pub struct PoolMetadata {
    pub id: Uint64,
    pub tick: Int64,
    pub fee: Uint64,
    pub pair_id: PairID,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageResponse {
    /// **next_key** is the key to be passed to PageRequest.key to
    /// query the next page most efficiently. It will be empty if
    /// there are no more results.
    pub next_key: Option<Binary>,
    /// **total** is total number of results available if PageRequest.count_total
    /// was set, its value is undefined otherwise
    pub total: Option<Uint64>,
}

fn convert_page_request(page_request: Option<PageRequest>) -> Option<PageRequestGen> {
    match page_request {
        Some(p) => Some(PageRequestGen {
            key: p.key.into(),
            offset: p.offset,
            limit: p.limit,
            count_total: p.count_total,
            reverse: p.reverse,
        }),
        None => None,
    }
}

/// deserialize_expiration_time deserealizes an optional expiration_time value on dex module's rules:
/// - if it's None, it returns None (non-expiring limit orders);
/// - if it's a default golang time.Time value used for JIT limit order type (0001-01-01T00:00:00Z),
/// it returns 0, because the timestamp for this value is invalid (-62135596800);
/// - in the rest of the cases, it assumes it's a valid RFC 3339 formatted date time and tries to
/// parse it and returns a unix timestamp.
fn deserialize_expiration_time<'de, D>(deserializer: D) -> Result<Option<Int64>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the field as an Option<&str>
    let opt_date_time_string: Option<&str> = Option::deserialize(deserializer)?;

    match opt_date_time_string {
        None => Ok(None),

        Some(date_time_str) => match date_time_str {
            JIT_LIMIT_ORDER_TYPE_EXP_DATE_TIME => {
                Ok(Some(JIT_LIMIT_ORDER_TYPE_EXP_TIMESTAMP.into()))
            }

            // some RFC 3339 formatted date time to be parsed to a unix timestamp
            _ => Ok(Some(
                DateTime::parse_str_rfc3339(date_time_str)
                    .map_err(|_| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(date_time_str),
                            &"an RFC 3339 formatted date time",
                        )
                    })?
                    .timestamp()
                    .into(),
            )),
        },
    }
}
