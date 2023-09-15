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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_slot() {
        let mut slots = Slots::default();
        slots.add_slot("A1001".to_string());
        assert_eq!(slots.empty.len(), 1);
        assert_eq!(slots.empty[0].to_string(), "A1001");
    }

    #[test]
    fn test_get_empty_slot() {
        let mut slots = Slots::default();
        slots.add_slot("A1001".to_string());
        let slot = slots.get_empty().unwrap();
        assert_eq!(slot.to_string(), "A1001");
        assert_eq!(slots.empty.len(), 0);
    }

    #[test]
    fn test_get_empty_slot_when_none() {
        let mut slots = Slots::default();
        let slot = slots.get_empty();
        assert!(slot.is_none());
    }

    #[test]
    fn test_assign_pallet() {
        let mut slots = Slots::default();
        let slot = Slot::new("A1001".to_string());
        slots.assign_pallet(slot.clone(), 12345);
        assert_eq!(slots.occupied.get(&slot).unwrap(), &12345);
    }
}