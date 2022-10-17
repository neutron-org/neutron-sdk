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
    RegisterTransfersQuery {
        connection_id: String,
        update_period: u64,
        recipient: String,
        min_height: Option<u128>,
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
    },
    RemoveInterchainQuery {
        query_id: u64,
    },
    /// Used only in integration tests framework to simulate failures.
    IntegrationTestsRegisterQueryEmptyKeys {
        connection_id: String,
    },
    /// Used only in integration tests framework to simulate failures.
    IntegrationTestsRegisterQueryEmptyPath {
        connection_id: String,
    },
    /// Used only in integration tests framework to simulate failures.
    IntegrationTestsRegisterQueryEmptyId {
        connection_id: String,
    },
    /// Used only in integration tests framework to simulate failures.
    /// After executing this message, contract will attempt to alter state,
    /// zero out kv query statistics and then fail, all of this happening
    /// in sudo kv callback handler.
    IntegrationTestsSetKvQueryMock {},
    /// Used only in integration tests framework to simulate failures.
    /// After executing this message, contract will revert back to normal behaviour.
    IntegrationTestsUnsetKvQueryMock {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance { query_id: u64 },
    GetDelegations { query_id: u64 },
    GetRegisteredQuery { query_id: u64 },
    GetRecipientTxs { recipient: String },
    KvCallbackStats { query_id: u64 },
    GetTransfersAmount {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetRecipientTxsResponse {
    pub transfers: Vec<Transfer>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetTransfersAmountResponse {
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct KvCallbackStatsResponse {
    pub last_update_height: u64,
}
