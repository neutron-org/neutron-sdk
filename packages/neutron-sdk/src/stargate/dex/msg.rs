use crate::proto_types::neutron::dex::{
    MsgCancelLimitOrder, MsgDeposit, MsgMultiHopSwap, MsgPlaceLimitOrder,
    MsgWithdrawFilledLimitOrder, MsgWithdrawal,
};
use crate::stargate::aux::create_stargate_msg;
use crate::stargate::dex::types::{
    CancelLimitOrderRequest, DepositRequest, MultiHopSwapRequest, PlaceLimitOrderRequest,
    WithdrawFilledLimitOrderRequest, WithdrawalRequest,
};
use cosmwasm_std::CosmosMsg;

const DEPOSIT_MSG_PATH: &str = "/neutron.dex.MsgDeposit";
const WITHDRAWAL_MSG_PATH: &str = "/neutron.dex.MsgWithdrawal";
const PLACE_LIMIT_ORDER_MSG_PATH: &str = "/neutron.dex.MsgPlaceLimitOrder";
const WITHDRAW_FILLED_LIMIT_ORDER_MSG_PATH: &str = "/neutron.dex.MsgWithdrawFilledLimitOrder";
const CANCEL_LIMIT_ORDER_MSG_PATH: &str = "/neutron.dex.MsgCancelLimitOrder";
const MULTI_HOP_SWAP_MSG_PATH: &str = "/neutron.dex.MsgMultiHopSwap";

pub fn msg_deposit(req: DepositRequest) -> CosmosMsg {
    create_stargate_msg(MsgDeposit::from(req), DEPOSIT_MSG_PATH)
}

pub fn msg_withdrawal(req: WithdrawalRequest) -> CosmosMsg {
    create_stargate_msg(MsgWithdrawal::from(req), WITHDRAWAL_MSG_PATH)
}

pub fn msg_place_limit_order(req: PlaceLimitOrderRequest) -> CosmosMsg {
    create_stargate_msg(MsgPlaceLimitOrder::from(req), PLACE_LIMIT_ORDER_MSG_PATH)
}

pub fn msg_withdraw_filled_limit_order(req: WithdrawFilledLimitOrderRequest) -> CosmosMsg {
    create_stargate_msg(
        MsgWithdrawFilledLimitOrder::from(req),
        WITHDRAW_FILLED_LIMIT_ORDER_MSG_PATH,
    )
}

pub fn msg_cancel_limit_order(req: CancelLimitOrderRequest) -> CosmosMsg {
    create_stargate_msg(MsgCancelLimitOrder::from(req), CANCEL_LIMIT_ORDER_MSG_PATH)
}

pub fn msg_multi_hop_swap(req: MultiHopSwapRequest) -> CosmosMsg {
    create_stargate_msg(MsgMultiHopSwap::from(req), MULTI_HOP_SWAP_MSG_PATH)
}
