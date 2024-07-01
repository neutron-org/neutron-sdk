use std::ops::Div;
// import all types from v045 package
// to make it available from v047 package (kinda proxy) since they work with Cosmos SDK 0.47 as usual
pub use crate::interchain_queries::v045::types::*;

// But at the same time we replace some structs from v045 with structs below to create structures
// compatible with Cosmos SDK 0.47

use crate::interchain_queries::types::KVReconstruct;
use crate::{bindings::types::StorageValue, errors::error::NeutronResult, NeutronError};

use crate::interchain_queries::helpers::uint256_to_u128;
use crate::interchain_queries::v047::helpers::deconstruct_account_denom_balance_key;
use cosmos_sdk_proto::cosmos::staking::v1beta1::{
    Delegation, Params, Validator as CosmosValidator,
};
use cosmos_sdk_proto::traits::Message;
use cosmwasm_std::{Addr, Coin, Decimal256, Uint128, Uint256};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Key for Staking Params in the **staking** module's storage
/// <https://github.com/cosmos/cosmos-sdk/blob/54120626e9994b2f1cc7a2bebc60cfb99703028f/x/staking/types/keys.go#L56>
pub const STAKING_PARAMS_KEY: u8 = 0x51;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// A structure that can be reconstructed from **StorageValues**'s for the **Balance Interchain Query**.
/// Contains amounts of coins that are held by some account on remote chain.
pub struct Balances {
    pub coins: Vec<Coin>,
}

impl KVReconstruct for Balances {
    fn reconstruct(storage_values: &[StorageValue]) -> NeutronResult<Balances> {
        let mut coins: Vec<Coin> = Vec::with_capacity(storage_values.len());

        for kv in storage_values {
            let (_, denom) = deconstruct_account_denom_balance_key(kv.key.to_vec())?;
            let amount = if kv.value.is_empty() {
                Uint128::zero()
            } else {
                Uint128::from_str(&String::from_utf8(kv.value.to_vec())?)?
            };

            coins.push(Coin::new(amount.u128(), denom))
        }

        Ok(Balances { coins })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// A structure that can be reconstructed from **StorageValues**'s for the **Delegator Delegation Interchain Query**.
/// Contains delegations which some delegator has on remote chain.
pub struct Delegations {
    pub delegations: Vec<StdDelegation>,
}

impl KVReconstruct for Delegations {
    fn reconstruct(storage_values: &[StorageValue]) -> NeutronResult<Delegations> {
        // We are taking 2 items chunks from starage_value to calculate one delegation
        let mut delegations: Vec<StdDelegation> = Vec::with_capacity(storage_values.len() / 2);

        if storage_values.is_empty() {
            return Err(NeutronError::InvalidQueryResultFormat(
                "storage_values length is 0".into(),
            ));
        }
        // first StorageValue is staking params
        if storage_values[0].value.is_empty() {
            // Incoming params cannot be empty, it should always be configured on chain.
            // If we receive empty params, that means incoming data structure is corrupted
            // and we cannot build `cosmwasm_std::Delegation`'s using this data.
            return Err(NeutronError::InvalidQueryResultFormat(
                "params is empty".into(),
            ));
        }
        let params: Params = Params::decode(storage_values[0].value.as_slice())?;

        // the rest are delegations and validators alternately
        for chunk in storage_values[1..].chunks(2) {
            if chunk[0].value.is_empty() {
                // Incoming delegation can actually be empty, this just means that delegation
                // is not present on remote chain, which is to be expected. So, if it doesn't
                // exist, we can safely skip this and following chunk.
                continue;
            }
            let delegation_sdk: Delegation = Delegation::decode(chunk[0].value.as_slice())?;

            let mut delegation_std = StdDelegation {
                delegator: Addr::unchecked(delegation_sdk.delegator_address.as_str()),
                validator: delegation_sdk.validator_address,
                amount: Default::default(),
            };

            if chunk[1].value.is_empty() {
                // At this point, incoming validator cannot be empty, that would be invalid,
                // because delegation is already defined, so, building `cosmwasm_std::Delegation`
                // from this data is impossible, incoming data is corrupted.post
                return Err(NeutronError::InvalidQueryResultFormat(
                    "validator is empty".into(),
                ));
            }
            let validator: CosmosValidator = CosmosValidator::decode(chunk[1].value.as_slice())?;

            let delegation_shares = Decimal256::from_atomics(
                Uint256::from_str(&delegation_sdk.shares)?,
                DECIMAL_PLACES,
            )?;

            let delegator_shares = Decimal256::from_atomics(
                Uint256::from_str(&validator.delegator_shares)?,
                DECIMAL_PLACES,
            )?;

            let validator_tokens =
                Decimal256::from_atomics(Uint128::from_str(&validator.tokens)?, 0)?;

            // https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/staking/keeper/querier.go#L463
            // delegated_tokens = quotient(delegation.shares * validator.tokens / validator.total_shares);
            let delegated_tokens = delegation_shares
                .checked_mul(validator_tokens)?
                .div(delegator_shares)
                .atomics()
                .div(Uint256::from_u128(DECIMAL_FRACTIONAL));

            delegation_std.amount =
                Coin::new(uint256_to_u128(delegated_tokens)?, &params.bond_denom);

            delegations.push(delegation_std);
        }

        Ok(Delegations { delegations })
    }
}
