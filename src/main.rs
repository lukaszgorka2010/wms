#![allow(dead_code)]
use confy;
use serde::{Serialize, Deserialize};
use std::io;

mod orderfilling;
mod pallet;
mod slots;
mod storage;

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
    let config: Config = confy::load("wms", "cfg")?;
    let mut storage = storage::Storage::new(config.depot_id);
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
                println!("Pallet id\t\tSKU\t\tQty\tSlot\tStatus");
                for (id, pallet) in storage.pallets() {
                    println!("{}\t{}\t\t{}\t{}\t{}",
                        id, pallet.sku(), pallet.quantity(),pallet.slot_name(),pallet.status());
                }
            },
            "exit" | "q" | "quit" => {
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
