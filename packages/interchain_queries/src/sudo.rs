use crate::error::{ContractError, ContractResult};
use crate::queries::get_registered_query;
use crate::types::{COSMOS_SDK_TRANSFER_MSG_URL, QUERY_TRANSFERS};
use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{TxBody, TxRaw};
use cosmwasm_std::{DepsMut, Env, Response, StdError};
use prost::Message as ProstMessage;
use schemars::_serde_json;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

/// TransferRecipientQuery is used to parse the query_data field of a QUERY_TRANSFERS query.
#[derive(Serialize, Deserialize)]
struct TransferRecipientQuery {
    #[serde(rename = "name")]
    recipient: String,
}

/// sudo_check_tx_query_result is an example callback that checks if a given transaction
/// satisfies the registered transaction query. Here, we check that the provided transaction
/// contains a Send message from a specific address.
pub fn sudo_check_tx_query_result(
    deps: DepsMut,
    _env: Env,
    query_id: u64,
    _height: u64,
    data: Vec<u8>,
) -> ContractResult<Response> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_check_tx_query_result received: {:?} {:?}",
            query_id, data
        )
        .as_str(),
    );

    // Decode the transaction data
    let tx: TxRaw = TxRaw::decode(Cursor::new(data))?;
    let body: TxBody = TxBody::decode(Cursor::new(tx.body_bytes))?;

    // Get the registered query by ID and retrieve the raw query string
    let registered_query = get_registered_query(deps.as_ref(), query_id)?;
    let query_string = registered_query.registered_query.query_data.clone();

    // Depending of the query type, check the transaction data to see whether is satisfies
    // the original query.
    match registered_query.registered_query.query_type.as_str() {
        QUERY_TRANSFERS => {
            // For transfer queries, query data looks like "{"transfer.recipient": "some_address"}"
            let query_data: TransferRecipientQuery = _serde_json::from_str(query_string.as_str())?;

            let mut matching_message_found = false;
            for message in body.messages {
                // Skip all messages in this transaction that are not Send messages.
                if message.type_url != *COSMOS_SDK_TRANSFER_MSG_URL.to_string() {
                    continue;
                }

                // Parse a Send message and check that it has the required recipient.
                let transfer_msg: MsgSend = MsgSend::decode(Cursor::new(message.value))?;
                if transfer_msg.to_address == query_data.recipient {
                    matching_message_found = true;
                    break;
                }
            }

            // If we didn't find a Send message with the correct recipient, return an error, and
            // this query result will be rejected by Neutron: no data will be saved to state.
            match matching_message_found {
                true => Ok(Response::new()),
                false => Err(ContractError::Std(StdError::generic_err(
                    "matching messages not found in transaction",
                ))),
            }
        }

        // If you don't write specific checks for a transaction query type, all submitted results
        // will be treated as valid and saved to state.
        _ => Ok(Response::new()),
    }
}
