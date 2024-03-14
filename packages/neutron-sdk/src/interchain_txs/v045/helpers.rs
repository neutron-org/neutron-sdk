use cosmos_sdk_proto::{
    cosmos::base::abci::v1beta1::{MsgData, TxMsgData},
    traits::Message,
};
use cosmwasm_std::{Binary, StdError, StdResult};

/// Decodes acknowledgement into `Vec<MsgData>` structure
pub fn decode_acknowledgement_response(data: Binary) -> StdResult<Vec<MsgData>> {
    let msg_data: Result<TxMsgData, _> = TxMsgData::decode(data.as_slice());
    match msg_data {
        Err(e) => Err(StdError::generic_err(format!(
            "Can't decode response: {}",
            e
        ))),
        #[allow(deprecated)]
        Ok(msg) => Ok(msg.data),
    }
}
