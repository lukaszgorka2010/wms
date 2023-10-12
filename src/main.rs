#![allow(dead_code)]
// Imports
//use std::{collections::HashMap};

//use orderfilling::Order;
//use slots::Slots;
//use storage::Storage;
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
    /*let order= Order {
        order_number: 1234547891u64,
        depot_id: 2137,
        order_lines: {
            let mut map: HashMap<String, u32> = HashMap::new();
            map.insert(12345678.to_string(), 236);
            map.insert(12457894.to_string(), 12);
            map
        }
    };
    let json = serde_json::to_string_pretty(&order);
    print!("{:#?}", order);
    match json {
        Ok(son) => print!("{:?}", son),
        Err(_) => println!("Serialisation error"),
    }*/
/*let mut stock = Storage::default();
for _ in 0..3 {
    stock.add_pallet(123456, 125);
}

let mut slots = Slots::default();
for i in 0..10 {
    let slot: String = format!("A203{}", i);
    slots.add_slot(slot);
}

for _ in 0..5 {
    stock.receiving(&mut slots, 21372137, 45);
}
println!("Slots:\n{:#?}", slots);

println!("Stock\n{:#?}",stock);
*/
Ok(())
}
