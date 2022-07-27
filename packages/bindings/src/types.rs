use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ProtobufAny {
    pub type_url: String,
    ///  Must be a valid serialized protocol buffer of the above specified type.
    pub value: Vec<u8>,
}
