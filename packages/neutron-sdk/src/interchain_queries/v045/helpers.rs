use crate::errors::error::NeutronResult;
use crate::interchain_queries::helpers::length_prefix;
use crate::interchain_queries::v045::types::{
    BALANCES_PREFIX, DELEGATION_KEY, FEE_POOL_KEY, PARAMS_STORE_DELIMITER, PROPOSALS_KEY_PREFIX,
    SUPPLY_PREFIX, VALIDATORS_KEY, WASM_CONTRACT_STORE_PREFIX,
};
use cosmos_sdk_proto::cosmos::staking::v1beta1::Commission as ValidatorCommission;
use cosmwasm_std::{Binary, Decimal, Uint128};
use std::str::{from_utf8, FromStr};

/// Creates KV key to get **module** param by **key**
pub fn create_params_store_key(module: &str, key: &str) -> Vec<u8> {
    let mut s = String::with_capacity(module.len() + 1 + key.len());
    s.push_str(module);
    s.push_str(PARAMS_STORE_DELIMITER);
    s.push_str(key);

    s.into_bytes()
}

/// Creates balances Cosmos-SDK storage prefix for account with **addr**
/// <https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/x/bank/types/key.go#L55>
pub fn create_account_balances_prefix<AddrBytes: AsRef<[u8]>>(
    addr: AddrBytes,
) -> NeutronResult<Vec<u8>> {
    let mut prefix: Vec<u8> = vec![BALANCES_PREFIX];
    prefix.extend_from_slice(length_prefix(addr)?.as_slice());

    Ok(prefix)
}

/// Creates **denom** balance Cosmos-SDK storage key for account with **addr**
pub fn create_account_denom_balance_key<AddrBytes: AsRef<[u8]>, S: AsRef<str>>(
    addr: AddrBytes,
    denom: S,
) -> NeutronResult<Vec<u8>> {
    let mut account_balance_key = create_account_balances_prefix(addr)?;
    account_balance_key.extend_from_slice(denom.as_ref().as_bytes());

    Ok(account_balance_key)
}

/// Creates **denom** balance Cosmos-SDK storage key for account with **addr**
pub fn create_denom_balance_key<AddrBytes: AsRef<[u8]>, S: AsRef<str>>(
    addr: AddrBytes,
    denom: S,
) -> NeutronResult<Vec<u8>> {
    let mut account_balance_key = create_account_balances_prefix(addr)?;
    account_balance_key.extend_from_slice(denom.as_ref().as_bytes());

    Ok(account_balance_key)
}

/// Creates **denom** total Cosmos-SDK storage key for bank module
pub fn create_total_denom_key<S: AsRef<str>>(denom: S) -> NeutronResult<Vec<u8>> {
    let mut total_supply: Vec<u8> = vec![SUPPLY_PREFIX];
    total_supply.extend_from_slice(denom.as_ref().as_bytes());

    Ok(total_supply)
}

/// Creates delegations Cosmos-SDK storage prefix for delegator with **delegator_addr**
/// <https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/x/staking/types/keys.go#L181>
pub fn create_delegations_key<AddrBytes: AsRef<[u8]>>(
    delegator_address: AddrBytes,
) -> NeutronResult<Vec<u8>> {
    let mut key: Vec<u8> = vec![DELEGATION_KEY];
    key.extend_from_slice(length_prefix(delegator_address)?.as_slice());

    Ok(key)
}

/// Creates Cosmos-SDK storage key for delegation between delegator with **delegator_addr** and validator with **validator_addr**
/// <https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/x/staking/types/keys.go#L176>
pub fn create_delegation_key<AddrBytes: AsRef<[u8]>>(
    delegator_address: AddrBytes,
    validator_address: AddrBytes,
) -> NeutronResult<Vec<u8>> {
    let mut delegations_key: Vec<u8> = create_delegations_key(delegator_address)?;
    delegations_key.extend_from_slice(length_prefix(validator_address)?.as_slice());

    Ok(delegations_key)
}

