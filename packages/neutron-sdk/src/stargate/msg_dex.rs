use crate::proto_types::neutron::dex::{
    MsgCancelLimitOrder, MsgDeposit, MsgMultiHopSwap, MsgPlaceLimitOrder,
    MsgWithdrawFilledLimitOrder, MsgWithdrawal,
};
use crate::stargate::aux::create_stargate_msg;
use cosmwasm_std::CosmosMsg;

const DEPOSIT_MSG_PATH: &str = "/neutron.dex.Msg/Deposit";
const WITHDRAWAL_MSG_PATH: &str = "/neutron.dex.Msg/Withdrawal";
const PLACE_LIMIT_ORDER_MSG_PATH: &str = "/neutron.dex.Msg/PlaceLimitOrder";
const WITHDRAW_FILLED_LIMIT_ORDER_MSG_PATH: &str = "/neutron.dex.Msg/WithdrawFilledLimitOrder";
const CANCEL_LIMIT_ORDER_MSG_PATH: &str = "/neutron.dex.Msg/CancelLimitOrder";
const MULTI_HOP_SWAP_MSG_PATH: &str = "/neutron.dex.Msg/MultiHopSwap";

pub fn msg_deposit(req: MsgDeposit) -> CosmosMsg {
    create_stargate_msg(req, DEPOSIT_MSG_PATH)
}

pub fn msg_withdrawal(req: MsgWithdrawal) -> CosmosMsg {
    create_stargate_msg(req, WITHDRAWAL_MSG_PATH)
}

pub fn msg_place_limit_order(req: MsgPlaceLimitOrder) -> CosmosMsg {
    create_stargate_msg(req, PLACE_LIMIT_ORDER_MSG_PATH)
}

pub fn msg_withdraw_filled_limit_order(req: MsgWithdrawFilledLimitOrder) -> CosmosMsg {
    create_stargate_msg(req, WITHDRAW_FILLED_LIMIT_ORDER_MSG_PATH)
}

pub fn msg_cancel_limit_order(req: MsgCancelLimitOrder) -> CosmosMsg {
    create_stargate_msg(req, CANCEL_LIMIT_ORDER_MSG_PATH)
}

pub fn msg_multi_hop_swap(req: MsgMultiHopSwap) -> CosmosMsg {
    create_stargate_msg(req, MULTI_HOP_SWAP_MSG_PATH)
}
