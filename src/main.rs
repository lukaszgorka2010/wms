#![allow(dead_code)]

use storage::Storage;

mod pallet;
mod slots;
mod storage;

const DID: u64 = 2137;

fn main() {
let mut stock = Storage::default();

stock.add_pallet(123456, 125);

println!("{:?}",stock);
}
