use cw_storage_plus::Item;

pub const PRICES: Item<(String, Option<Vec<u64>>)> = Item::new("price");
pub const CHANNEL: Item<String> = Item::new("oracle-channel");
