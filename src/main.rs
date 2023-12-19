#![allow(dead_code)]
// Imports
use confy;
use serde::{Serialize, Deserialize};
use std::io;

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
    let confg: Config = confy::load("wms", "cfg")?;
    let mut storage = storage::Storage::new(confg.depot_id);
    let mut input = String::new();
    
    loop {
        println!("Enter command");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        println!("{}",input);
        match input.as_str().trim() {
            "addpal" => {
                println!("Adding pallet to storage. Enter SKU:");
                let mut inp = String::new();
                io::stdin().read_line(&mut inp).unwrap();
                //println!("{inp}");
                let sku: u32 = inp.trim().parse().unwrap();
                
                println!("Enter quantity");
                let mut inp = String::new();
                io::stdin().read_line(&mut inp).unwrap();
                let quantity: u16 = inp.trim().parse().unwrap();
                storage.add_pallet(sku, quantity);
            },
            "prnstg" => {
                println!("{:#?}", storage);
            },
            "exit" => {
                println!("Program closing");
                break;
            },
            _ => {
                println!("Not a valid command")
            },
        }
    }
    
Ok(())
}
