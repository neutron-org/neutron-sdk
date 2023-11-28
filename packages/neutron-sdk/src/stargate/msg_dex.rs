use crate::proto_types::neutron::dex::{
    DepositOptions as DepositOptionsGen, LimitOrderType, MsgCancelLimitOrder, MsgDeposit,
    MsgMultiHopSwap, MsgPlaceLimitOrder, MsgWithdrawFilledLimitOrder, MsgWithdrawal, MultiHopRoute,
};
use crate::stargate::aux::create_stargate_msg;
use cosmwasm_std::{CosmosMsg, Timestamp};
use prost_types::Timestamp as TimestampGen;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const DEPOSIT_MSG_PATH: &str = "/neutron.dex.MsgDeposit";
const WITHDRAWAL_MSG_PATH: &str = "/neutron.dex.MsgWithdrawal";
const PLACE_LIMIT_ORDER_MSG_PATH: &str = "/neutron.dex.MsgPlaceLimitOrder";
const WITHDRAW_FILLED_LIMIT_ORDER_MSG_PATH: &str = "/neutron.dex.MsgWithdrawFilledLimitOrder";
const CANCEL_LIMIT_ORDER_MSG_PATH: &str = "/neutron.dex.MsgCancelLimitOrder";
const MULTI_HOP_SWAP_MSG_PATH: &str = "/neutron.dex.MsgMultiHopSwap";

pub fn msg_deposit(
    sender: String,
    receiver: String,
    token_a: String,
    token_b: String,
    amounts_a: Vec<String>,
    amounts_b: Vec<String>,
    tick_indexes_a_to_b: Vec<i64>,
    fees: Vec<u64>,
    options: Vec<DepositOptions>,
) -> CosmosMsg {
    let msg = MsgDeposit {
        creator: sender,
        receiver,
        token_a,
        token_b,
        amounts_a,
        amounts_b,
        tick_indexes_a_to_b,
        fees,
        options: options.into_iter().map(|o| o.into()).collect(),
    };
    create_stargate_msg(msg, DEPOSIT_MSG_PATH)
}

pub fn msg_withdrawal(
    sender: String,
    receiver: String,
    token_a: String,
    token_b: String,
    shares_to_remove: Vec<String>,
    tick_indexes_a_to_b: Vec<i64>,
    fees: Vec<u64>,
) -> CosmosMsg {
    let msg = MsgWithdrawal {
        creator: sender,
        receiver,
        token_a,
        token_b,
        shares_to_remove,
        tick_indexes_a_to_b,
        fees,
    };
    create_stargate_msg(msg, WITHDRAWAL_MSG_PATH)
}

pub fn msg_place_limit_order(
    sender: String,
    receiver: String,
    token_in: String,
    token_out: String,
    tick_index_in_to_out: i64,
    amount_in: String,
    order_type: LimitOrderType,
    expiration_time: Option<Timestamp>,
    max_amount_out: Option<String>,
) -> CosmosMsg {
    let msg = MsgPlaceLimitOrder {
        creator: sender,
        receiver,
        token_in,
        token_out,
        tick_index_in_to_out,
        amount_in,
        order_type: i32::from(order_type),
        expiration_time: expiration_time.map(|e| convert_timestamp(e)),
        max_amount_out: max_amount_out.unwrap_or_default(),
    };
    create_stargate_msg(msg, PLACE_LIMIT_ORDER_MSG_PATH)
}

pub fn msg_withdraw_filled_limit_order(sender: String, tranche_key: String) -> CosmosMsg {
    let msg = MsgWithdrawFilledLimitOrder {
        creator: sender,
        tranche_key,
    };
    create_stargate_msg(msg, WITHDRAW_FILLED_LIMIT_ORDER_MSG_PATH)
}

pub fn msg_cancel_limit_order(sender: String, tranche_key: String) -> CosmosMsg {
    let msg = MsgCancelLimitOrder {
        creator: sender,
        tranche_key,
    };
    create_stargate_msg(msg, CANCEL_LIMIT_ORDER_MSG_PATH)
}

pub fn msg_multi_hop_swap(
    sender: String,
    receiver: String,
    routes: Vec<Vec<String>>,
    amount_in: String,
    exit_limit_price: String,
    pick_best_route: bool,
) -> CosmosMsg {
    let msg = MsgMultiHopSwap {
        creator: sender,
        receiver,
        routes: routes
            .into_iter()
            .map(|r| MultiHopRoute { hops: r })
            .collect(),
        amount_in,
        exit_limit_price,
        pick_best_route,
    };
    create_stargate_msg(msg, MULTI_HOP_SWAP_MSG_PATH)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DepositOptions {
    pub disable_autoswap: bool,
}

impl Into<DepositOptionsGen> for DepositOptions {
    fn into(self) -> DepositOptionsGen {
        DepositOptionsGen {
            disable_autoswap: self.disable_autoswap,
        }
    }
}

fn convert_timestamp(timestamp: Timestamp) -> TimestampGen {
    TimestampGen {
        seconds: i64::try_from(timestamp.seconds()).unwrap(),
        nanos: i32::try_from(timestamp.subsec_nanos()).unwrap(),
    }
}
