use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Ica {
        interchain_account_id: String,
        connection_id: String,
    },
    IcaContract {
        interchain_account_id: String,
    },
    AcknowledgementResult {
        interchain_account_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Register {
        connection_id: String,
        interchain_account_id: String,
    },
    Delegate {
        interchain_account_id: String,
        validator: String,
        amount: u128,
    },
    Undelegate {
        interchain_account_id: String,
        validator: String,
        amount: u128,
    },
}

/// Serves for storing acknowledgement calls for interchain transactions
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AcknowledgementResult {
    /// Ack - Got success acknowledgement in sudo with array of message item types in it
    Ack(Vec<String>),
    /// Error - Got error acknowledgement in sudo with payload message in it
    Error(String),
    /// Timeout - Got timeout acknowledgement in sudo with payload message in it
    Timeout(String),
}
