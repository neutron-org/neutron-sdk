use crate::bindings::query::{PageRequest, PageResponse};
use crate::stargate::aux::convert_timestamp;
use crate::stargate::proto_types::neutron::dex::{
    DepositOptions as DepositOptionsGen, MultiHopRoute, QueryAllInactiveLimitOrderTrancheRequest,
    QueryAllLimitOrderTrancheRequest, QueryAllLimitOrderTrancheUserRequest,
    QueryAllPoolMetadataRequest, QueryAllPoolReservesRequest, QueryAllTickLiquidityRequest,
    QueryAllUserDepositsRequest, QueryAllUserLimitOrdersRequest, QueryEstimateMultiHopSwapRequest,
    QueryEstimatePlaceLimitOrderRequest, QueryGetInactiveLimitOrderTrancheRequest,
    QueryGetLimitOrderTrancheRequest, QueryGetLimitOrderTrancheUserRequest,
    QueryGetPoolMetadataRequest, QueryGetPoolReservesRequest, QueryParamsRequest,
    QueryPoolByIdRequest, QueryPoolRequest,
};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest as PageRequestGen;
use cosmwasm_std::{Coin, Int128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LimitOrderTrancheUserResponse {
    pub limit_order_tranche_user: Option<LimitOrderTrancheUser>,
}

impl From<LimitOrderTrancheUserRequest> for QueryGetLimitOrderTrancheUserRequest {
    fn from(v: LimitOrderTrancheUserRequest) -> QueryGetLimitOrderTrancheUserRequest {
        QueryGetLimitOrderTrancheUserRequest {
            address: v.address,
            tranche_key: v.tranche_key,
        }
    }
}

// LimitOrderTrancheUserAll query

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LimitOrderTrancheUserAllRequest {
    pub pagination: Option<PageRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LimitOrderTrancheUserAllRespose {
    pub limit_order_tranche_user: Vec<LimitOrderTrancheUser>,
    pub pagination: Option<PageRequest>,
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
    pub expiration_time: Option<u64>,
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
            expiration_time: v.expiration_time.map(convert_timestamp),
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
#[serde(rename_all = "snake_case")]
pub struct Params {
    pub fee_tiers: Vec<u64>,
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[repr(i32)]
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
#[serde(rename_all = "snake_case")]
pub struct LimitOrderTrancheUser {
    pub trade_pair_id: TradePairID,
    pub tick_index_taker_to_maker: i64,
    pub tranche_key: String,
    pub address: String,
    pub shares_owned: Int128,
    pub shares_withdrawn: Int128,
    pub shares_cancelled: Int128,
    pub order_type: LimitOrderType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LimitOrderTrancheKey {
    pub trade_pair_id: TradePairID,
    pub tick_index_taker_to_maker: i64,
    pub tranche_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LimitOrderTranche {
    pub key: LimitOrderTrancheKey,
    pub reserves_maker_denom: Int128,
    pub reserves_taker_denom: Int128,
    pub total_maker_denom: Int128,
    pub total_taker_denom: Int128,
    pub expiration_time: Option<u64>,
    pub price_taker_to_maker: String, // TODO: refactor to PrecDec
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DepositRecord {
    pub pair_id: PairID,
    pub shares_owned: Int128,
    pub center_tick_index: i64,
    pub lower_tick_index: i64,
    pub upper_tick_index: i64,
    pub fee: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub struct PairID {
    pub token0: String,
    pub token1: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TickLiquidity {
    #[serde(rename = "Liquidity")]
    pub liquidity: Liquidity,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Liquidity {
    PoolReserves(PoolReserves),
    LimitOrderTranche(LimitOrderTranche),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PoolReserves {
    pub key: PoolReservesKey,
    pub reserves_maker_denom: Int128,
    pub price_taker_to_maker: String, // TODO: refactor to PrecDec
    pub price_opposite_taker_to_maker: String, // TODO: refactor to PrecDec
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PoolReservesKey {
    pub trade_pair_id: TradePairID,
    pub tick_index_taker_to_maker: i64,
    pub fee: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TradePairID {
    pub maker_denom: String,
    pub taker_denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Pool {
    #[serde(default)]
    pub id: u64,
    pub lower_tick0: Option<PoolReserves>,
    pub lower_tick1: Option<PoolReserves>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
#[serde(default)]
pub struct PoolMetadata {
    pub id: u64,
    pub tick: i64,
    pub fee: u64,
    pub pair_id: PairID,
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
