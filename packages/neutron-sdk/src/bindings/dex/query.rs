use crate::bindings::dex::types::{
    DepositRecord, LimitOrderTranche, LimitOrderTrancheUser, LimitOrderType, MultiHopRoute, Params,
    Pool, PoolMetadata, PoolReserves, PrecDec, TickLiquidity,
};
use crate::bindings::query::{PageRequest, PageResponse};
use cosmwasm_std::{Coin, Int128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DexQuery {
    /// Parameters queries the parameters of the module.
    Params {},
    /// Queries a LimitOrderTrancheUser by index.
    LimitOrderTrancheUser {
        address: String,
        tranche_key: String,
    },
    /// Queries a list of LimitOrderTrancheMap items.
    LimitOrderTrancheUserAll { pagination: Option<PageRequest> },
    /// Queries a list of LimitOrderTrancheUser items for a given address.
    LimitOrderTrancheUserAllByAddress {
        address: String,
        pagination: Option<PageRequest>,
    },
    /// Queries a LimitOrderTranche by index.
    LimitOrderTranche {
        pair_id: String,
        tick_index: i64,
        token_in: String,
        tranche_key: String,
    },
    /// Queries a list of LimitOrderTranche items for a given pairID / TokenIn combination.
    LimitOrderTrancheAll {
        pair_id: String,
        token_in: String,
        pagination: Option<PageRequest>,
    },
    /// Queries a list of UserDeposits items.
    UserDepositAll {
        address: String,
        include_pool_data: bool,
        pagination: Option<PageRequest>,
    },
    /// Queries a list of TickLiquidity items.
    TickLiquidityAll {
        pair_id: String,
        token_in: String,
        pagination: Option<PageRequest>,
    },
    /// Queries a InactiveLimitOrderTranche by index.
    InactiveLimitOrderTranche {
        pair_id: String,
        tick_index: i64,
        token_in: String,
        tranche_key: String,
    },
    /// Queries a list of InactiveLimitOrderTranche items.
    InactiveLimitOrderTrancheAll { pagination: Option<PageRequest> },
    /// Queries a list of PoolReserves items.
    PoolReservesAll {
        pair_id: String,
        token_in: String,
        pagination: Option<PageRequest>,
    },
    /// Queries a PoolReserve by index
    PoolReserves {
        pair_id: String,
        token_in: String,
        tick_index: i64,
        fee: u64,
    },
    /// Queries the simulated result of a multihop swap
    EstimateMultiHopSwap {
        creator: String,
        receiver: String,
        routes: Vec<MultiHopRoute>,
        amount_in: Int128,
        exit_limit_price: PrecDec,
        pick_best_route: bool,
    },
    /// Queries the simulated result of a PlaceLimit order
    EstimatePlaceLimitOrder {
        creator: String,
        receiver: String,
        token_in: String,
        token_out: String,
        tick_index_in_to_out: i64,
        amount_in: Int128,
        order_type: LimitOrderType,
        // expirationTime is only valid iff orderType == GOOD_TIL_TIME.
        expiration_time: Option<u64>,
        max_amount_out: Option<Int128>,
    },
    /// Queries a pool by pair, tick and fee
    Pool {
        pair_id: String,
        tick_index: i64,
        fee: u64,
    },
    /// Queries a pool by ID
    #[serde(rename = "pool_by_id")]
    PoolByID { pool_id: u64 },
    /// Queries a PoolMetadata by ID
    PoolMetadata { id: u64 },
    /// Queries a list of PoolMetadata items.
    PoolMetadataAll { pagination: Option<PageRequest> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ParamsResponse {
    pub params: Params,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LimitOrderTrancheUserResponse {
    pub limit_order_tranche_user: Option<LimitOrderTrancheUser>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AllLimitOrderTrancheUserResponse {
    #[serde(default)]
    pub limit_order_tranche_user: Vec<LimitOrderTrancheUser>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub struct AllUserLimitOrdersResponse {
    #[serde(default)]
    pub limit_orders: Vec<LimitOrderTrancheUser>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LimitOrderTrancheResponse {
    pub limit_order_tranche: Option<LimitOrderTranche>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub struct AllLimitOrderTrancheResponse {
    #[serde(default)]
    pub limit_order_tranche: Vec<LimitOrderTranche>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub struct AllUserDepositsResponse {
    #[serde(default)]
    pub deposits: Vec<DepositRecord>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AllTickLiquidityResponse {
    pub tick_liquidity: Vec<TickLiquidity>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InactiveLimitOrderTrancheResponse {
    pub inactive_limit_order_tranche: LimitOrderTranche,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AllInactiveLimitOrderTrancheResponse {
    pub inactive_limit_order_tranche: Vec<LimitOrderTranche>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AllPoolReservesResponse {
    pub pool_reserves: Vec<PoolReserves>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PoolReservesResponse {
    pub pool_reserves: PoolReserves,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct EstimateMultiHopSwapResponse {
    pub coin_out: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct EstimatePlaceLimitOrderResponse {
    /// Total amount of coin used for the limit order
    /// You can derive makerLimitInCoin using the equation: totalInCoin = swapInCoin + makerLimitInCoin
    pub total_in_coin: Coin,
    /// Total amount of the token in that was immediately swapped for swapOutCoin
    pub swap_in_coin: Coin,
    /// Total amount of coin received from the taker portion of the limit order
    /// This is the amount of coin immediately available in the users account after executing the
    /// limit order. It does not include any future proceeds from the maker portion which will have withdrawn in the future
    pub swap_out_coin: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PoolResponse {
    pub pool: Pool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PoolMetadataResponse {
    pub pool_metadata: PoolMetadata,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AllPoolMetadataResponse {
    pub pool_metadata: Vec<PoolMetadata>,
    pub pagination: Option<PageResponse>,
}
