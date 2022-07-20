use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RegisterBalanceQuery {
        zone_id: String,
        connection_id: String,
        update_period: u64,
        addr: String,
        denom: String,
    },
    RegisterTransfersQuery {
        zone_id: String,
        connection_id: String,
        update_period: u64,
        recipient: String,
    },
    RegisterDelegatorDelegationsQuery {
        delegator: String,
        zone_id: String,
        connection_id: String,
        update_period: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance {
        zone_id: String,
        addr: String,
        denom: String,
    },
    GetTransfers {
        zone_id: String,
        recipient: String,
        start: u64,
        limit: u64,
    },
    GetDelegations {
        zone_id: String,
        delegator: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
