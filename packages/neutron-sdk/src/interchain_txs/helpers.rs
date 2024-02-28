use cosmos_sdk_proto::{
    cosmos::base::abci::v1beta1::{MsgData, TxMsgData},
    traits::Message,
};
use cosmwasm_std::{Binary, StdError, StdResult};

/// Decodes acknowledgement into `Vec<MsgData>` structure
/// We consider this method as deprecated. Use v047 instead.
pub fn decode_acknowledgement_response(data: Binary) -> StdResult<Vec<MsgData>> {
    let tx_msg_data: Result<TxMsgData, _> = TxMsgData::decode(data.as_slice());

    match tx_msg_data {
        Err(e) => Err(StdError::generic_err(format!(
            "Can't decode response: {}",
            e
        ))),
        Ok(msg) => {
            if !msg.msg_responses.is_empty() {
                msg.msg_responses
                    .into_iter()
                    .map(|any_msg| {
                        Ok(MsgData {
                            msg_type: any_msg.type_url,
                            data: any_msg.value,
                        })
                    })
                    .collect::<StdResult<Vec<MsgData>>>()
            } else {
                // Field `.data` is deprecated since cosmos-sdk v047.
                // But for backwards compatibility we still allow that. Given function can be used w both v045 & v047
                #[allow(deprecated)]
                Ok(msg.data)
            }
        }
    }
}

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
