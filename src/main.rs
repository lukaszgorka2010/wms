#![allow(dead_code)]

use storage::Storage;

use crate::slots::Slots;

mod orderfilling;
mod pallet;
mod slots;
mod storage;

const DID: u32 = 2137;

fn main() {
let mut stock = Storage::default();
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
}
