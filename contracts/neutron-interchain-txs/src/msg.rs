use cosmwasm_std::{Binary, CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub connection_id: String,
    pub interchain_account_id: String,
    pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Delegate {
        delegator: String,
        validator: String,
        amount: u128,
    },
    Undelegate {
        delegator: String,
        validator: String,
        amount: u128,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MsgSubmitTx {
    pub from_address: String,
    pub owner: String,
    pub connection_id: String,
    pub msgs: Vec<Binary>,
    pub memo: String,
}

impl From<MsgSubmitTx> for CosmosMsg<MsgSubmitTx> {
    fn from(original: MsgSubmitTx) -> Self {
        CosmosMsg::Custom(original)
    }
}

impl CustomMsg for MsgSubmitTx {}
