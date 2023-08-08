use crate::slots::Slot;

#[derive(Debug)]
pub struct Pallet {
    id: u64,
    sku: u32,
    quantity: u16,
    slot: Option<Slot>,
    status: Status,
}

#[derive(Debug)]
enum Status {
    Available,
    AwaitingPutaway(Slot),
}
impl Pallet {
    pub fn update_quantity(&mut self, new_quantity: u16) {
        self.quantity = new_quantity;
    }
    pub fn update_slot(&mut self, new_slot: Slot) {
        self.slot = Some(new_slot);
    }
    pub fn change_to_available (&mut self) {
        self.status = Status::Available;
    }
    pub fn change_to_awaiting_putaway(&mut self, slot: String) {
        self.status = Status::AwaitingPutaway(Slot::new(slot));
    }
    pub fn create(id: u64, sku: u32, quantity: u16) -> Pallet {
        Pallet {
            id,
            sku,
            quantity,
            slot: None,
            status: Status::Available,
        }
    }
}

