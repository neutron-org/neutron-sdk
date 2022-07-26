use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::num::ParseIntError;

const PATH_KEY_DELIMITER: &str = "/";
const KV_KEYS_DELIMITER: &str = ",";

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push_str(&format!("{:02x}", b));
    }

    s
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct KVKey {
    pub path: String,
    pub key: Vec<u8>,
}

impl KVKey {
    pub fn new(path: String, key: Vec<u8>) -> Self {
        KVKey { path, key }
    }

    pub fn from_string(s: &str) -> Result<Self, ParseIntError> {
        let kv: Vec<String> = s.split(PATH_KEY_DELIMITER).map(String::from).collect();
        if kv.len() < 2 {}

        Ok(KVKey {
            path: kv[0].clone(),
            key: decode_hex(kv[1].as_str())?,
        })
    }
}

impl Into<String> for &KVKey {
    fn into(self) -> String {
        let mut s = String::with_capacity(self.path.len() + 1 + self.key.len() * 2);
        s.push_str(&self.path);
        s.push_str(PATH_KEY_DELIMITER);
        s.push_str(&encode_hex(&self.key));

        s
    }
}

pub struct KVKeys {
    keys: Vec<KVKey>,
}

impl Into<String> for KVKeys {
    fn into(self) -> String {
        self.keys
            .iter()
            .map(|kv| kv.into())
            .collect::<Vec<String>>()
            .join(KV_KEYS_DELIMITER)
    }
}

const QUERY_TYPE_KV_VALUE: &str = "kv";
const QUERY_TYPE_TX_VALUE: &str = "tx";

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, JsonSchema)]
pub enum InterchainQueryType {
    #[serde(rename = "kv")]
    KV,

    #[serde(rename = "tx")]
    TX,
}

impl InterchainQueryType {
    pub fn try_from_str(s: &str) -> Option<InterchainQueryType> {
        match s {
            QUERY_TYPE_KV_VALUE => Some(InterchainQueryType::KV),
            QUERY_TYPE_TX_VALUE => Some(InterchainQueryType::TX),
            _ => None,
        }
    }
}

impl Into<String> for InterchainQueryType {
    fn into(self) -> String {
        match self {
            InterchainQueryType::KV => QUERY_TYPE_KV_VALUE.to_string(),
            InterchainQueryType::TX => QUERY_TYPE_TX_VALUE.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ProtobufAny {
    pub type_url: String,
    ///  Must be a valid serialized protocol buffer of the above specified type.
    pub value: Vec<u8>,
}
