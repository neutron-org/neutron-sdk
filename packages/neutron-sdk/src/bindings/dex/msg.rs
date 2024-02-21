use crate::bindings::dex::types::LimitOrderType;
use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::types::{DepositOption, MultiHopRoute, PrecDec};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DexMsg {
    Deposit {
        receiver: String,
        token_a: String,
        token_b: String,
        amounts_a: Vec<Uint128>,
        amounts_b: Vec<Uint128>,
        tick_indexes_a_to_b: Vec<i64>,
        fees: Vec<u64>,
        options: Vec<DepositOption>,
    },
    Withdrawal {
        receiver: String,
        token_a: String,
        token_b: String,
        shares_to_remove: Vec<Uint128>,
        tick_indexes_a_to_b: Vec<i64>,
        fees: Vec<u64>,
    },
    PlaceLimitOrder {
        receiver: String,
        token_in: String,
        token_out: String,
        tick_index_in_to_out: i64,
        amount_in: Uint128,
        order_type: LimitOrderType,
        // TODO: fix time representation
        // expirationTime is only valid iff orderType == GOOD_TIL_TIME.
        expiration_time: Option<u64>,
        max_amount_out: Option<Uint128>,
    },
    WithdrawFilledLimitOrder {
        tranche_key: String,
    },
    CancelLimitOrder {
        tranche_key: String,
    },
    MultiHopSwap {
        receiver: String,
        routes: Vec<MultiHopRoute>,
        amount_in: Uint128,
        exit_limit_price: PrecDec,
        pick_best_route: bool,
    },
}
