use cosmos_sdk_proto::cosmos::base::abci::v1beta1::{MsgData, TxMsgData};
use cosmwasm_std::{Binary, Deps, Reply, StdError, StdResult};
use prost::{DecodeError, Message};

/// Parse acknowledgement into Vec<MsgData> structure
pub fn parse_response(data: Binary) -> StdResult<Vec<MsgData>> {
    let result = Binary::from_base64(&data.to_string())?;
    let msg_data: Result<TxMsgData, DecodeError> = TxMsgData::decode(result.as_slice());
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

/// Parse protobuff any item into T structure
pub fn parse_item<T: prost::Message + Default>(item: &Vec<u8>) -> StdResult<T> {
    let res = T::decode(item.as_slice());
    match res {
        Err(e) => return Err(StdError::generic_err(format!("Can't decode item: {}", e))),
        Ok(data) => Ok(data),
    }
}

/// Parse sequence number from reply
pub fn parse_sequence(deps: Deps, msg: Reply) -> StdResult<(String, u64)> {
    let mut may_seq_id: Option<u64> = None;
    let mut may_channel_id: Option<String> = None;
    for attr in msg
        .result
        .into_result()
        .map_err(StdError::generic_err)?
        .events
        .iter()
        .find(|e| e.ty == "send_packet")
        .ok_or_else(|| StdError::generic_err("failed to find packet_sequence attribute"))?
        .attributes
        .iter()
    {
        if attr.key == "packet_sequence" {
            may_seq_id = Some(
                str::parse(&attr.value).map_err(|_e| StdError::generic_err("parse int error"))?,
            );
        }
        if attr.key == "packet_src_channel" {
            may_channel_id = Some(attr.value.clone())
        }
        if let (Some(seq_id), Some(channel_id)) = (may_seq_id, &may_channel_id) {
            deps.api.debug(
                format!(
                    "WASMDEBUG: parse_sequence: reply result: {:?} {:?}",
                    channel_id, seq_id
                )
                .as_str(),
            );
            return Ok((channel_id.clone(), seq_id));
        }
    }

    Err(StdError::generic_err(format!(
        "failed to find channel_id or seq_id: {:?} {:?}",
        may_channel_id, may_seq_id
    )))
}

const CONTROLLER_PORT_PREFIX: &str = "icacontroller-";
const ICA_OWNER_DELIMITER: &str = ".";

/// Constructs a full ICA controller port identifier for a contract with **contract_address** and **interchain_account_id**
/// https://github.com/cosmos/ibc-go/blob/46e020640e66f9043c14c53a4d215a5b457d6703/modules/apps/27-interchain-accounts/types/port.go#L11
pub fn get_port_id(contract_address: String, interchain_account_id: &str) -> String {
    CONTROLLER_PORT_PREFIX.to_string()
        + &contract_address
        + ICA_OWNER_DELIMITER
        + interchain_account_id
}
