use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Slot(String);
impl Slot {
    pub fn new(slot: String) -> Self {
        Self(slot)
    }
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Default)]
pub struct Slots {
    empty: Vec<Slot>,
    occupied: HashMap<Slot, u64>,
}

impl Slots {
    pub fn add_slot( &mut self, slot: String) {
        self.empty.push(Slot(slot));
    }

    pub fn get_empty(&mut self) -> Option<Slot> {
        self.empty.pop()
    }

    pub fn assign_pallet(&mut self, slot: Slot, pallet_id: u64) {
        self.occupied.insert(slot, pallet_id);
    }
    //pub fn remove_pallet(&mut self,)
}