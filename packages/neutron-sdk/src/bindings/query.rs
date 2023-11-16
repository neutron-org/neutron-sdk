use crate::bindings::types::{Failure, InterchainQueryResult, RegisteredQuery};
use cosmwasm_std::{Binary, Coin, CustomQuery, Int128, QueryRequest, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{
    msg::{MultiHopRoute, PrecDec},
    types::{LimitOrderTranche, LimitOrderTrancheUser, LimitOrderType, TradePairID},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// The queries to interact with neutron specific blockchain modules.
pub enum NeutronQuery {
    /// Query a result of registered interchain query on remote chain
    InterchainQueryResult {
        /// **query_id** is an ID registered interchain query
        query_id: u64,
    },

    /// Query a registered interchain account address for a specific connection_id
    /// Every contract may have as many interchain accounts as necessary.
    InterchainAccountAddress {
        /// **owner_address** is an address of contract which registered interchain account
        owner_address: String,

        /// **interchain_account_id** is an identifier of your interchain account. Can be any string
        /// This identifier allows contracts to have multiple interchain accounts on remote chains
        interchain_account_id: String,

        /// **connection_id** is an IBC connection identifier between Neutron and remote chain
        connection_id: String,
    },

    /// Query all registered interchain queries on all remote chains
    RegisteredInterchainQueries {
        owners: Vec<String>,
        connection_id: String,
        pagination: PageRequest,
    },

    /// Query registered interchain query with a specific query_id
    RegisteredInterchainQuery {
        /// **query_id** is an ID registered interchain query
        query_id: u64,
    },

    /// Query total amount of burned neutron fees
    TotalBurnedNeutronsAmount {},

    /// Query minimum IBC fee
    MinIbcFee {},

    /// TokenFactory query. Given a subdenom minted by a contract via
    /// [`NeutronMsg::MintTokens`](crate::bindings::msg::NeutronMsg::MintTokens),
    /// returns the full denom as used by [`BankMsg::Send`](cosmwasm_std::BankMsg::Send).
    FullDenom {
        creator_addr: String,
        subdenom: String,
    },

    /// TokenFactory query. Returns the admin of a denom, if the denom is a TokenFactory denom.
    DenomAdmin {
        subdenom: String,
    },

    /// TokenFactory query. Returns the before send hook address of a denom, if the denom is a TokenFactory denom.
    BeforeSendHook {
        denom: String,
    },

    /// Contractmanager query. Returns the failures for a particular contract address.
    Failures {
        address: String,
        pagination: PageRequest,
    },

    Dex(DexQuery),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DexQuery {
    // Parameters queries the parameters of the module.
    Params {},
    // Queries a LimitOrderTrancheUser by index.
    LimitOrderTrancheUser {
        address: String,
        tranche_key: String,
    },
    // Queries a list of LimitOrderTrancheMap items.
    LimitOrderTrancheUserAll {
        pagination: Option<PageRequest>,
    },
    // Queries a list of LimitOrderTrancheUser items for a given address.
    LimitOrderTrancheUserAllByAddress {
        address: String,
        pagination: Option<PageRequest>,
    },
    // Queries a LimitOrderTranche by index.
    LimitOrderTranche {
        pair_id: String,
        tick_index: i64,
        token_in: String,
        tranche_key: String,
    },
    // Queries a list of LimitOrderTranche items for a given pairID / TokenIn combination.
    LimitOrderTrancheAll {
        pair_id: String,
        token_in: String,
        pagination: Option<PageRequest>,
    },
    // Queries a list of UserDeposits items.
    UserDepositAll {
        address: String,
        pagination: Option<PageRequest>,
    },
    // Queries a list of TickLiquidity items.
    TickLiquidityAll {
        pair_id: String,
        token_in: String,
        pagination: Option<PageRequest>,
    },
    // Queries a InactiveLimitOrderTranche by index.
    InactiveLimitOrderTranche {
        pair_id: String,
        tick_index: i64,
        token_in: String,
        tranche_key: String,
    },
    // Queries a list of InactiveLimitOrderTranche items.
    InactiveLimitOrderTrancheAll {
        pagination: Option<PageRequest>,
    },
    // Queries a list of PoolReserves items.
    PoolReservesAll {
        pair_id: String,
        token_in: String,
        pagination: Option<PageRequest>,
    },
    // Queries a PoolReserve by index
    PoolReserves {
        pair_id: String,
        token_in: String,
        tick_index: i64,
        fee: u64,
    },
    // Queries the simulated result of a multihop swap
    EstimateMultiHopSwap {
        creator: String,
        receiver: String,
        routes: Vec<MultiHopRoute>,
        amount_id: Int128,
        exit_limit_price: PrecDec,
        pick_best_route: bool,
    },
    // // Queries the simulated result of a PlaceLimit order
    EstimatePlaceLimitOrder {
        creator: String,
        receiver: String,
        token_in: String,
        token_out: String,
        tick_index_in_to_out: Uint128,
        order_type: LimitOrderType,
        // expirationTime is only valid iff orderType == GOOD_TIL_TIME.
        expiration_time: Option<u64>,
        max_amount_out: Option<Int128>,
    },
    // Queries a pool by pair, tick and fee
    Pool {
        pair_id: String,
        tick_index: i64,
        fee: u64,
    },
    // Queries a pool by ID
    #[serde(rename = "pool_by_id")]
    PoolByID {
        pool_id: u64,
    },
    // Queries a PoolMetadata by ID
    PoolMetadata {
        id: u64,
    },
    // Queries a list of PoolMetadata items.
    PoolMetadataAll {
        pagination: Option<PageRequest>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Params {
    fee_tiers: Vec<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ParamsResponse {
    params: Params,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LimitOrderTrancheUserResponse {
    limit_order_tranche_user: Option<LimitOrderTrancheUser>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AllLimitOrderTrancheUserResponse {
    #[serde(default)]
    limit_order_tranche_user: Vec<LimitOrderTrancheUser>,
    pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub struct AllUserLimitOrdersResponse {
    #[serde(default)]
    limit_orders: Vec<LimitOrderTrancheUser>,
    pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LimitOrderTrancheResponse {
    limit_order_tranche: Option<LimitOrderTranche>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub struct AllLimitOrderTrancheResponse {
    #[serde(default)]
    limit_order_tranche: Vec<LimitOrderTranche>,
    pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub struct AllUserDepositsResponse {
    #[serde(default)]
    deposits: Vec<DepositRecord>,
    pagination: Option<PageResponse>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DepositRecord {
    pair_id: PairID,
    shares_owned: Int128,
    center_tick_index: i64,
    lower_tick_index: i64,
    upper_tick_index: i64,
    fee: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PairID {
    token0: String,
    token1: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AllTickLiquidityResponse {
    tick_liquidity: Vec<TickLiquidity>,
    pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TickLiquidity {
    #[serde(rename = "Liquidity")]
    liquidity: Liquidity,
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
    key: PoolReservesKey,
    reserves_maker_denom: Int128,
    price_taker_to_maker: PrecDec,
    price_opposite_taker_to_maker: PrecDec,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PoolReservesKey {
    trade_pair_id: TradePairID,
    tick_index_taker_to_maker: i64,
    fee: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InactiveLimitOrderTrancheResponse {
    inactive_limit_order_tranche: LimitOrderTranche,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AllInactiveLimitOrderTrancheResponse {
    inactive_limit_order_tranche: Vec<LimitOrderTranche>,
    pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AllPoolReservesResponse {
    pool_reserves: Vec<PoolReserves>,
    pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PoolReservesResponse {
    pool_reserves: PoolReserves,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct EstimateMultiHopSwapResponse {
    coin_out: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct EstimatePlaceLimitOrderResponse {
    // Total amount of coin used for the limit order
    // You can derive makerLimitInCoin using the equation: totalInCoin = swapInCoin + makerLimitInCoin
    total_in_coin: Coin,
    // Total amount of the token in that was immediately swapped for swapOutCoin
    swap_in_coin: Coin,
    // Total amount of coin received from the taker portion of the limit order
    // This is the amount of coin immediately available in the users account after executing the
    // limit order. It does not include any future proceeds from the maker portion which will have withdrawn in the future
    swap_out_coin: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PoolResponse {
    pool: Pool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Pool {
    id: u64,
    lower_tick0: Option<PoolReserves>,
    lower_tick1: Option<PoolReserves>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PoolMetadataResponse {
    pool_metadata: PoolMetadata,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PoolMetadata {
    id: u64,
    tick: i64,
    fee: u64,
    pair_id: PairID,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AllPoolMetadataResponse {
    pool_metadata: Vec<PoolMetadata>,
    pagination: Option<PageResponse>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageRequest {
    /// **key** is a value returned in PageResponse.next_key to begin
    /// querying the next page most efficiently. Only one of offset or key
    /// should be set.
    pub key: Binary,
    /// **offset** is a numeric offset that can be used when key is unavailable.
    /// It is less efficient than using key. Only one of offset or key should
    /// be set.
    pub offset: u64,
    /// **limit** is the total number of results to be returned in the result page.
    /// If left empty it will default to a value to be set by each app.
    pub limit: u64,
    /// **count_total** is set to true  to indicate that the result set should include
    /// a count of the total number of items available for pagination in UIs.
    /// count_total is only respected when offset is used. It is ignored when key
    /// is set.
    pub count_total: bool,
    /// reverse is set to true if results are to be returned in the descending order.
    pub reverse: bool,
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
    pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredQueriesResponse {
    /// **registered_queries** is a list of registered queries
    pub registered_queries: Vec<RegisteredQuery>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredQueryResponse {
    /// **registered_query** is a registered query
    pub registered_query: RegisteredQuery,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredQueryResultResponse {
    pub result: InterchainQueryResult,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryInterchainAccountAddressResponse {
    /// **interchain_account_address** is a interchain account address on the remote chain
    pub interchain_account_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryFailuresResponse {
    /// **failures** is a list of failures of sudo handler calls
    pub failures: Vec<Failure>,
}

impl CustomQuery for NeutronQuery {}

impl From<DexQuery> for QueryRequest<NeutronQuery> {
    fn from(msg: DexQuery) -> Self {
        QueryRequest::Custom(NeutronQuery::Dex(msg))
    }
}
