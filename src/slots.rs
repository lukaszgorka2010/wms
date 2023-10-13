use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Slot {
    id: String,
}
impl Slot {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string()
        }
    }
}

#[derive(Debug, Default)]
pub struct Slots {
    empty: Vec<Slot>,
    occupied: HashMap<Slot, u64>,
}

impl Slots {
    pub fn add_slot( &mut self, slot: &str) {
        self.empty.push(Slot::new(slot));
    }

    pub fn get_empty(&mut self) -> Option<Slot> {
        self.empty.pop()
    }

    pub fn assign_pallet(&mut self, slot: Slot, pallet_id: u64) {
        self.occupied.insert(slot, pallet_id);
    }
}