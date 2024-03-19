use cosmos_sdk_proto::{cosmos::base::abci::v1beta1::TxMsgData, traits::Message};
use cosmwasm_std::{Binary, StdError, StdResult};
use prost_types::Any;

/// Decodes acknowledgement into `Vec<MsgData>` structure
pub fn decode_acknowledgement_response(data: Binary) -> StdResult<Vec<Any>> {
    let msg_data: Result<TxMsgData, _> = TxMsgData::decode(data.as_slice());
    match msg_data {
        Err(e) => Err(StdError::generic_err(format!(
            "Can't decode response: {}",
            e
        ))),
        Ok(msg) => Ok(msg.msg_responses),
    }
}
