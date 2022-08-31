use crate::state::{IntegrationTestsKvMock, INTEGRATION_TESTS_KV_MOCK};
use cosmwasm_std::{DepsMut, Response};
use interchain_queries::error::ContractResult;
use neutron_bindings::{msg::NeutronMsg, query::InterchainQueries};

pub fn set_kv_query_mock(deps: DepsMut<InterchainQueries>) -> ContractResult<Response<NeutronMsg>> {
    INTEGRATION_TESTS_KV_MOCK.save(deps.storage, &IntegrationTestsKvMock::Enabled)?;
    Ok(Response::default())
}

pub fn unset_kv_query_mock(
    deps: DepsMut<InterchainQueries>,
) -> ContractResult<Response<NeutronMsg>> {
    INTEGRATION_TESTS_KV_MOCK.save(deps.storage, &IntegrationTestsKvMock::Disabled)?;
    Ok(Response::default())
}
