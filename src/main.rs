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
    did: u64,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self { did: 00000 }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let _confg: Config = confy::load("wms", "cfg")?;
Ok(())
}
