use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type Recipient = str;

/// contains all transfers mapped by a recipient address observed by the contract.
pub const RECIPIENT_TXS: Map<&Recipient, Vec<Transfer>> = Map::new("recipient_txs");
/// contains number of transfers to addresses observed by the contract.
pub const TRANSFERS: Item<u64> = Item::new("transfers");
/// contains number of fake loops for tx query result to consume more gas
pub const FAKE_LOOP_LIMIT: Item<u64> = Item::new("fake_loop_limit");

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
