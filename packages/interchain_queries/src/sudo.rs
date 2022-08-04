use crate::error::{ContractError, ContractResult};
use crate::queries::get_registered_query;
use crate::types::COSMOS_SDK_TRANSFER_MSG_URL;
use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{TxBody, TxRaw};
use cosmwasm_std::{Binary, DepsMut, Env, Response, StdError};
use neutron_bindings::query::InterchainQueries;
use prost::Message as ProstMessage;
use serde::{Deserialize, Serialize};
use serde_json_wasm;

/// TransferRecipientQuery is used to parse the query_data field of a QUERY_TRANSFERS query.
#[derive(Serialize, Deserialize)]
struct TransferRecipientQuery {
    #[serde(rename = "transfer.recipient")]
    recipient: String,
}

/// sudo_check_tx_query_result is an example callback that checks if a given transaction
/// satisfies the registered transaction query. Here, we check that the provided transaction
/// contains a Send message from a specific address.
pub fn sudo_tx_query_result(
    deps: DepsMut<InterchainQueries>,
    _env: Env,
    query_id: u64,
    _height: u64,
    data: Binary,
) -> ContractResult<Response> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_check_tx_query_result received; query_id: {:?}",
            query_id,
        )
        .as_str(),
    );

    // Decode the transaction data
    let tx: TxRaw = TxRaw::decode(data.as_slice())?;
    let body: TxBody = TxBody::decode(tx.body_bytes.as_slice())?;

    // Get the registered query by ID and retrieve the raw query string
    let registered_query = get_registered_query(deps.as_ref(), query_id)?;
    let transactions_filter = registered_query.registered_query.transactions_filter;

    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_check_tx_query_result loaded query string; query_id: {:?},\
             transactions_filter: {:?}",
            query_id, transactions_filter,
        )
        .as_str(),
    );

    #[allow(clippy::match_single_binding)]
    // Depending of the query type, check the transaction data to see whether is satisfies
    // the original query.
    match registered_query.registered_query.query_type.as_str() {
        // If you don't write specific checks for a transaction query type, all submitted results
        // will be treated as valid.
        // TODO: come up with solution to determine transactions filter type
        _ => {
            // For transfer queries, query data looks like "{"transfer.recipient": "some_address"}"
            let query_data: TransferRecipientQuery =
                serde_json_wasm::from_str(transactions_filter.as_str())?;

            deps.api.debug(
                format!(
                    "WASMDEBUG: sudo_check_tx_query_result parsed query string; query_id: {:?}",
                    query_id
                )
                .as_str(),
            );

            let mut matching_message_found = false;
            for message in body.messages {
                // Skip all messages in this transaction that are not Send messages.
                if message.type_url != *COSMOS_SDK_TRANSFER_MSG_URL.to_string() {
                    continue;
                }

                // Parse a Send message and check that it has the required recipient.
                let transfer_msg: MsgSend = MsgSend::decode(message.value.as_slice())?;

                if transfer_msg.to_address == query_data.recipient {
                    deps.api.debug(
                        format!(
                            "WASMDEBUG: sudo_check_tx_query_result found a matching transaction; \
                             query_id: {:?}, from_address: {:?}",
                            query_id, transfer_msg.from_address,
                        )
                        .as_str(),
                    );

                    matching_message_found = true;
                    break;
                }

                // Note: use `crate::types::{protobuf_coin_to_std_coin}` to cast proto
                // coins to sdk coins.
            }

            // If we didn't find a Send message with the correct recipient, return an error, and
            // this query result will be rejected by Neutron: no data will be saved to state.
            match matching_message_found {
                true => Ok(Response::new()),
                false => {
                    deps.api.debug(
                        format!(
                            "WASMDEBUG: sudo_check_tx_query_result failed to find a matching \
                             transaction; query_id: {:?}",
                            query_id
                        )
                        .as_str(),
                    );
                    Err(ContractError::Std(StdError::generic_err(
                        "failed to find a matching transaction message",
                    )))
                }
            }
        }
    }
}

/// sudo_check_kv_query_result is an example callback that processes a KV query result.
/// Note that only the query id is provided, so you need to read the query result from
/// state.
pub fn sudo_kv_query_result(
    deps: DepsMut<InterchainQueries>,
    _env: Env,
    query_id: u64,
) -> ContractResult<Response> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_kv_query_result received; query_id: {:?}",
            query_id,
        )
        .as_str(),
    );

    // TODO: provide an actual example. Currently to many things are going to change
    // after @pro0n00gler's PRs to implement this.

    Ok(Response::default())
}
