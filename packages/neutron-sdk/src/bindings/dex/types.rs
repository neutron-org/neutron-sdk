use cosmwasm_std::Int128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LimitOrderType {
    #[default]
    GoodTilCancelled,
    FillOrKill,
    ImmediateOrCancel,
    JustInTime,
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
    pub max_true_taker_spread: Option<PrecDec>
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