/// Creates Cosmos-SDK storage key for validator with **operator_address**
/// <https://github.com/cosmos/cosmos-sdk/blob/f2d94445c0f5f52cf5ed999b81048b575de94964/x/staking/types/keys.go#L55>
pub fn create_validator_key<AddrBytes: AsRef<[u8]>>(
    operator_address: AddrBytes,
) -> NeutronResult<Vec<u8>> {
    let mut key: Vec<u8> = vec![VALIDATORS_KEY];
    key.extend_from_slice(length_prefix(operator_address)?.as_slice());

    Ok(key)
}

/// Creates Wasm key for contract state.
/// This function is similar to
/// <https://github.com/CosmWasm/wasmd/blob/e6d451bf9dd96a555b10e72aa3c0f6b820d34684/x/wasm/types/keys.go#L59>,
/// but it also concatenates resulting contract store prefix with contract's storage key,
/// resulting in a complete storage key.
pub fn create_wasm_contract_store_key<AddrBytes: AsRef<[u8]>, Key: AsRef<[u8]>>(
    contract_address: AddrBytes,
    key: Key,
) -> NeutronResult<Vec<u8>> {
    let mut prefix: Vec<u8> = vec![WASM_CONTRACT_STORE_PREFIX];
    prefix.extend_from_slice(contract_address.as_ref());
    prefix.extend_from_slice(key.as_ref());

    Ok(prefix)
}

/// Creates Cosmos-SDK distribution key for fee pool
/// <https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/distribution/types/keys.go#L46>
pub fn create_fee_pool_key() -> NeutronResult<Vec<u8>> {
    let key: Vec<u8> = vec![FEE_POOL_KEY];

    Ok(key)
}

/// Creates Cosmos-SDK governance key for proposal with specific id
/// <https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/gov/types/keys.go#L41>
pub fn create_gov_proposal_key(proposal_id: u64) -> NeutronResult<Vec<u8>> {
    let mut key: Vec<u8> = vec![PROPOSALS_KEY_PREFIX];
    key.extend_from_slice(proposal_id.to_be_bytes().as_slice());

    Ok(key)
}

/// Returns validator max change rate
pub fn get_max_change_rate(commission: &Option<ValidatorCommission>) -> Option<Decimal> {
    let commission_rates = commission.as_ref().map(|v| v.commission_rates.as_ref())?;
    commission_rates
        .map(|v| Decimal::new(Uint128::from_str(v.max_change_rate.as_str()).unwrap_or_default()))
}

/// Returns validator max rate
pub fn get_max_rate(commission: &Option<ValidatorCommission>) -> Option<Decimal> {
    let commission_rates = commission.as_ref().map(|v| v.commission_rates.as_ref())?;
    commission_rates
        .map(|v| Decimal::new(Uint128::from_str(v.max_rate.as_str()).unwrap_or_default()))
}

/// Returns current validator rate
pub fn get_rate(commission: &Option<ValidatorCommission>) -> Option<Decimal> {
    let commission_rates = commission.as_ref().map(|v| v.commission_rates.as_ref())?;
    commission_rates.map(|v| Decimal::new(Uint128::from_str(v.rate.as_str()).unwrap_or_default()))
}

/// Returns current validator rate
pub fn get_update_time(commission: &Option<ValidatorCommission>) -> Option<u64> {
    let commission_rates = commission.as_ref().map(|v| v.update_time.as_ref())?;
    commission_rates.map(|v| v.seconds as u64)
}

/// Returns denom for total supply from StorageValue key
pub fn get_total_supply_denom(denom: &Binary) -> Option<String> {
    if denom.len() < 2 {
        return None;
    }
    if denom[0] == SUPPLY_PREFIX {
        // We need to cut off first byte because it contains storage key following by denom.
        return from_utf8(&denom[1..]).ok().map(|d| d.to_string());
    }

    None
}

/// Returns total supply amount from StorageValue key
pub fn get_total_supply_amount(amount: &Binary) -> Option<Uint128> {
    from_utf8(amount).ok().map(|a| Uint128::from_str(a).ok())?
}
