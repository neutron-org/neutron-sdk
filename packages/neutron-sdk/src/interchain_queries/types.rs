use crate::{
    bindings::types::{KVKey, StorageValue},
    errors::error::{NeutronError, NeutronResult},
};
use cosmos_sdk_proto::cosmos::{
    base::v1beta1::Coin as CosmosCoin,
    distribution::v1beta1::FeePool as CosmosFeePool,
    gov::v1beta1::Proposal as CosmosProposal,
    staking::v1beta1::{Delegation, Validator as CosmosValidator},
};
use cosmwasm_std::{from_binary, Addr, Coin, Decimal, Uint128};
use prost::Message as ProstMessage;
use schemars::{JsonSchema, _serde_json::Value};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{ops::Div, str::FromStr};

use super::helpers::{
    get_max_change_rate, get_max_rate, get_rate, get_total_supply_amount, get_total_supply_denom,
    get_update_time,
};

const DECIMAL_PLACES: u32 = 18;
const DECIMAL_FRACTIONAL: u128 = 10u128.pow(DECIMAL_PLACES);

const QUERY_TYPE_KV_VALUE: &str = "kv";
const QUERY_TYPE_TX_VALUE: &str = "tx";

/// Protobuf type url of standard Cosmos SDK bank transfer message
pub const COSMOS_SDK_TRANSFER_MSG_URL: &str = "/cosmos.bank.v1beta1.MsgSend";

/// Storage prefix for account balances store
/// https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/bank/types/key.go#L27
pub const BALANCES_PREFIX: u8 = 0x02;

/// Storage prefix for bank supply store
/// https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/bank/types/key.go#L28
pub const SUPPLY_PREFIX: u8 = 0x00;

/// Key for delegations in the **staking** module's storage
/// https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/staking/types/keys.go#L39
pub const DELEGATION_KEY: u8 = 0x31;

/// Key for validators in the **staking** module's storage
/// https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/staking/types/keys.go#L35
pub const VALIDATORS_KEY: u8 = 0x21;

/// Key for Fee Pool in the **distribution** module's storage
/// https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/distribution/types/keys.go#L46
pub const FEE_POOL_KEY: u8 = 0x00;

/// Key for Proposals in the **gov** module's storage
/// https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/gov/types/keys.go#L41
pub const PROPOSALS_KEY_PREFIX: u8 = 0x00;

/// Maximum length of address
pub const MAX_ADDR_LEN: usize = 255;

/// Name of the standard **bank** Cosmos-SDK module
pub const BANK_STORE_KEY: &str = "bank";

/// Name of the standard **staking** Cosmos-SDK module
pub const STAKING_STORE_KEY: &str = "staking";

/// Name of the standard **distribution** Cosmos-SDK module
pub const DISTRIBUTION_STORE_KEY: &str = "distribution";

/// Name of the standard **gov** Cosmos-SDK module
pub const GOV_STORE_KEY: &str = "gov";

/// Key for bond denomination param of Cosmos-SDK staking module
/// https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/staking/types/params.go#L39
pub const KEY_BOND_DENOM: &str = "BondDenom";

/// Name of the standard **params** Cosmos-SDK module
pub const PARAMS_STORE_KEY: &str = "params";

/// Default delimiter of **params** Cosmos-SDK module
pub const PARAMS_STORE_DELIMITER: &str = "/";

pub const RECIPIENT_FIELD: &str = "transfer.recipient";
pub const HEIGHT_FIELD: &str = "tx.height";

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum TransactionFilterOp {
    Eq,
    Lt,
    Gt,
    Lte,
    Gte,
}

#[derive(PartialEq, Eq, Debug)]
pub enum TransactionFilterValue {
    String(String),
    Int(u64),
}

impl<'de> Deserialize<'de> for TransactionFilterValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let v = Value::deserialize(deserializer)?;
        let n = v.as_u64();
        if let Some(n) = n {
            Ok(Self::Int(n))
        } else {
            let n = v
                .as_str()
                .ok_or_else(|| D::Error::custom("Value must be number or string"))?;
            Ok(Self::String(n.to_string()))
        }
    }
}

impl Serialize for TransactionFilterValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TransactionFilterValue::String(v) => serializer.serialize_str(v),
            TransactionFilterValue::Int(v) => serializer.serialize_u64(*v),
        }
    }
}

