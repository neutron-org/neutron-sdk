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
        /// The account to which PoolShares will be issued
        receiver: String,
        /// Denom for one side of the deposit
        token_a: String,
        /// Denom for the opposing side of the deposit
        token_b: String,
        /// Amounts of tokenA to deposit
        amounts_a: Vec<Uint128>,
        /// Amounts of tokenB to deposit
        amounts_b: Vec<Uint128>,
        /// Tick indexes to deposit at defined in terms of TokenA to TokenB (ie. TokenA is on the left)
        tick_indexes_a_to_b: Vec<i64>,
        /// Fees to use for each deposit
        fees: Vec<u64>,
        /// Additional deposit options
        options: Vec<DepositOption>,
    },
    /// Withdraw is used to redeem PoolShares for the user’s pro-rata
    /// portion of tokens within a liquidity pool. Users can withdraw from a pool at any time
    Withdrawal {
        /// The account to which the tokens are credited
        receiver: String,
        /// Denom for one side of the deposit
        token_a: String,
        /// Denom for the opposing side of the deposit
        token_b: String,
        /// Amount of shares to remove from each pool
        shares_to_remove: Vec<Uint128>,
        /// Tick indexes of the target LiquidityPools defined in terms of TokenA to TokenB
        /// (ie. TokenA is on the left)
        tick_indexes_a_to_b: Vec<i64>,
        /// Fee for the target LiquidityPools
        fees: Vec<u64>,
    },
    /// PlaceLimitOrder provides the primary mechanism for trading on the Duality Dex. Limit
    /// orders can provide liquidity to the Dex (“Maker Limit Orders”) and/or can be used to
    /// trade against preexisting liquidity (“Taker Limit Orders”)
    PlaceLimitOrder {
        /// Account to which TokenOut is credited or that will be allowed to
        /// withdraw or cancel a maker order
        receiver: String,
        /// Token being “sold”
        token_in: String,
        /// Token being “bought”
        token_out: String,
        /// Limit tick for a limit order, specified in terms of TokenIn to TokenOut
        tick_index_in_to_out: i64,
        /// Amount of TokenIn to be traded
        amount_in: Uint128,
        /// Type of limit order to be used. Must be one of:
        /// GOOD_TIL_CANCELLED, FILL_OR_KILL, IMMEDIATE_OR_CANCEL, JUST_IN_TIME, or GOOD_TIL_TIME
        order_type: LimitOrderType,
        // expirationTime is only valid if orderType == GOOD_TIL_TIME.
        /// Expiration time for order. Only valid for GOOD_TIL_TIME limit orders
        expiration_time: Option<u64>,
        /// Maximum amount of TokenB can be bought. For everything except JUST_IN_TIME OrderType
        max_amount_out: Option<Uint128>,
        /// Accepts standard decimals and decimals with scientific notation (ie. 1234.23E-7)
        limit_sell_price: String,
    },
    /// WithdrawFilledLimitOrder. Once a limit order has been filled – either partially or in
    /// its entirety, it can be withdrawn at any time. Withdrawing from a limit order credits
    /// all available proceeds to the user. Withdraw can be called on a limit order multiple
    /// times as new proceeds become available
    WithdrawFilledLimitOrder {
        /// TrancheKey for the target limit order
        tranche_key: String,
    },
    /// CancelLimitOrder. Standard Taker limit orders (Good-til-cancelled & Good-til-Time)
    /// can be canceled at any time if they have not been completely filled
    CancelLimitOrder {
        /// TrancheKey for the target limit order
        tranche_key: String,
    },
    /// MultiHopSwap provides a swapping mechanism to achieve better prices by routing
    /// through a series of pools
    MultiHopSwap {
        /// Account to which TokenOut is credited
        receiver: String,
        /// Array of possible routes
        routes: Vec<MultiHopRoute>,
        /// Amount of TokenIn to swap
        amount_in: Uint128,
        /// Minimum price that that must be satisfied for a route to succeed
        exit_limit_price: PrecDec,
        /// If true all routes are run and the route with the best price is used
        pick_best_route: bool,
    },
}
