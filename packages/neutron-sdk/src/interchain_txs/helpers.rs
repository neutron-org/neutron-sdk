use crate::bindings::msg::ChannelOrdering;
use cosmos_sdk_proto::traits::Message;
use cosmwasm_std::{Addr, CosmosMsg, StdError, StdResult};
use neutron_std::shim::Any;
use neutron_std::types::cosmos::base::v1beta1::Coin;
use neutron_std::types::neutron::feerefunder::Fee;
use neutron_std::types::neutron::interchaintxs::v1::{MsgRegisterInterchainAccount, MsgSubmitTx};

/// Decodes protobuf any item into T structure
pub fn decode_message_response<T: Message + Default>(item: &Vec<u8>) -> StdResult<T> {
    let res = T::decode(item.as_slice());
    match res {
        Err(e) => Err(StdError::generic_err(format!("Can't decode item: {}", e))),
        Ok(data) => Ok(data),
    }
}

const CONTROLLER_PORT_PREFIX: &str = "icacontroller-";
const ICA_OWNER_DELIMITER: &str = ".";

/// Constructs a full ICA controller port identifier for a contract with **contract_address** and **interchain_account_id**
/// <https://github.com/cosmos/ibc-go/blob/46e020640e66f9043c14c53a4d215a5b457d6703/modules/apps/27-interchain-accounts/types/port.go#L11>
pub fn get_port_id<R: AsRef<str>>(contract_address: R, interchain_account_id: R) -> String {
    CONTROLLER_PORT_PREFIX.to_string()
        + contract_address.as_ref()
        + ICA_OWNER_DELIMITER
        + interchain_account_id.as_ref()
}

pub fn register_interchain_account(
    contract: Addr,
    connection_id: String,
    interchain_account_id: String,
    register_fee: Vec<Coin>,
    ordering: Option<ChannelOrdering>,
) -> CosmosMsg {
    MsgRegisterInterchainAccount {
        from_address: contract.to_string(),
        connection_id,
        interchain_account_id,
        register_fee,
        ordering: ordering.unwrap_or(ChannelOrdering::OrderOrdered).into(),
    }
    .into()
}

/// Basic helper to define a submit tx message:
/// * **contract** is a contract that is sending the message
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **interchain_account_id** is an identifier of your interchain account from which you want to execute msgs;
/// * **msgs** is a list of protobuf encoded Cosmos-SDK messages you want to execute on remote chain;
/// * **memo** is a memo you want to attach to your interchain transaction. It behaves like a memo in usual Cosmos transaction;
/// * **timeout** is a timeout in seconds after which the packet times out.
/// * **fee** is a fee that is used for different kinds of callbacks. Unused fee types will be returned to msg sender.
pub fn submit_tx(
    contract: Addr,
    connection_id: String,
    interchain_account_id: String,
    msgs: Vec<Any>,
    memo: String,
    timeout: u64,
    fee: Fee,
) -> CosmosMsg {
    MsgSubmitTx {
        from_address: contract.to_string(),
        connection_id,
        interchain_account_id,
        msgs,
        memo,
        timeout,
        fee: Some(fee),
    }.into()
}
