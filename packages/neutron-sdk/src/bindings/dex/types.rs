use cosmwasm_std::Int128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LimitOrderType {
    #[default]
    /// Good-til-Cancelled limit orders are hybrid maker and taker limit orders.
    /// They will attempt to trade the supplied AmountIn at the TickIndex or better.
    /// However, if they total AmountIn cannot be traded at the limit price they are remaining
    /// amount will be placed as a maker limit order. The proceeds from the taker portion
    /// are deposited into the user’s account immediately, however, the proceeds from the
    /// maker portion must be explicitly withdrawn via WithdrawLimitOrder.
    GoodTilCancelled,
    /// Fill-or-Kill limit orders are taker limit orders that either successfully swap 100%
    /// of the supplied AmountIn or return an error. If there is insufficient liquidity to
    /// complete the trade at or above the supplied TickIndex a Fill-or-Kill order will
    /// return an error `codespace: dex, code: 1134`
    /// (<https://github.com/neutron-org/neutron/blob/main/x/dex/types/errors.go#L107> ErrGoodTilOrderWithoutExpiration).
    FillOrKill,
    /// Immediate-or-Cancel limit orders are taker orders that will swap as much as of the
    /// AmountIn as possible given available liquidity above the supplied TickIndex.
    /// Unlike Fill-or-Kill orders they will still successfully complete even if they
    /// are only able to partially trade through the AmountIn at the TickIndex or better.
    ImmediateOrCancel,
    /// Just-in-Time limit orders are an advanced maker limit order that provides tradeable
    /// liquidity for exactly one block. At the end of the same block in which the Just-in-Time
    /// order was submitted the order is canceled and any untraded portion will no longer be
    /// usable as active liquidity.
    JustInTime,
    /// Good-til-Time limit order function exactly the same as Good-til-Cancelled limit orders
    /// first trying to trade as a taker limit order and then placing any remaining amount
    /// as a maker limit order. However, the maker portion of the limit order has a specified ExpirationTime.
    /// After the ExpirationTime the order will be cancelled and can no longer be traded against.
    /// When withdrawing a Good-til-Time limit order the user will receive both the successfully
    /// traded portion of the limit order (TokenOut) as well as any remaining untraded amount (TokenIn).
    GoodTilTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
#[serde(rename_all = "snake_case", default)]
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
    pub price_taker_to_maker: PrecDec,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub struct TradePairID {
    pub maker_denom: String,
    pub taker_denom: String,
}

// TODO implement math for PrecDec
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(from = "String")]
#[serde(into = "String")]
pub struct PrecDec {
    pub i: String,
}

#[allow(clippy::from_over_into)]
impl Into<String> for PrecDec {
    fn into(self) -> String {
        self.i
    }
}

impl From<String> for PrecDec {
    fn from(value: String) -> Self {
        PrecDec { i: value }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DepositOption {
    pub disable_swap: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MultiHopRoute {
    pub hops: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Params {
    pub fee_tiers: Vec<u64>,
    pub paused: bool,
    pub max_jits_per_block: u64,
    pub good_til_purge_allowance: u64,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Pool {
    #[serde(default)]
    pub id: u64,
    pub lower_tick0: Option<PoolReserves>,
    pub upper_tick1: Option<PoolReserves>,
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
    pub total_shares: Option<Int128>,
    pub pool: Option<Pool>,
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
    pub price_taker_to_maker: PrecDec,
    pub price_opposite_taker_to_maker: PrecDec,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PoolReservesKey {
    pub trade_pair_id: TradePairID,
    pub tick_index_taker_to_maker: i64,
    pub fee: Option<u64>,
}
