use cosmwasm_std::{Binary, CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct OpenAckVersion {
    pub version: String,
    pub controller_connection_id: String,
    pub host_connection_id: String,
    pub address: String,
    pub encoding: String,
    pub tx_type: String,
}
