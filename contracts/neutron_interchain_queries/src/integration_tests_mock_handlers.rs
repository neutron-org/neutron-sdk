use crate::state::{IntegrationTestsQueryMock, INTEGRATION_TESTS_QUERY_MOCK};
use cosmwasm_std::{DepsMut, Response};
use neutron_sdk::bindings::msg::NeutronMsg;
use neutron_sdk::bindings::query::InterchainQueries;
use neutron_sdk::NeutronResult;

pub fn set_query_mock(deps: DepsMut<InterchainQueries>) -> NeutronResult<Response<NeutronMsg>> {
    INTEGRATION_TESTS_QUERY_MOCK.save(deps.storage, &IntegrationTestsQueryMock::Enabled)?;
    Ok(Response::default())
}

pub fn unset_query_mock(deps: DepsMut<InterchainQueries>) -> NeutronResult<Response<NeutronMsg>> {
    INTEGRATION_TESTS_QUERY_MOCK.save(deps.storage, &IntegrationTestsQueryMock::Disabled)?;
    Ok(Response::default())
}
