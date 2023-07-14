use crate::state::NftTransfer;
use cosmwasm_std::Uint128;
use neutron_sdk::bindings::types::KVKey;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RegisterTransferNftQuery {
        connection_id: String,
        update_period: u64,
        min_height: u64,
        recipient: String,
        sender: String,
        contract_address: String,
        token_id: String,
    },
    RemoveInterchainQuery {
        query_id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    TransferNft { query_id: u64 },
    GetRegisteredQuery { query_id: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetRecipientTxsResponse {
    pub transfers: Vec<NftTransfer>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TransferNftResponse {
    pub sender: String,
    pub token_id: String,
    pub contract_address: String,
}

