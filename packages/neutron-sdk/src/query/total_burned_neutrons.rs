use crate::{bindings::query::InterchainQueries, NeutronResult};
use cosmwasm_std::{Coin, Deps};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TotalBurnedNeutronsAmountResponse {
    pub coins: Vec<Coin>,
}

/// Returns total amount of burned neutron fees
pub fn query_total_burned_neutrons(
    deps: Deps<InterchainQueries>,
) -> NeutronResult<TotalBurnedNeutronsAmountResponse> {
    let query = InterchainQueries::TotalBurnedNeutronsAmount {};

    let res: TotalBurnedNeutronsAmountResponse = deps.querier.query(&query.into())?;
    Ok(res)
}
