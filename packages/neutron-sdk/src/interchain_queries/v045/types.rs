use crate::interchain_queries::types::KVReconstruct;
use crate::{
    bindings::types::StorageValue,
    errors::error::{NeutronError, NeutronResult},
};
use cosmos_sdk_proto::cosmos::{
    base::v1beta1::Coin as CosmosCoin,
    distribution::v1beta1::FeePool as CosmosFeePool,
    gov::v1beta1::Proposal as CosmosProposal,
    staking::v1beta1::{Delegation, Validator as CosmosValidator},
};
use cosmwasm_std::{from_binary, Addr, Coin, Decimal, StdError, Uint128};
use prost::Message as ProstMessage;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{ops::Div, str::FromStr};

use super::helpers::{
    get_max_change_rate, get_max_rate, get_rate, get_total_supply_amount, get_total_supply_denom,
    get_update_time,
};

pub const DECIMAL_PLACES: u32 = 18;
const DECIMAL_FRACTIONAL: u128 = 10u128.pow(DECIMAL_PLACES);

/// Protobuf type url of standard Cosmos SDK bank transfer message
pub const COSMOS_SDK_TRANSFER_MSG_URL: &str = "/cosmos.bank.v1beta1.MsgSend";

/// Storage prefix for account balances store
/// <https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/bank/types/key.go#L27>
pub const BALANCES_PREFIX: u8 = 0x02;

/// Storage prefix for bank supply store
/// <https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/bank/types/key.go#L28>
pub const SUPPLY_PREFIX: u8 = 0x00;

/// Key for delegations in the **staking** module's storage
/// <https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/staking/types/keys.go#L39>
pub const DELEGATION_KEY: u8 = 0x31;

/// Key for validators in the **staking** module's storage
/// <https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/staking/types/keys.go#L35>
pub const VALIDATORS_KEY: u8 = 0x21;

/// Key for Fee Pool in the **distribution** module's storage
/// <https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/distribution/types/keys.go#L46>
pub const FEE_POOL_KEY: u8 = 0x00;

/// Key for Proposals in the **gov** module's storage
/// <https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/gov/types/keys.go#L41>
pub const PROPOSALS_KEY_PREFIX: u8 = 0x00;

/// Key for Wasm Contract Store in the **wasm** module's storage
/// <https://github.com/CosmWasm/wasmd/blob/e6d451bf9dd96a555b10e72aa3c0f6b820d34684/x/wasm/types/keys.go#L28>
pub const WASM_CONTRACT_STORE_PREFIX: u8 = 0x03;

/// Name of the standard **bank** Cosmos-SDK module
pub const BANK_STORE_KEY: &str = "bank";

/// Name of the standard **staking** Cosmos-SDK module
pub const STAKING_STORE_KEY: &str = "staking";

/// Name of the standard **distribution** Cosmos-SDK module
pub const DISTRIBUTION_STORE_KEY: &str = "distribution";

/// Name of the standard **gov** Cosmos-SDK module
pub const GOV_STORE_KEY: &str = "gov";

/// Key for bond denomination param of Cosmos-SDK staking module
/// <https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/staking/types/params.go#L39>
pub const KEY_BOND_DENOM: &str = "BondDenom";

/// Name of the standard **params** Cosmos-SDK module
pub const PARAMS_STORE_KEY: &str = "params";

/// Default delimiter of **params** Cosmos-SDK module
pub const PARAMS_STORE_DELIMITER: &str = "/";

/// Name of the **wasm** Cosmos module
pub const WASM_STORE_KEY: &str = "wasm";

pub const RECIPIENT_FIELD: &str = "transfer.recipient";
pub const HEIGHT_FIELD: &str = "tx.height";

