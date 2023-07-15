use neutron_sdk::bindings::query::QueryRegisteredQueryResponse;

use crate::state::NftTransfer;

use cosmwasm_schema::QueryResponses;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub connection_id: String,
    pub contract_addr: String, // This is a stargaze address, so it should NOT be validated locally
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))]
pub enum ExecuteMsg {
    RegisterICA,
    MintNft {
        token_id: String,
    },
    RegisterTransferNftQuery {
        update_period: u64,
        min_height: u64,
        sender: String,
        token_id: String,
        ica_account: String,
        connection_id: String,
    },
    RemoveInterchainQuery {
        query_id: u64,
    },
    UnlockNft {
        token_id: String,
        destination: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(NftTransfersResponse)]
    NftTransfers { sender: String },
    #[returns(QueryRegisteredQueryResponse)]
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
pub struct NftTransfersResponse {
    pub transfers: Vec<NftTransfer>,
}
