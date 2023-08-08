use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Slot(String);
impl Slot {
    pub fn new(slot: String) -> Slot {
        Slot(slot)
    }
}

#[derive(Debug)]
pub struct Slots {
    empty: Vec<Slot>,
    occupied: Vec<HashMap<Slot, u64>>,
}

impl Slots {
    pub fn add_slot( &mut self, slot: String) {
        self.empty.push(Slot(slot));
    }

    pub fn get_empty(&self) -> Option<&Slot> {
        self.empty.last()
    }

    pub fn assign_pallet(&mut self, slot: Slot, pallet_id: u64) {
        self.occupied.push({
            let mut map: HashMap<Slot, u64> = HashMap::new();
            map.insert(slot, pallet_id);
            map
        });
    }
}