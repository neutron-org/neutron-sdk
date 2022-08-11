use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RequestPacketTimeoutHeight {
    pub revision_number: Option<u64>,
    pub revision_height: Option<u64>,
}
