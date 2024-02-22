use crate::bindings::dex::types::LimitOrderType;
use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::types::{DepositOption, MultiHopRoute, PrecDec};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DexMsg {
    /// Deposit provides liquidity to a specific trading pair by depositing tokens
    /// at a specific price into one or both sides of the pair in “a liquidity pool”
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
    /// Withdraw is used to redeem PoolShares for the user’s pro-rata
    /// portion of tokens within a liquidity pool. Users can withdraw from a pool at any time
    Withdrawal {
        receiver: String,
        token_a: String,
        token_b: String,
        shares_to_remove: Vec<Uint128>,
        tick_indexes_a_to_b: Vec<i64>,
        fees: Vec<u64>,
    },
    /// PlaceLimitOrder provides the primary mechanism for trading on the Duality Dex. Limit
    /// orders can provide liquidity to the Dex (“Maker Limit Orders”) and/or can be used to
    /// trade against preexisting liquidity (“Taker Limit Orders”)
    PlaceLimitOrder {
        receiver: String,
        token_in: String,
        token_out: String,
        tick_index_in_to_out: i64,
        amount_in: Uint128,
        order_type: LimitOrderType,
        // expirationTime is only valid if orderType == GOOD_TIL_TIME.
        expiration_time: Option<u64>,
        max_amount_out: Option<Uint128>,
    },
    /// WithdrawFilledLimitOrder. Once a limit order has been filled – either partially or in
    /// its entirety, it can be withdrawn at any time. Withdrawing from a limit order credits
    /// all available proceeds to the user. Withdraw can be called on a limit order multiple
    /// times as new proceeds become available
    WithdrawFilledLimitOrder {
        tranche_key: String,
    },
    /// CancelLimitOrder. Standard Taker limit orders (Good-til-cancelled & Good-til-Time)
    /// can be canceled at any time if they have not been completely filled
    CancelLimitOrder {
        tranche_key: String,
    },
    /// MultiHopSwap provides a swapping mechanism to achieve better prices by routing
    /// through a series of pools
    MultiHopSwap {
        receiver: String,
        routes: Vec<MultiHopRoute>,
        amount_in: Uint128,
        exit_limit_price: PrecDec,
        pick_best_route: bool,
    },
}