pub const MAX_TX_FILTERS: usize = 32;

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionFilterItem {
    pub field: String,
    pub op: TransactionFilterOp,
    pub value: TransactionFilterValue,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// Describes possible interchain query types
pub enum QueryType {
    #[serde(rename = "kv")]
    /// **kv** is an interchain query type to query KV values from remote chain
    KV,

    /// **tx** is an interchain query type to query transactions from remote chain
    #[serde(rename = "tx")]
    TX,
}

#[allow(clippy::from_over_into)]
impl Into<String> for QueryType {
    fn into(self) -> String {
        match self {
            QueryType::KV => QUERY_TYPE_KV_VALUE.to_string(),
            QueryType::TX => QUERY_TYPE_TX_VALUE.to_string(),
        }
    }
}

/// Describes possible interchain query types with a payload
pub enum QueryPayload {
    /// **kv** is an interchain query type to query KV values from remote chain
    /// payload is kvkeys
    KV(Vec<KVKey>),

    /// **tx** is an interchain query type to query transactions from remote chain
    /// payload is transactions filter
    TX(Vec<TransactionFilterItem>),
}

/// Bytes representations of Bech32 address
pub type AddressBytes = Vec<u8>;

/// A **data structure** that can be reconstructed from slice of **StorageValue** structures.
/// Neutron provides `KVReconstruct` for many primitive and standard Cosmos-SDK types and query responses.
/// The complete list is [here][TODO_LINK]. All of these can be deserialized using Neutron out of the box.
///
/// Third-party projects may provide `KVReconstruct` implementations for types that they introduce.
/// For example if some query is not implemented in Neutron standard library, developers can create their own type/query and implement `KVReconstruct` for it.
///
///
/// Usually used together with `query_kv_result` function. For example, there is an Interchain Query with some `query_id` to query balance from remote chain.
/// And there is an implemented `KVReconstruct` for `Balance` structure.
/// So you can easily get reconstructed response for the query just in one line:
/// ```rust ignore
/// let balances: Balances = query_kv_result(deps, query_id)?;
/// ```
///
/// Anyone can implement `KVReconstruct` for any type and use `query_kv_result` without any problems.
pub trait KVReconstruct: Sized {
    /// Reconstructs this value from the slice of **StorageValue**'s.
    fn reconstruct(kvs: &[StorageValue]) -> NeutronResult<Self>;
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
                let amount = Uint128::from_str(pool_coin.amount.as_str())?;
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

#[cfg(test)]
mod tests {
    use crate::bindings::types::StorageValue;
    use crate::interchain_queries::helpers::{
        create_account_denom_balance_key, create_delegation_key, create_fee_pool_key,
        create_gov_proposal_key, create_params_store_key, create_total_denom_key,
        create_validator_key, decode_and_convert,
    };
    use crate::interchain_queries::types::{
        Balances, Delegations, FeePool, GovernmentProposal, KVReconstruct, Proposal,
        StakingValidator, TallyResult, TotalSupply, Validator as ContractValidator, KEY_BOND_DENOM,
        STAKING_STORE_KEY,
    };
    use crate::{NeutronError, NeutronResult};
    use cosmos_sdk_proto::cosmos::base::v1beta1::{Coin, DecCoin};
    use cosmos_sdk_proto::cosmos::distribution::v1beta1::FeePool as CosmosFeePool;
    use cosmos_sdk_proto::cosmos::gov::v1beta1::{
        Proposal as CosmosProposal, TallyResult as CosmosTallyResult,
    };
    use cosmos_sdk_proto::cosmos::staking::v1beta1::{
        Commission, CommissionRates, Delegation, Description, Validator,
    };
    use cosmwasm_std::{
        to_binary, Addr, Binary, Coin as StdCoin, Decimal, Delegation as StdDelegation, Uint128,
    };
    use prost::Message as ProstMessage;
    use std::str::FromStr;

    #[test]
    fn test_balance_reconstruct() {
        struct TestCase {
            addr: String,
            coins: Vec<(String, Uint128)>,
        }
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                addr: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
                coins: vec![("uosmo".to_string(), Uint128::from(100u128))],
            },
            TestCase {
                addr: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
                coins: vec![
                    ("uosmo".to_string(), Uint128::from(100u128)),
                    ("uatom".to_string(), Uint128::from(500u128)),
                    ("uluna".to_string(), Uint128::from(80u128)),
                ],
            },
            TestCase {
                addr: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
                coins: vec![],
            },
        ];

        for ts in test_cases {
            let mut st_values: Vec<StorageValue> = vec![];

            let converted_addr_bytes = decode_and_convert(ts.addr.as_str()).unwrap();
            for coin in &ts.coins {
                let balance_key =
                    create_account_denom_balance_key(converted_addr_bytes.clone(), &coin.0)
                        .unwrap();

                let balance_amount = Coin {
                    denom: coin.0.clone(),
                    amount: coin.1.to_string(),
                };
                let s = StorageValue {
                    storage_prefix: "".to_string(),
                    key: Binary(balance_key),
                    value: Binary(balance_amount.encode_to_vec()),
                };
                st_values.push(s);
            }

            let balances = Balances::reconstruct(&st_values).unwrap();
            assert_eq!(balances.coins.len(), ts.coins.len());
            for (i, coin) in balances.coins.iter().enumerate() {
                assert_eq!(coin.denom, ts.coins[i].0);
                assert_eq!(coin.amount, ts.coins[i].1)
            }
        }
    }

    #[test]
    fn test_bank_total_supply_reconstruct() {
        struct TestValue {
            denom: String,
            amount: String,
        }
        struct TestCase {
            values: Vec<TestValue>,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                values: vec![TestValue {
                    denom: "uatom".to_string(),
                    amount: "100".to_string(),
                }],
            },
            TestCase {
                values: vec![
                    TestValue {
                        denom: "uatom".to_string(),
                        amount: "100".to_string(),
                    },
                    TestValue {
                        denom: "uosmo".to_string(),
                        amount: "200".to_string(),
                    },
                ],
            },
            TestCase { values: vec![] },
        ];

        for ts in test_cases {
            let mut st_values: Vec<StorageValue> = vec![];

            for case in &ts.values {
                let denom_key = create_total_denom_key(case.denom.as_str()).unwrap();
                let s = StorageValue {
                    storage_prefix: "".to_string(),
                    key: Binary(denom_key),
                    value: Binary(case.amount.as_str().as_bytes().to_vec()),
                };
                st_values.push(s);
            }

            let total_supply = TotalSupply::reconstruct(&st_values).unwrap();
            assert_eq!(total_supply.coins.len(), ts.values.len());
            for (i, coin) in total_supply.coins.iter().enumerate() {
                assert_eq!(coin.denom, ts.values[i].denom);
                assert_eq!(
                    coin.amount,
                    Uint128::from_str(ts.values[i].amount.as_str()).unwrap()
                )
            }
        }
    }

    #[test]
    fn test_staking_validators_reconstruct() {
        struct TestCase {
            validators: Vec<Validator>,
            expected_result: NeutronResult<StakingValidator>,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                validators: vec![Validator {
                    operator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3"
                        .to_string(),
                    consensus_pubkey: None,
                    jailed: false,
                    status: 0,
                    tokens: "1000000000000000000".to_string(),
                    delegator_shares: "1000000000000000000".to_string(),
                    description: None,
                    unbonding_height: 0,
                    unbonding_time: None,
                    commission: None,
                    min_self_delegation: "".to_string(),
                }],
                expected_result: Ok(StakingValidator {
                    validators: vec![ContractValidator {
                        operator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3"
                            .to_string(),
                        status: 0,
                        tokens: "1000000000000000000".to_string(),
                        delegator_shares: "1000000000000000000".to_string(),
                        moniker: None,
                        identity: None,
                        website: None,
                        security_contact: None,
                        details: None,
                        unbonding_height: 0,
                        unbonding_time: None,
                        rate: None,
                        max_rate: None,
                        max_change_rate: None,
                        update_time: None,
                        min_self_delegation: Decimal::from_str("0").unwrap(),
                        jailed: false,
                    }],
                }),
            },
            TestCase {
                validators: vec![Validator {
                    operator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3"
                        .to_string(),
                    consensus_pubkey: None,
                    jailed: false,
                    status: 0,
                    tokens: "1000000000000000000".to_string(),
                    delegator_shares: "1000000000000000000".to_string(),
                    description: Some(Description {
                        moniker: "Test validator".to_string(),
                        identity: "JHFDHHFHF".to_string(),
                        website: "https://neutron.org".to_string(),
                        security_contact: "".to_string(),
                        details: "Validator details".to_string(),
                    }),
                    unbonding_height: 0,
                    unbonding_time: Some(prost_types::Timestamp {
                        seconds: 1203981203,
                        nanos: 123123,
                    }),
                    commission: Some(Commission {
                        commission_rates: Some(CommissionRates {
                            rate: "5".to_string(),
                            max_rate: "20".to_string(),
                            max_change_rate: "1".to_string(),
                        }),
                        update_time: Some(prost_types::Timestamp {
                            seconds: 56324234,
                            nanos: 1343,
                        }),
                    }),
                    min_self_delegation: "".to_string(),
                }],
                expected_result: Ok(StakingValidator {
                    validators: vec![ContractValidator {
                        operator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3"
                            .to_string(),
                        status: 0,
                        tokens: "1000000000000000000".to_string(),
                        delegator_shares: "1000000000000000000".to_string(),
                        moniker: Some("Test validator".to_string()),
                        identity: Some("JHFDHHFHF".to_string()),
                        website: Some("https://neutron.org".to_string()),
                        security_contact: Some("".to_string()),
                        details: Some("Validator details".to_string()),
                        unbonding_height: 0,
                        unbonding_time: Some(1203981203),
                        rate: Some(Decimal::from_str("5").unwrap()),
                        max_rate: Some(Decimal::from_str("20").unwrap()),
                        max_change_rate: Some(Decimal::from_str("1").unwrap()),
                        update_time: Some(56324234),
                        min_self_delegation: Decimal::from_str("0").unwrap(),
                        jailed: false,
                    }],
                }),
            },
            TestCase {
                validators: vec![
                    Validator {
                        operator_address: "cosmosvaloper132juzk0gdmwuxvx4phug7m3ymyatxlh9734g4w"
                            .to_string(),
                        consensus_pubkey: None,
                        jailed: false,
                        status: 0,
                        tokens: "1000000000000000000".to_string(),
                        delegator_shares: "1000000000000000000".to_string(),
                        description: None,
                        unbonding_height: 0,
                        unbonding_time: None,
                        commission: None,
                        min_self_delegation: "".to_string(),
                    },
                    Validator {
                        operator_address: "cosmosvaloper1sjllsnramtg3ewxqwwrwjxfgc4n4ef9u2lcnj0"
                            .to_string(),
                        consensus_pubkey: None,
                        jailed: false,
                        status: 0,
                        tokens: "2000000000000000000".to_string(),
                        delegator_shares: "3000000000000000000".to_string(),
                        description: None,
                        unbonding_height: 0,
                        unbonding_time: None,
                        commission: None,
                        min_self_delegation: "".to_string(),
                    },
                ],
                expected_result: Ok(StakingValidator {
                    validators: vec![
                        ContractValidator {
                            operator_address:
                                "cosmosvaloper132juzk0gdmwuxvx4phug7m3ymyatxlh9734g4w".to_string(),
                            status: 0,
                            tokens: "1000000000000000000".to_string(),
                            delegator_shares: "1000000000000000000".to_string(),
                            moniker: None,
                            identity: None,
                            website: None,
                            security_contact: None,
                            details: None,
                            unbonding_height: 0,
                            unbonding_time: None,
                            rate: None,
                            max_rate: None,
                            max_change_rate: None,
                            update_time: None,
                            min_self_delegation: Decimal::from_str("0").unwrap(),
                            jailed: false,
                        },
                        ContractValidator {
                            operator_address:
                                "cosmosvaloper1sjllsnramtg3ewxqwwrwjxfgc4n4ef9u2lcnj0".to_string(),
                            status: 0,
                            tokens: "2000000000000000000".to_string(),
                            delegator_shares: "3000000000000000000".to_string(),
                            moniker: None,
                            identity: None,
                            website: None,
                            security_contact: None,
                            details: None,
                            unbonding_height: 0,
                            unbonding_time: None,
                            rate: None,
                            max_rate: None,
                            max_change_rate: None,
                            update_time: None,
                            min_self_delegation: Decimal::from_str("0").unwrap(),
                            jailed: false,
                        },
                    ],
                }),
            },
            TestCase {
                validators: vec![],
                expected_result: Ok(StakingValidator { validators: vec![] }),
            },
        ];

        for ts in test_cases {
            let mut st_values: Vec<StorageValue> = vec![];

            for validator in &ts.validators {
                let val_addr = decode_and_convert(validator.operator_address.as_str()).unwrap();

                let validator_key = create_validator_key(&val_addr).unwrap();
                let s = StorageValue {
                    storage_prefix: "".to_string(),
                    key: Binary(validator_key),
                    value: Binary(validator.encode_to_vec()),
                };
                st_values.push(s);
            }

            let stakin_validator = StakingValidator::reconstruct(&st_values);

            assert_eq!(stakin_validator, ts.expected_result)
        }
    }

    #[test]
    fn test_government_proposals_reconstruct() {
        struct TestCase {
            proposals: Vec<CosmosProposal>,
            expected_result: NeutronResult<GovernmentProposal>,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                proposals: vec![CosmosProposal {
                    proposal_id: 1,
                    content: Some(prost_types::Any {
                        type_url: "proposal_type".to_string(),
                        value: vec![],
                    }),
                    status: 1,
                    final_tally_result: None,
                    submit_time: None,
                    deposit_end_time: None,
                    total_deposit: vec![Coin {
                        amount: "100000".to_string(),
                        denom: "stake".to_string(),
                    }],
                    voting_start_time: None,
                    voting_end_time: None,
                }],
                expected_result: Ok(GovernmentProposal {
                    proposals: vec![Proposal {
                        proposal_id: 1,
                        proposal_type: Some("proposal_type".to_string()),
                        total_deposit: vec![StdCoin::new(100000u128, "stake")],
                        status: 1,
                        submit_time: None,
                        deposit_end_time: None,
                        voting_start_time: None,
                        voting_end_time: None,
                        final_tally_result: None,
                    }],
                }),
            },
            TestCase {
                proposals: vec![CosmosProposal {
                    proposal_id: 1,
                    content: Some(prost_types::Any {
                        type_url: "proposal_type".to_string(),
                        value: vec![],
                    }),
                    status: 1,
                    final_tally_result: Some(CosmosTallyResult {
                        abstain: "1".to_string(),
                        no: "2".to_string(),
                        no_with_veto: "3".to_string(),
                        yes: "4".to_string(),
                    }),
                    submit_time: Some(prost_types::Timestamp {
                        seconds: 2222222,
                        nanos: 123123,
                    }),
                    deposit_end_time: Some(prost_types::Timestamp {
                        seconds: 3333333,
                        nanos: 123123,
                    }),
                    total_deposit: vec![Coin {
                        amount: "100000".to_string(),
                        denom: "stake".to_string(),
                    }],
                    voting_start_time: Some(prost_types::Timestamp {
                        seconds: 4444444,
                        nanos: 123123,
                    }),
                    voting_end_time: Some(prost_types::Timestamp {
                        seconds: 555555555,
                        nanos: 123123,
                    }),
                }],
                expected_result: Ok(GovernmentProposal {
                    proposals: vec![Proposal {
                        proposal_id: 1,
                        proposal_type: Some("proposal_type".to_string()),
                        total_deposit: vec![StdCoin::new(100000u128, "stake")],
                        status: 1,
                        submit_time: Some(2222222),
                        deposit_end_time: Some(3333333),
                        voting_start_time: Some(4444444),
                        voting_end_time: Some(555555555),
                        final_tally_result: Some(TallyResult {
                            abstain: "1".to_string(),
                            no: "2".to_string(),
                            no_with_veto: "3".to_string(),
                            yes: "4".to_string(),
                        }),
                    }],
                }),
            },
            TestCase {
                proposals: vec![
                    CosmosProposal {
                        proposal_id: 1,
                        content: Some(prost_types::Any {
                            type_url: "proposal_type1".to_string(),
                            value: vec![],
                        }),
                        status: 1,
                        final_tally_result: None,
                        submit_time: None,
                        deposit_end_time: None,
                        total_deposit: vec![Coin {
                            amount: "100000".to_string(),
                            denom: "stake".to_string(),
                        }],
                        voting_start_time: None,
                        voting_end_time: None,
                    },
                    CosmosProposal {
                        proposal_id: 2,
                        content: Some(prost_types::Any {
                            type_url: "proposal_type2".to_string(),
                            value: vec![],
                        }),
                        status: 1,
                        final_tally_result: None,
                        submit_time: None,
                        deposit_end_time: None,
                        total_deposit: vec![Coin {
                            amount: "200000".to_string(),
                            denom: "osmo".to_string(),
                        }],
                        voting_start_time: None,
                        voting_end_time: None,
                    },
                ],
                expected_result: Ok(GovernmentProposal {
                    proposals: vec![
                        Proposal {
                            proposal_id: 1,
                            proposal_type: Some("proposal_type1".to_string()),
                            total_deposit: vec![StdCoin::new(100000u128, "stake")],
                            status: 1,
                            submit_time: None,
                            deposit_end_time: None,
                            voting_start_time: None,
                            voting_end_time: None,
                            final_tally_result: None,
                        },
                        Proposal {
                            proposal_id: 2,
                            proposal_type: Some("proposal_type2".to_string()),
                            total_deposit: vec![StdCoin::new(200000u128, "osmo")],
                            status: 1,
                            submit_time: None,
                            deposit_end_time: None,
                            voting_start_time: None,
                            voting_end_time: None,
                            final_tally_result: None,
                        },
                    ],
                }),
            },
            TestCase {
                proposals: vec![],
                expected_result: Ok(GovernmentProposal { proposals: vec![] }),
            },
        ];

        for ts in test_cases {
            let mut st_values: Vec<StorageValue> = vec![];

            for proposal in &ts.proposals {
                let proposal_key = create_gov_proposal_key(proposal.proposal_id).unwrap();
                let s = StorageValue {
                    storage_prefix: "".to_string(),
                    key: Binary(proposal_key),
                    value: Binary(proposal.encode_to_vec()),
                };
                st_values.push(s);
            }

            let gov_proposal = GovernmentProposal::reconstruct(&st_values);

            assert_eq!(gov_proposal, ts.expected_result)
        }
    }

    #[test]
    fn test_fee_pool_reconstruct() {
        struct TestCase {
            coins: Vec<(String, Uint128)>,
        }
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                coins: vec![("uosmo".to_string(), Uint128::from(100u128))],
            },
            TestCase {
                coins: vec![
                    ("uosmo".to_string(), Uint128::from(100u128)),
                    ("uatom".to_string(), Uint128::from(500u128)),
                    ("uluna".to_string(), Uint128::from(80u128)),
                ],
            },
            TestCase { coins: vec![] },
        ];

        for ts in test_cases {
            let mut coins: Vec<DecCoin> = vec![];

            for coin in &ts.coins {
                let balance_amount = DecCoin {
                    denom: coin.0.clone(),
                    amount: coin.1.to_string(),
                };

                coins.push(balance_amount);
            }

            let fee_pool = CosmosFeePool {
                community_pool: coins,
            };

            let fee_pool_key = create_fee_pool_key().unwrap();

            let st_value = StorageValue {
                storage_prefix: "".to_string(),
                key: Binary(fee_pool_key),
                value: Binary(fee_pool.encode_to_vec()),
            };

            let fee_pool_coins = FeePool::reconstruct(&[st_value]).unwrap();
            assert_eq!(fee_pool_coins.coins.len(), ts.coins.len());
            for (i, coin) in fee_pool_coins.coins.iter().enumerate() {
                assert_eq!(coin.denom, ts.coins[i].0);
                assert_eq!(coin.amount, ts.coins[i].1)
            }
        }
    }

    #[test]
    fn test_delegations_reconstruct() {
        struct TestCase {
            stake_denom: String,
            delegations: Vec<Delegation>,
            validators: Vec<Validator>,
            expected_result: NeutronResult<Delegations>,
        }
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                stake_denom: "stake".to_string(),
                delegations: vec![Delegation {
                    delegator_address: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
                    validator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3"
                        .to_string(),
                    shares: "1000000000000000000".to_string(),
                }],
                validators: vec![Validator {
                    operator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3"
                        .to_string(),
                    consensus_pubkey: None,
                    jailed: false,
                    status: 0,
                    tokens: "1000000000000000000".to_string(),
                    delegator_shares: "1000000000000000000".to_string(),
                    description: None,
                    unbonding_height: 0,
                    unbonding_time: None,
                    commission: None,
                    min_self_delegation: "".to_string(),
                }],
                expected_result: Ok(Delegations {
                    delegations: vec![StdDelegation {
                        delegator: Addr::unchecked("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"),
                        validator: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3".to_string(),
                        amount: StdCoin::new(1000000000000000000u128, "stake"),
                    }],
                }),
            },
            TestCase {
                stake_denom: "stake".to_string(),
                delegations: vec![
                    Delegation {
                        delegator_address: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"
                            .to_string(),
                        validator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3"
                            .to_string(),
                        shares: "1000000000000000000".to_string(),
                    },
                    Delegation {
                        delegator_address: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"
                            .to_string(),
                        validator_address: "osmovaloper1lzhlnpahvznwfv4jmay2tgaha5kmz5qxwmj9we"
                            .to_string(),
                        shares: "1000000000000000000".to_string(),
                    },
                ],
                validators: vec![
                    Validator {
                        operator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3"
                            .to_string(),
                        consensus_pubkey: None,
                        jailed: false,
                        status: 0,
                        tokens: "1000000000000000000".to_string(),
                        delegator_shares: "1000000000000000000".to_string(),
                        description: None,
                        unbonding_height: 0,
                        unbonding_time: None,
                        commission: None,
                        min_self_delegation: "".to_string(),
                    },
                    Validator {
                        operator_address: "osmovaloper1lzhlnpahvznwfv4jmay2tgaha5kmz5qxwmj9we"
                            .to_string(),
                        consensus_pubkey: None,
                        jailed: false,
                        status: 0,
                        tokens: "1000000000000000000".to_string(),
                        delegator_shares: "1000000000000000000".to_string(),
                        description: None,
                        unbonding_height: 0,
                        unbonding_time: None,
                        commission: None,
                        min_self_delegation: "".to_string(),
                    },
                ],
                expected_result: Ok(Delegations {
                    delegations: vec![
                        StdDelegation {
                            delegator: Addr::unchecked(
                                "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs",
                            ),
                            validator: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3"
                                .to_string(),
                            amount: StdCoin::new(1000000000000000000u128, "stake"),
                        },
                        StdDelegation {
                            delegator: Addr::unchecked(
                                "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs",
                            ),
                            validator: "osmovaloper1lzhlnpahvznwfv4jmay2tgaha5kmz5qxwmj9we"
                                .to_string(),
                            amount: StdCoin::new(1000000000000000000u128, "stake"),
                        },
                    ],
                }),
            },
            TestCase {
                stake_denom: "stake".to_string(),
                delegations: vec![],
                validators: vec![],
                expected_result: Ok(Delegations {
                    delegations: vec![],
                }),
            },
            TestCase {
                stake_denom: Default::default(),
                delegations: vec![],
                validators: vec![],
                expected_result: Err(NeutronError::InvalidQueryResultFormat(
                    "denom is empty".into(),
                )),
            },
            TestCase {
                stake_denom: "stake".to_string(),
                delegations: vec![Delegation {
                    delegator_address: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
                    validator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3"
                        .to_string(),
                    shares: "1000000000000000000".to_string(),
                }],
                validators: vec![],
                expected_result: Err(NeutronError::InvalidQueryResultFormat(
                    "validator is empty".into(),
                )),
            },
        ];

        for ts in &test_cases {
            // prepare storage values
            let mut st_values: Vec<StorageValue> = vec![StorageValue {
                storage_prefix: STAKING_STORE_KEY.to_string(),
                key: Binary(create_params_store_key(STAKING_STORE_KEY, KEY_BOND_DENOM)),
                value: {
                    if ts.stake_denom.is_empty() {
                        return Default::default();
                    }
                    to_binary(&ts.stake_denom).unwrap()
                },
            }];

            for (i, d) in ts.delegations.iter().enumerate() {
                let delegator_addr = decode_and_convert(&d.delegator_address).unwrap();
                let val_addr = decode_and_convert(&d.validator_address).unwrap();

                st_values.push(StorageValue {
                    storage_prefix: STAKING_STORE_KEY.to_string(),
                    key: Binary(create_delegation_key(&delegator_addr, &val_addr).unwrap()),
                    value: Binary::from(d.encode_to_vec()),
                });

                if let Some(v) = ts.validators.get(i) {
                    st_values.push(StorageValue {
                        storage_prefix: STAKING_STORE_KEY.to_string(),
                        key: Binary(create_validator_key(&val_addr).unwrap()),
                        value: Binary::from(v.encode_to_vec()),
                    });
                }
            }

            // test reconstruction
            let delegations = Delegations::reconstruct(&st_values);

            assert_eq!(delegations, ts.expected_result)
        }
    }
}
