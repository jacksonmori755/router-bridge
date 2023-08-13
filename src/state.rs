use cw_storage_plus::Item;

pub const NONCE: Item<u64> = Item::new("nonce");
