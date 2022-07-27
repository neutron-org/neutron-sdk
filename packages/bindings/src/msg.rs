use crate::types::ProtobufAny;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum NeutronMsg {
    RegisterInterchainAccount {
        connection_id: String,
        interchain_account_id: String,
    },

    SubmitTX {
        connection_id: String,
        interchain_account_id: String,
        msgs: Vec<ProtobufAny>,
        memo: String,
    },

    RegisterInterchainQuery {
        query_type: String,
        query_data: String,
        zone_id: String,
        connection_id: String,
        update_period: u64,
    },
}
