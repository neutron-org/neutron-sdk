use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type Recipient = str;

/// contains all transfers mapped by a recipient address observed by the contract.
pub const RECIPIENT_TXS: Map<&Recipient, Vec<NftTransfer>> = Map::new("recipient_txs");
/// contains number of transfers to addresses observed by the contract.
pub const TRANSFERS: Item<u64> = Item::new("transfers");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct NftTransfer {
    pub recipient: String,
    pub sender: String,
    pub contract_address: String,
    pub token_id: String,
}
