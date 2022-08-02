use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
/// Type for wrapping any protobuf message
pub struct ProtobufAny {
    /// **type_url** describes the type of the serialized message
    pub type_url: String,

    ///  **value** must be a valid serialized protocol buffer of the above specified type
    pub value: Vec<u8>,
}

impl ProtobufAny {
    /// Helper to create new ProtobufAny type:
    /// * **type_url** describes the type of the serialized message
    /// * **value** must be a valid serialized protocol buffer of the above specified type
    pub fn new(type_url: String, value: Vec<u8>) -> Self {
        ProtobufAny { type_url, value }
    }
}
