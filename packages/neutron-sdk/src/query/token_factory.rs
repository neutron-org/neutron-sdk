use crate::{bindings::query::NeutronQuery, NeutronResult};
use cosmwasm_std::Deps;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FullDenomResponse {
    pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DenomAdminResponse {
    pub admin: String,
}

pub fn query_full_denom(
    deps: Deps<NeutronQuery>,
    creator_addr: impl Into<String>,
    subdenom: impl Into<String>,
) -> NeutronResult<FullDenomResponse> {
    let query = NeutronQuery::FullDenom {
        creator_addr: creator_addr.into(),
        subdenom: subdenom.into(),
    };
    Ok(deps.querier.query(&query.into())?)
}

pub fn query_denom_admin(
    deps: Deps<NeutronQuery>,
    subdenom: impl Into<String>,
) -> NeutronResult<DenomAdminResponse> {
    let query = NeutronQuery::DenomAdmin {
        subdenom: subdenom.into(),
    };
    Ok(deps.querier.query(&query.into())?)
}
