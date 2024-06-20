use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub b: u32,
    pub e: u32,
}

pub const CONFIG: Item<Config> = Item::new("config");