impl KVReconstruct for Uint128 {
    fn reconstruct(storage_values: &[StorageValue]) -> NeutronResult<Uint128> {
        let value = storage_values
            .first()
            .ok_or_else(|| StdError::generic_err("empty query result"))?;
        let balance: Uint128 = from_binary(&value.value)?;
        Ok(balance)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// A structure that can be reconstructed from **StorageValues**'s for the **Balance Interchain Query**.
/// Contains coins that are held by some account on remote chain.
pub struct Balances {
    pub coins: Vec<Coin>,
}

impl KVReconstruct for Balances {
    fn reconstruct(storage_values: &[StorageValue]) -> NeutronResult<Balances> {
        let mut coins: Vec<Coin> = Vec::with_capacity(storage_values.len());

        for kv in storage_values {
            let balance: CosmosCoin = CosmosCoin::decode(kv.value.as_slice())?;
            let amount = Uint128::from_str(balance.amount.as_str())?;
            coins.push(Coin::new(amount.u128(), balance.denom));
        }

        Ok(Balances { coins })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// A structure that can be reconstructed from **StorageValues**'s for the **Bank Total Interchain Query**.
/// Contains total supply for specific denom that are held on remote chain.
pub struct TotalSupply {
    pub coins: Vec<Coin>,
}

impl KVReconstruct for TotalSupply {
    fn reconstruct(storage_values: &[StorageValue]) -> NeutronResult<TotalSupply> {
        let mut coins: Vec<Coin> = Vec::with_capacity(storage_values.len());

        for kv in storage_values {
            let denom = get_total_supply_denom(&kv.key);
            let amount = get_total_supply_amount(&kv.value);
            if let (Some(denom), Some(amount)) = (denom, amount) {
                coins.push(Coin::new(amount.u128(), denom));
            }
        }
        Ok(TotalSupply { coins })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// A structure that can be reconstructed from **StorageValues**'s for the **Fee Pool Interchain Query**.
/// Contains coins that are held by some account on remote chain.
pub struct FeePool {
    pub coins: Vec<Coin>,
}

impl KVReconstruct for FeePool {
    fn reconstruct(storage_values: &[StorageValue]) -> NeutronResult<FeePool> {
        let mut coins: Vec<Coin> = Vec::with_capacity(storage_values.len());

        for kv in storage_values {
            let cosmos_pool: CosmosFeePool = CosmosFeePool::decode(kv.value.as_slice())?;

            for pool_coin in cosmos_pool.community_pool {
                // amount is stored as Dec which is gogo proto encoded to string without a decimal point.
                // e.g. Dec(1) is 1 + 18 zeros
                // https://github.com/cosmos/cosmos-sdk/blob/9c145c827001222df2e3e1101010874aeac20997/types/decimal_test.go#L498
                let amount: Uint128 = Uint128::from_str(pool_coin.amount.as_str())
                    .unwrap()
                    .checked_div_floor(
                        Decimal::one()
                            .checked_mul(Decimal::from_ratio(10u64, 1u64).pow(DECIMAL_PLACES))
                            .unwrap(),
                    )
                    .unwrap();
                coins.push(Coin::new(amount.u128(), pool_coin.denom));
            }
        }

        Ok(FeePool { coins })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// Validator structure for the querier. Contains validator from staking module
pub struct Validator {
    pub operator_address: String,
    /// jailed defined whether the validator has been jailed from bonded status or not.
    pub jailed: bool,
    /// status is the validator status (bonded/unbonding/unbonded).
    pub status: i32,
    /// tokens define the delegated tokens (incl. self-delegation).
    pub tokens: String,
    /// delegator_shares defines total shares issued to a validator's delegators.
    pub delegator_shares: String,
    /// moniker defines a human-readable name for the validator.
    pub moniker: Option<String>,
    /// identity defines an optional identity signature (ex. UPort or Keybase).
    pub identity: Option<String>,
    /// website defines an optional website link.
    pub website: Option<String>,
    /// security_contact defines an optional email for security contact.
    pub security_contact: Option<String>,
    /// details define other optional details.
    pub details: Option<String>,
    /// unbonding_height defines, if unbonding, the height at which this validator has begun unbonding.
    pub unbonding_height: u64,
    /// unbonding_time defines, if unbonding, the min time for the validator to complete unbonding.
    pub unbonding_time: Option<u64>,
    /// rate is the commission rate charged to delegators, as a fraction.
    pub rate: Option<Decimal>,
    /// max_rate defines the maximum commission rate which validator can ever charge, as a fraction.
    pub max_rate: Option<Decimal>,
    /// max_change_rate defines the maximum daily increase of the validator commission, as a fraction.
    pub max_change_rate: Option<Decimal>,
    /// update_time is the last time the commission rate was changed.
    pub update_time: Option<u64>,
    /// min_self_delegation is the validator's self declared minimum self delegation.
    pub min_self_delegation: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// A structure that can be reconstructed from **StorageValues**'s for the **Staking Validator Interchain Query**.
/// Contains validator info from remote chain.
pub struct StakingValidator {
    pub validators: Vec<Validator>,
}

impl KVReconstruct for StakingValidator {
    fn reconstruct(storage_values: &[StorageValue]) -> NeutronResult<StakingValidator> {
        let mut validators = Vec::with_capacity(storage_values.len());

        for kv in storage_values {
            let validator: CosmosValidator = CosmosValidator::decode(kv.value.as_slice())?;
            let description = &validator.description;
            let commission = &validator.commission;

            let validator = Validator {
                operator_address: validator.operator_address,
                delegator_shares: validator.delegator_shares,
                jailed: validator.jailed,
                status: validator.status,
                tokens: validator.tokens,
                unbonding_height: validator.unbonding_height as u64,
                unbonding_time: validator.unbonding_time.map(|v| v.seconds as u64),
                moniker: description.as_ref().map(|v| v.moniker.to_string()),
                identity: description.as_ref().map(|v| v.identity.to_string()),
                website: description.as_ref().map(|v| v.website.to_string()),
                security_contact: description.as_ref().map(|v| v.security_contact.to_string()),
                details: description.as_ref().map(|v| v.details.to_string()),
                max_change_rate: get_max_change_rate(commission),
                max_rate: get_max_rate(commission),
                rate: get_rate(commission),
                update_time: get_update_time(commission),
                min_self_delegation: Decimal::from_str(validator.min_self_delegation.as_str())
                    .unwrap_or_default(),
            };

            validators.push(validator)
        }

        Ok(StakingValidator { validators })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// TallyResult defines a standard tally for a governance proposal.
pub struct TallyResult {
    pub yes: String,
    pub no: String,
    pub abstain: String,
    pub no_with_veto: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// Proposal defines the core field members of a governance proposal.
pub struct Proposal {
    pub proposal_id: u64,
    pub proposal_type: Option<String>,
    pub total_deposit: Vec<Coin>,
    pub status: i32,
    pub submit_time: Option<u64>,
    pub deposit_end_time: Option<u64>,
    pub voting_start_time: Option<u64>,
    pub voting_end_time: Option<u64>,
    pub final_tally_result: Option<TallyResult>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// A structure that can be reconstructed from **StorageValues**'s for the **Government Proposal Interchain Query**.
/// Contains coins that are held by some account on remote chain.
pub struct GovernmentProposal {
    pub proposals: Vec<Proposal>,
}

impl KVReconstruct for GovernmentProposal {
    fn reconstruct(storage_values: &[StorageValue]) -> NeutronResult<GovernmentProposal> {
        let mut proposals = Vec::with_capacity(storage_values.len());

        for kv in storage_values {
            let proposal: CosmosProposal = CosmosProposal::decode(kv.value.as_slice())?;

            let mut coins: Vec<Coin> = Vec::with_capacity(proposal.total_deposit.len());

            for coin in proposal.total_deposit {
                let amount = Uint128::from_str(coin.amount.as_str())?;
                coins.push(Coin::new(amount.u128(), coin.denom));
            }

            let final_tally_result = &proposal.final_tally_result;

            let proposal = Proposal {
                proposal_id: proposal.proposal_id,
                proposal_type: proposal.content.map(|v| v.type_url),
                total_deposit: coins,
                status: proposal.status,
                submit_time: proposal.submit_time.map(|v| v.seconds as u64),
                deposit_end_time: proposal.deposit_end_time.map(|v| v.seconds as u64),
                voting_end_time: proposal.voting_end_time.map(|v| v.seconds as u64),
                voting_start_time: proposal.voting_start_time.map(|v| v.seconds as u64),
                final_tally_result: final_tally_result.as_ref().map(|v| TallyResult {
                    abstain: v.abstain.to_string(),
                    no: v.no.to_string(),
                    no_with_veto: v.no_with_veto.to_string(),
                    yes: v.yes.to_string(),
                }),
            };

            proposals.push(proposal);
        }

        Ok(GovernmentProposal { proposals })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// A structure that can be reconstructed from **StorageValues**'s for the **Delegator Delegation Interchain Query**.
/// Contains delegations which some delegator has on remote chain.
pub struct Delegations {
    pub delegations: Vec<cosmwasm_std::Delegation>,
}

impl KVReconstruct for Delegations {
    fn reconstruct(storage_values: &[StorageValue]) -> NeutronResult<Delegations> {
        // We are taking 2 items chunks from starage_value to calculate one delegation
        let mut delegations: Vec<cosmwasm_std::Delegation> =
            Vec::with_capacity(storage_values.len() / 2);

        if storage_values.is_empty() {
            return Err(NeutronError::InvalidQueryResultFormat(
                "storage_values length is 0".into(),
            ));
        }
        // first StorageValue is denom
        if storage_values[0].value.is_empty() {
            // Incoming denom cannot be empty, it should always be configured on chain.
            // If we receive empty denom, that means incoming data structure is corrupted
            // and we cannot build `cosmwasm_std::Delegation`'s using this data.
            return Err(NeutronError::InvalidQueryResultFormat(
                "denom is empty".into(),
            ));
        }
        let denom: String = from_binary(&storage_values[0].value)?;

        // the rest are delegations and validators alternately
        for chunk in storage_values[1..].chunks(2) {
            if chunk[0].value.is_empty() {
                // Incoming delegation can actually be empty, this just means that delegation
                // is not present on remote chain, which is to be expected. So, if it doesn't
                // exist, we can safely skip this and following chunk.
                continue;
            }
            let delegation_sdk: Delegation = Delegation::decode(chunk[0].value.as_slice())?;

            let mut delegation_std = cosmwasm_std::Delegation {
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

            let delegation_shares =
                Decimal::from_atomics(Uint128::from_str(&delegation_sdk.shares)?, DECIMAL_PLACES)?;

            let delegator_shares = Decimal::from_atomics(
                Uint128::from_str(&validator.delegator_shares)?,
                DECIMAL_PLACES,
            )?;

            let validator_tokens = Decimal::from_atomics(Uint128::from_str(&validator.tokens)?, 0)?;

            // https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/staking/keeper/querier.go#L463
            // delegated_tokens = quotient(delegation.shares * validator.tokens / validator.total_shares);
            let delegated_tokens = delegation_shares
                .checked_mul(validator_tokens)?
                .div(delegator_shares)
                .atomics()
                .u128()
                .div(DECIMAL_FRACTIONAL);

            delegation_std.amount = Coin::new(delegated_tokens, &denom);

            delegations.push(delegation_std);
        }

        Ok(Delegations { delegations })
    }
}
