use crate::state::TOTAL_MINTED_TOKENS;
use cosmwasm_std::Addr;
use cosmwasm_std::Deps;
use cosmwasm_std::DepsMut;
use cosmwasm_std::Env;
use cosmwasm_std::Response;
use cosmwasm_std::StdError;
use neutron_sdk::bindings::msg::NeutronMsg;
use neutron_sdk::bindings::query::NeutronQuery;
use neutron_sdk::NeutronResult;

use crate::state::MINTED_TOKENS;

const MINT_AMOUNT: u128 = 100;
pub const THRESHOLD_BURN_AMOUNT: u128 = 50;
const NEUTRON_BECH32_PREFIX: &str = "neutron";

/// This function transfer the addr to a local neutron addr
pub fn any_addr_to_neutron(deps: Deps<NeutronQuery>, addr: String) -> NeutronResult<Addr> {
    // TODO, test this snippet
    let (_hrp, data, _variant) = bech32::decode(&addr)?;
    let neutron_addr = bech32::encode(NEUTRON_BECH32_PREFIX, data, bech32::Variant::Bech32)?;
    Ok(deps.api.addr_validate(&neutron_addr)?)
}

// In order to create a token denom that is reasonable and doesn't make assumptions on token name length
// We give each new token transfer an increasing id (see https://docs.neutron.org/neutron/modules/3rdparty/osmosis/tokenfactory/overview)
pub fn format_token_sub_denom(_token_id: String, token_count: u64) -> String {
    format!("{}", token_count)
}

// In order to create a token denom that is reasonable and doesn't make assumptions on token name length
// We give each new token transfer an increasing id (see https://docs.neutron.org/neutron/modules/3rdparty/osmosis/tokenfactory/overview)
pub fn format_token_denom(env: Env, token_id: String, token_count: u64) -> String {
    let sub_denom = format_token_sub_denom(token_id, token_count);

    format!("factory/{}/{}", env.contract.address, sub_denom)
}

pub fn mint_native_receipt(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    token_id: String,
    addr: Addr,
) -> NeutronResult<Response<NeutronMsg>> {
    // First we see where we are at in terms of numbers of tokens
    let token_count = TOTAL_MINTED_TOKENS.load(deps.storage).unwrap_or(0);
    TOTAL_MINTED_TOKENS.save(deps.storage, &(token_count + 1))?;

    // MINTED_TOKENS should not contain anything at token id location
    MINTED_TOKENS.update(deps.storage, token_id.clone(), |s| match s {
        None => Ok(token_count),
        Some(_) => Err(StdError::generic_err(format!(
            "Token {}, was already migrated",
            token_id
        ))),
    })?;

    let subdenom = format_token_sub_denom(token_id.clone(), token_count);

    Ok(Response::new()
        .add_message(NeutronMsg::CreateDenom { subdenom })
        .add_message(NeutronMsg::MintTokens {
            denom: format_token_denom(env, token_id, token_count),
            amount: MINT_AMOUNT.into(),
            mint_to_address: addr.to_string(),
        }))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::MockApi;
    use cosmwasm_std::testing::MockQuerier;
    use cosmwasm_std::testing::MockStorage;
    use cosmwasm_std::OwnedDeps;
    use neutron_sdk::NeutronResult;
    use std::marker::PhantomData;

    use super::any_addr_to_neutron;

    #[test]
    fn right_address_generation() -> NeutronResult<()> {
        let deps = OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: MockQuerier::default(),
            custom_query_type: PhantomData,
        };
        let address = "stars1phaxpevm5wecex2jyaqty2a4v02qj7qmruxmf7";
        let neutron_addr = any_addr_to_neutron(deps.as_ref(), address.to_string())?;
        assert_eq!(
            neutron_addr,
            "neutron1phaxpevm5wecex2jyaqty2a4v02qj7qmnlcycg"
        );

        Ok(())
    }
}
