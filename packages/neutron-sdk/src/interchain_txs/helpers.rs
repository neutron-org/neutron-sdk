use crate::bindings::msg::ChannelOrdering;
use crate::NeutronResult;
use cosmos_sdk_proto::traits::Message;
use cosmwasm_std::{Addr, CosmosMsg, StdError, StdResult};
use neutron_std::types::cosmos::base::v1beta1::Coin;
use neutron_std::types::neutron::interchaintxs::v1::MsgRegisterInterchainAccount;

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
) -> NeutronResult<CosmosMsg> {
    Ok(MsgRegisterInterchainAccount {
        from_address: contract.to_string(),
        connection_id,
        interchain_account_id,
        register_fee,
        ordering: ordering.unwrap_or(ChannelOrdering::OrderOrdered).into(),
    }
    .into())
}
