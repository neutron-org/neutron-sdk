use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type Recipient = str;

pub const RECIPIENT_TXS: Map<&Recipient, Vec<Transfer>> = Map::new("recipient_txs");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Transfer {
    pub recipient: String,
    pub sender: String,
    pub denom: String,
    pub amount: String,
}

pub const INTEGRATION_TESTS_KV_MOCK: Item<IntegrationTestsKvMock> =
    Item::new("integration_tests_kv_mock");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum IntegrationTestsKvMock {
    Enabled,
    Disabled,
}

pub const KV_CALLBACK_STATS: Map<u64, u64> = Map::new("kv_callback_stats");
