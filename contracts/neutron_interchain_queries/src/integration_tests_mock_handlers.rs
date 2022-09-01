use crate::state::{IntegrationTestsKvMock, INTEGRATION_TESTS_KV_MOCK};
use cosmwasm_std::{DepsMut, Response};
use neutron_sdk::bindings::msg::NeutronMsg;
use neutron_sdk::bindings::query::InterchainQueries;
use neutron_sdk::NeutronResult;

pub fn set_kv_query_mock(deps: DepsMut<InterchainQueries>) -> NeutronResult<Response<NeutronMsg>> {
    INTEGRATION_TESTS_KV_MOCK.save(deps.storage, &IntegrationTestsKvMock::Enabled)?;
    Ok(Response::default())
}

pub fn unset_kv_query_mock(
    deps: DepsMut<InterchainQueries>,
) -> NeutronResult<Response<NeutronMsg>> {
    INTEGRATION_TESTS_KV_MOCK.save(deps.storage, &IntegrationTestsKvMock::Disabled)?;
    Ok(Response::default())
}
