use cosmos_sdk_proto::cosmos::base::abci::v1beta1::{MsgData, TxMsgData};
use cosmwasm_std::{Binary, StdError, StdResult};
use prost::{DecodeError, Message};

use std::io::Cursor;

pub fn parse_response(data: String) -> StdResult<Vec<MsgData>> {
    let result = Binary::from_base64(&data)?;
    let msg_data: Result<TxMsgData, DecodeError> =
        TxMsgData::decode(Cursor::new(result.as_slice()));
    match msg_data {
        Err(e) => {
            return Err(StdError::generic_err(format!(
                "Can't decode response: {}",
                e
            )))
        }
        Ok(msg) => Ok(msg.data),
    }
}

pub fn parse_item<T: prost::Message + Default>(item: &Vec<u8>) -> StdResult<T> {
    let res = T::decode(Cursor::new(item));
    match res {
        Err(e) => return Err(StdError::generic_err(format!("Can't decode item: {}", e))),
        Ok(data) => Ok(data),
    }
}
