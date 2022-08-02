use crate::error::{ContractError, ContractResult};
use crate::types::TmpRegisteredQuery;
use cosmwasm_std::{Deps, StdError};
use cw_storage_plus::{Item, Map};
use neutron_bindings::query::InterchainQueries;
use schemars::_serde_json::to_string;
use serde::Serialize;

/// Storage for temp requests used in reply logic
pub const TMP_REGISTER_INTERCHAIN_QUERY_REQUEST: Item<TmpRegisteredQuery> =
    Item::new("tmp_register_interchain_query_request");

/// Storage to store registered query ids where key are (zone_id, query_type, query_data) and value is registered_query_id
pub const REGISTERED_INTERCHAIN_QUERIES: Map<(&str, &str, &str), u64> =
    Map::new("registered_interchain_queries");

/// Returns registered query id by zone_id, query_type and query_data
pub fn get_registered_query_id<T>(
    deps: Deps<InterchainQueries>,
    zone_id: &str,
    query_type: &str,
    query_data: &T,
) -> ContractResult<u64>
where
    T: ?Sized + Serialize,
{
    let query_data_json_encoded =
        to_string(&query_data).map_err(|e| StdError::generic_err(e.to_string()))?;

    let registered_query_id = REGISTERED_INTERCHAIN_QUERIES.may_load(
        deps.storage,
        (zone_id, query_type, query_data_json_encoded.as_str()),
    )?;

    if registered_query_id.is_none() {
        return Err(ContractError::InterchainQueryIsNotRegistered {
            zone_id: zone_id.to_string(),
            query_type: query_type.to_string(),
            query_data_json_encoded,
        });
    }
    #[allow(clippy::unwrap_used)]
    Ok(registered_query_id.unwrap())
}
