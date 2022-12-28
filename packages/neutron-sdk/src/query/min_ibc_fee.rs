use crate::{
    bindings::{msg::IbcFee, query::NeutronQuery},
    NeutronResult,
};
use cosmwasm_std::Deps;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MinIbcFeeResponse {
    pub min_fee: IbcFee,
}

pub fn query_min_ibc_fee(deps: Deps<NeutronQuery>) -> NeutronResult<MinIbcFeeResponse> {
    let query = NeutronQuery::MinIbcFee {};
    Ok(deps.querier.query(&query.into())?)
}
