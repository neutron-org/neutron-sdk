use crate::{
    bindings::{msg::IbcFee, query::InterchainQueries},
    NeutronResult,
};
use cosmwasm_std::Deps;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MinimumIbcFeeResponse {
    pub min_fee: IbcFee,
}

pub fn query_minimum_ibc_fee(
    deps: Deps<InterchainQueries>,
) -> NeutronResult<MinimumIbcFeeResponse> {
    let query = InterchainQueries::MinimumIbcFee {};
    Ok(deps.querier.query(&query.into())?)
}
