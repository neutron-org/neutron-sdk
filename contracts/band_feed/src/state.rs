use cw_storage_plus::Item;
use neutron_sdk::bindings::msg::IbcFee;

pub const PRICE: Item<IbcFee> = Item::new("price");
