use cosmwasm_std::{
    from_json, to_json_vec, to_vec, Binary, ContractResult, CosmosMsg, Deps, Empty, QueryRequest,
    StdError, StdResult, SystemResult,
};
use prost_types::Timestamp as TimestampGen;
use serde::de::DeserializeOwned;

pub(crate) fn make_stargate_query<Req, Res>(deps: Deps, req: Req, path: &str) -> StdResult<Res>
where
    Req: prost::Message,
    Res: DeserializeOwned,
{
    let raw = to_vec::<QueryRequest<Empty>>(&QueryRequest::Stargate {
        path: path.to_string(),
        data: req.encode_to_vec().into(),
    })
    .map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    })?;
    match deps.querier.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier system error: {}",
            system_err
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            contract_err
        ))),
        // response(value) is base64 encoded bytes
        SystemResult::Ok(ContractResult::Ok(value)) => {
            let str = value.to_base64();
            deps.api.debug(
                format!(
                    "WASMDEBUG: raw make_stargate_query resp: {:?}",
                    value.to_string()
                )
                .as_str(),
            );
            deps.api
                .debug(format!("WASMDEBUG: make_stargate_query resp: {:?}", str).as_str());
            return from_json(value);
        }
    }
}

pub(crate) fn create_stargate_msg<Req: prost::Message>(req: Req, path: &str) -> CosmosMsg {
    cosmwasm_std::CosmosMsg::Stargate {
        type_url: path.to_string(),
        value: Binary::from(req.encode_to_vec()),
    }
}

pub(crate) fn convert_timestamp(timestamp: i64) -> TimestampGen {
    TimestampGen {
        seconds: timestamp,
        nanos: 0,
    }
}
