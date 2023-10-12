#![allow(dead_code)]
// Imports
use serde::{Serialize, Deserialize};

mod orderfilling;
mod pallet;
mod slots;
mod storage;

// Unique depot identifier, should stored in a config file.
#[derive(Serialize, Deserialize)]
struct Config {
    depot_id: u64,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self { depot_id: 99999}
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let _confg: Config = confy::load("wms", "cfg")?;
Ok(())
}
