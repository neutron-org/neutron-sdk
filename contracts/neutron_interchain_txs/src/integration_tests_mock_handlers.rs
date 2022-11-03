use crate::storage::{IntegrationTestsSudoMock, INTEGRATION_TESTS_SUDO_MOCK};
use cosmwasm_std::{DepsMut, Response, StdResult};
use neutron_sdk::bindings::msg::NeutronMsg;

pub fn set_sudo_failure_mock(deps: DepsMut) -> StdResult<Response<NeutronMsg>> {
    INTEGRATION_TESTS_SUDO_MOCK.save(deps.storage, &IntegrationTestsSudoMock::Enabled)?;
    Ok(Response::default())
}

pub fn unset_sudo_failure_mock(deps: DepsMut) -> StdResult<Response<NeutronMsg>> {
    INTEGRATION_TESTS_SUDO_MOCK.save(deps.storage, &IntegrationTestsSudoMock::Disabled)?;
    Ok(Response::default())
}
