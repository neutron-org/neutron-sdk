use crate::errors::error::{ContractError, ContractResult};
use crate::interchain_queries::types::{
    AddressBytes, BALANCES_PREFIX, DELEGATION_KEY, MAX_ADDR_LEN, PARAMS_STORE_DELIMITER,
    VALIDATORS_KEY,
};

/// Creates KV key to get **module** param by **key**
pub fn create_params_store_key(module: &str, key: &str) -> Vec<u8> {
    let mut s = String::with_capacity(module.len() + 1 + key.len());
    s.push_str(module);
    s.push_str(PARAMS_STORE_DELIMITER);
    s.push_str(key);

    s.into_bytes()
}

/// Decodes a bech32 encoded string and converts to base64 encoded bytes
/// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/types/bech32/bech32.go#L20
pub fn decode_and_convert(encoded: &str) -> ContractResult<AddressBytes> {
    let (_hrp, bytes, _variant) = bech32::decode(encoded)?;

    Ok(bech32::convert_bits(&bytes, 5, 8, false)?)
}

/// Prefixes the address bytes with its length
pub fn length_prefix<AddrBytes: AsRef<[u8]>>(addr: AddrBytes) -> ContractResult<Vec<u8>> {
    let bz_length = addr.as_ref().len();

    if bz_length == 0 {
        return Ok(vec![]);
    }

    if bz_length > MAX_ADDR_LEN {
        return Err(ContractError::MaxAddrLength {
            max: MAX_ADDR_LEN,
            actual: bz_length,
        });
    }

    let mut p: Vec<u8> = vec![bz_length as u8];
    p.extend_from_slice(addr.as_ref());

    Ok(p)
}

/// Creates balances Cosmos-SDK storage prefix for account with **addr**
/// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/x/bank/types/key.go#L55
pub fn create_account_balances_prefix<AddrBytes: AsRef<[u8]>>(
    addr: AddrBytes,
) -> ContractResult<Vec<u8>> {
    let mut prefix: Vec<u8> = vec![BALANCES_PREFIX];
    prefix.extend_from_slice(length_prefix(addr)?.as_slice());

    Ok(prefix)
}

/// Creates delegations Cosmos-SDK storage prefix for delegator with **delegator_addr**
/// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/x/staking/types/keys.go#L181
pub fn create_delegations_key<AddrBytes: AsRef<[u8]>>(
    delegator_address: AddrBytes,
) -> ContractResult<Vec<u8>> {
    let mut key: Vec<u8> = vec![DELEGATION_KEY];
    key.extend_from_slice(length_prefix(delegator_address)?.as_slice());

    Ok(key)
}

/// Creates Cosmos-SDK storage key for delegation between delegator with **delegator_addr** and validator with **validator_addr**
/// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/x/staking/types/keys.go#L176
pub fn create_delegation_key<AddrBytes: AsRef<[u8]>>(
    delegator_address: AddrBytes,
    validator_address: AddrBytes,
) -> ContractResult<Vec<u8>> {
    let mut delegations_key: Vec<u8> = create_delegations_key(delegator_address)?;
    delegations_key.extend_from_slice(length_prefix(validator_address)?.as_slice());

    Ok(delegations_key)
}

/// Creates Cosmos-SDK storage key for validator with **operator_address**
/// https://github.com/cosmos/cosmos-sdk/blob/f2d94445c0f5f52cf5ed999b81048b575de94964/x/staking/types/keys.go#L55
pub fn create_validator_key<AddrBytes: AsRef<[u8]>>(
    operator_address: AddrBytes,
) -> ContractResult<Vec<u8>> {
    let mut key: Vec<u8> = vec![VALIDATORS_KEY];
    key.extend_from_slice(length_prefix(operator_address)?.as_slice());

    Ok(key)
}
