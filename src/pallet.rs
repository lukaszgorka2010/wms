#[derive(Debug)]
pub struct Pallet {
    id: u64,
    sku: u32,
    quantity: u16,
    slot: String,
}
impl Pallet {
    pub fn update_quantity(&mut self, new_quantity: u16) {
        self.quantity = new_quantity;
    }
    pub fn update_slot(&mut self, new_slot: String) {
        self.slot = new_slot;
    }
}
pub fn add(id: u64, sku: u32, quantity: u16, slot: String) -> Pallet {
    Pallet {
        id,
        sku,
        quantity,
        slot,
    }
}