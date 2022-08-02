use crate::error::{ContractError, ContractResult};
use crate::storage::{REGISTERED_INTERCHAIN_QUERIES, TMP_REGISTER_INTERCHAIN_QUERY_REQUEST};
use cosmwasm_std::{DepsMut, Env, Reply, Response, SubMsgResult};
use neutron_bindings::query::InterchainQueries;
use protobuf::Message;
use stargate::interchain::interchainqueries_tx::MsgRegisterInterchainQueryResponse;

pub fn register_interchain_query_reply_handler(
    deps: DepsMut<InterchainQueries>,
    _env: Env,
    msg: Reply,
) -> ContractResult<Response> {
    let register_query_request = TMP_REGISTER_INTERCHAIN_QUERY_REQUEST.load(deps.storage)?;

    match msg.result {
        SubMsgResult::Ok(result) => {
            let result_data = match result.data {
                None => return Err(ContractError::EmptyInterchainQueryResult),
                Some(data) => data,
            };
            let register_interchain_query_response: MsgRegisterInterchainQueryResponse =
                Message::parse_from_bytes(result_data.as_slice())?;

            REGISTERED_INTERCHAIN_QUERIES.save(
                deps.storage,
                (
                    register_query_request.zone_id.as_str(),
                    register_query_request.query_type.as_str(),
                    register_query_request.query_data.as_str(),
                ),
                &register_interchain_query_response.id,
            )?;

            Ok(Response::new().add_attribute("action", "register"))
        }
        SubMsgResult::Err(err) => Err(ContractError::RegisterInterchainQueryFailed(err)),
    }
}
