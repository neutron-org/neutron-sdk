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
    RegisterICA {},
    #[cfg_attr(feature = "interface", payable)]
    MintNft {
        token_id: String,
    },
    #[cfg_attr(feature = "interface", payable)]
    RegisterTransferNftQuery {
        min_height: u64,
        sender: String,
        token_id: String,
    },
    RemoveInterchainQuery {
        query_id: u64,
    },
    UnlockNft {
        token_id: String,
        destination: String,
    },
    UpdateConfig {
        update_period: Option<u64>,
        nft_contract_address: Option<String>,
    },
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReplyMsg {
    RegisteredToken { token_id: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "interface", derive(cw_orch::QueryFns))]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(String)]
    IcaAccount {},
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
