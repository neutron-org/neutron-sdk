use cosmwasm_std::{Binary, CosmosMsg, Deps, QueryRequest, StdResult};
use prost_types::Timestamp as TimestampGen;
use serde::de::DeserializeOwned;

/// makes a stargate query by a given path and req and returns a response deserialised into a
/// given response model.
///
/// * **req** is a proto request model. Most likely it's a result of proto code generation;
/// * **path** is an RPC request path. Should be one of allowlisted stargate query paths;
///
/// Since stargate query results are JSON-encoded instead of protobuf-encoded, the Res is
/// expected to have a serde::de::DeserializeOwned trait. Why JSON, not proto? See the link:
/// <https://github.com/CosmWasm/wasmd/blob/6f6be7880f1caa666b86aaafea625208d70675dc/x/wasm/keeper/query_plugins.go#L360>
pub fn make_stargate_query<Req, Res>(deps: Deps, path: &str, req: Req) -> StdResult<Res>
where
    Req: prost::Message,
    Res: DeserializeOwned,
{
    #[allow(deprecated)]
    deps.querier.query(&QueryRequest::Stargate {
        path: path.to_string(),
        data: req.encode_to_vec().into(),
    })
}

/// creates a CosmosMsg::Stargate with given request payload and path.
///
/// * **req** is a proto request model. Most likely it's a result of proto code generation;
/// * **path** is an RPC request path. See Msg service definitions in neutron modules' proto files
/// for additional info.
pub fn create_stargate_msg<Req: prost::Message, T>(path: &str, req: Req) -> CosmosMsg<T> {
    #[allow(deprecated)]
    CosmosMsg::Stargate::<T> {
        type_url: path.to_string(),
        value: Binary::from(req.encode_to_vec()),
    }
}

/// creates a prost_types::Timestamp from a given unix timestamp value in seconds.
pub(crate) fn proto_timestamp_from_i64(timestamp: i64) -> TimestampGen {
    TimestampGen {
        seconds: timestamp,
        nanos: 0,
    }
}
