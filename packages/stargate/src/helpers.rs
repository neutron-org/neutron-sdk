use crate::interchain::interchainaccounts_tx::MsgSubmitTx;
use cosmwasm_std::{Binary, CosmosMsg, StdError, StdResult};
use protobuf::Message;

pub const SUBMIT_INTERCHAIN_TX_PATH: &str =
    "/neutron.interchainadapter.interchaintxs.v1.MsgSubmitTx";

/// Composes a protobuf encoded message (to be sent via stargate) for submitting an interchain transaction
pub fn make_stargate_tx(message: MsgSubmitTx) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Stargate {
        type_url: SUBMIT_INTERCHAIN_TX_PATH.to_string(),
        value: Binary::from(
            message
                .write_to_bytes()
                .map_err(|e| StdError::generic_err(e.to_string()))?,
        ),
    })
}
