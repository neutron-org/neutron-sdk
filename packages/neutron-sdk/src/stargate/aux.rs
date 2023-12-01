use cosmwasm_std::{Binary, CosmosMsg, Deps, QueryRequest, StdResult};
use prost_types::Timestamp as TimestampGen;
use serde::de::DeserializeOwned;

pub(crate) fn make_stargate_query<Req, Res>(deps: Deps, req: Req, path: &str) -> StdResult<Res>
where
    Req: prost::Message,
    Res: DeserializeOwned,
{
    deps.querier.query(&QueryRequest::Stargate {
        path: path.to_string(),
        data: req.encode_to_vec().into(),
    })
}

pub(crate) fn create_stargate_msg<Req: prost::Message>(req: Req, path: &str) -> CosmosMsg {
    cosmwasm_std::CosmosMsg::Stargate {
        type_url: path.to_string(),
        value: Binary::from(req.encode_to_vec()),
    }
}

pub(crate) fn convert_timestamp(timestamp: u64) -> TimestampGen {
    TimestampGen {
        seconds: timestamp as i64,
        nanos: 0,
    }
}
