use crate::state::Transfer;
use cosmwasm_std::Uint128;
use neutron_std::types::neutron::interchainqueries::KvKey;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RegisterBalancesQuery {
        connection_id: String,
        update_period: u64,
        addr: String,
        denoms: Vec<String>,
    },
    RegisterBankTotalSupplyQuery {
        connection_id: String,
        update_period: u64,
        denoms: Vec<String>,
    },
    RegisterDistributionFeePoolQuery {
        connection_id: String,
        update_period: u64,
    },
    RegisterStakingValidatorsQuery {
        connection_id: String,
        update_period: u64,
        validators: Vec<String>,
    },
    RegisterValidatorsSigningInfosQuery {
        connection_id: String,
        update_period: u64,
        validators: Vec<String>,
    },
    RegisterGovernmentProposalsQuery {
        connection_id: String,
        proposals_ids: Vec<u64>,
        update_period: u64,
    },
    RegisterTransfersQuery {
        connection_id: String,
        update_period: u64,
        recipient: String,
        min_height: Option<u64>,
    },
    RegisterDelegatorDelegationsQuery {
        delegator: String,
        validators: Vec<String>,
        connection_id: String,
        update_period: u64,
    },
    RegisterDelegatorUnbondingDelegationsQuery {
        delegator: String,
        validators: Vec<String>,
        connection_id: String,
        update_period: u64,
    },
    RegisterCw20BalanceQuery {
        connection_id: String,
        update_period: u64,
        cw20_contract_address: String,
        account_address: String,
    },
    UpdateInterchainQuery {
        query_id: u64,
        new_keys: Vec<KvKey>,
        new_update_period: u64,
        new_recipient: Option<String>,
    },
    RemoveInterchainQuery {
        query_id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance { query_id: u64 },
    BankTotalSupply { query_id: u64 },
    DistributionFeePool { query_id: u64 },
    StakingValidators { query_id: u64 },
    ValidatorsSigningInfos { query_id: u64 },
    GovernmentProposals { query_id: u64 },
    GetDelegations { query_id: u64 },
    GetUnbondingDelegations { query_id: u64 },
    Cw20Balance { query_id: u64 },
    GetRegisteredQuery { query_id: u64 },
    GetRecipientTxs { recipient: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Cw20BalanceResponse {
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetRecipientTxsResponse {
    pub transfers: Vec<Transfer>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MigrateMsg {}
