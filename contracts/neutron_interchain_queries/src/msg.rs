use crate::state::Transfer;
use neutron_sdk::bindings::types::KVKey;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RegisterBalanceQuery {
        connection_id: String,
        update_period: u64,
        addr: String,
        denom: String,
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
    UpdateInterchainQuery {
        query_id: u64,
        new_keys: Option<Vec<KVKey>>,
        new_update_period: Option<u64>,
        new_recipient: Option<String>,
    },
    RemoveInterchainQuery {
        query_id: u64,
    },
    /// Used only in integration tests framework to simulate failures. It tries to register query where keys is an empty array.
    IntegrationTestsRegisterQueryEmptyKeys {
        connection_id: String,
    },
    /// Used only in integration tests framework to simulate failures. It tries to register query where in keys array there is a key with empty path.
    IntegrationTestsRegisterQueryEmptyPath {
        connection_id: String,
    },
    /// Used only in integration tests framework to simulate failures. It tries to register query where in keys array there is a key with empty id.
    IntegrationTestsRegisterQueryEmptyId {
        connection_id: String,
    },
    /// Used only in integration tests framework to simulate failures.
    /// After executing this message, contract will attempt to alter state,
    /// zero out kv query statistics and then fail, all of this happening
    /// in sudo kv callback handler.
    IntegrationTestsSetQueryMock {},
    /// Used only in integration tests framework to simulate failures.
    /// After executing this message, contract will revert back to normal behaviour.
    IntegrationTestsUnsetQueryMock {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance { query_id: u64 },
    BankTotalSupply { query_id: u64 },
    DistributionFeePool { query_id: u64 },
    StakingValidators { query_id: u64 },
    GovernmentProposals { query_id: u64 },
    GetDelegations { query_id: u64 },
    GetRegisteredQuery { query_id: u64 },
    GetRecipientTxs { recipient: String },
    KvCallbackStats { query_id: u64 },
    GetTransfersNumber {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetRecipientTxsResponse {
    pub transfers: Vec<Transfer>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetTransfersAmountResponse {
    pub transfers_number: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct KvCallbackStatsResponse {
    pub last_update_height: u64,
}
