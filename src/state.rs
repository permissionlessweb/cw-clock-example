use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub val: u32,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const MOCK_DATA: Item<Vec<u8>> = Item::new("mock_data");
