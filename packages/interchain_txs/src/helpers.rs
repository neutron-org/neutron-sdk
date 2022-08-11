use cosmos_sdk_proto::cosmos::base::abci::v1beta1::{MsgData, TxMsgData};
use cosmwasm_std::{Binary, Deps, Reply, StdError, StdResult};
use prost::{DecodeError, Message};

use std::io::Cursor;

pub fn parse_response(data: Binary) -> StdResult<Vec<MsgData>> {
    let result = Binary::from_base64(&data.to_string())?;
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

pub fn parse_sequence(deps: Deps, msg: Reply) -> StdResult<u64> {
    let seq_id = str::parse(
        &msg.result
            .into_result()
            .map_err(StdError::generic_err)?
            .events
            .iter()
            .find(|e| e.ty == "send_packet")
            .and_then(|e| e.attributes.iter().find(|a| a.key == "packet_sequence"))
            .ok_or_else(|| StdError::generic_err("failed to find packet_sequence atribute"))?
            .value
            .clone(),
    )
    .map_err(|_e| StdError::generic_err("parse int error"))?;
    deps.api
        .debug(format!("WASMDEBUG: parse_sequence: reply result: {:?}", seq_id).as_str());
    Ok(seq_id)
}
