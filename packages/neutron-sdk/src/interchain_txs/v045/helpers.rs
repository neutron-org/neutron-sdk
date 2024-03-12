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
