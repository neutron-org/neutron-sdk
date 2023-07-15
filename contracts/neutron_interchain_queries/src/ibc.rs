use crate::state::save_reply_payload;
use crate::state::SudoPayload;
use crate::state::INTERCHAIN_ACCOUNTS;
use crate::reply::SUDO_PAYLOAD_REPLY_ID;
use cosmwasm_std::CosmosMsg;
use cosmwasm_std::DepsMut;
use cosmwasm_std::Response;
use cosmwasm_std::StdResult;
use cosmwasm_std::SubMsg;
use neutron_sdk::bindings::msg::IbcFee;
use neutron_sdk::bindings::msg::NeutronMsg;
use neutron_sdk::interchain_txs::helpers::get_port_id;

use neutron_sdk::bindings::query::NeutronQuery;

use cosmwasm_std::Env;

use neutron_sdk::NeutronResult;

const FEE_DENOM: &str = "untrn";

pub fn execute_register_ica(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    connection_id: String,
    interchain_account_id: String,
) -> NeutronResult<Response<NeutronMsg>> {
    let register =
        NeutronMsg::register_interchain_account(connection_id, interchain_account_id.clone());
    let key = get_port_id(env.contract.address.as_str(), &interchain_account_id);
    // we are saving empty data here because we handle response of registering ICA in sudo_open_ack method
    INTERCHAIN_ACCOUNTS.save(deps.storage, key, &None)?;
    Ok(Response::new().add_message(register))
}

pub fn min_ntrn_ibc_fee(fee: IbcFee) -> IbcFee {
    IbcFee {
        recv_fee: fee.recv_fee,
        ack_fee: fee
            .ack_fee
            .into_iter()
            .filter(|a| a.denom == FEE_DENOM)
            .collect(),
        timeout_fee: fee
            .timeout_fee
            .into_iter()
            .filter(|a| a.denom == FEE_DENOM)
            .collect(),
    }
}

// saves payload to process later to the storage and returns a SubmitTX Cosmos SubMsg with necessary reply id
pub fn msg_with_sudo_callback<C: Into<CosmosMsg<T>>, T>(
    deps: DepsMut<NeutronQuery>,
    msg: C,
    payload: SudoPayload,
) -> StdResult<SubMsg<T>> {
    save_reply_payload(deps.storage, payload)?;
    Ok(SubMsg::reply_on_success(msg, SUDO_PAYLOAD_REPLY_ID))
}
