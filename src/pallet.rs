use crate::slots::Slot;

#[derive(Debug, Clone)]
pub struct Pallet {
    id: u64,
    sku: u32,
    quantity: u16,
    slot: Option<Slot>,
    status: Status,
}

#[derive(Debug, Clone)]
pub enum Status {
    Available,
    AwaitingPutaway(Slot),
}
impl Pallet {
    pub fn update_quantity(&mut self, new_quantity: u16) {
        self.quantity = new_quantity;
    }
    pub fn update_slot(&mut self, new_slot: &Slot) {
        self.slot = Some(new_slot.clone());
    }
    pub fn change_to_available (&mut self) {
        self.status = Status::Available;
    }
    pub fn change_to_awaiting_putaway(&mut self, slot: Slot) {
        self.status = Status::AwaitingPutaway(slot);
    }
    pub fn status(&self) -> &Status {
        &self.status
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slots::Slot;

    #[test]
    fn test_update_quantity() {
        let mut pallet = Pallet::create(1, 1001, 50);
        pallet.update_quantity(60);
        assert_eq!(pallet.quantity, 60);
    }

    #[test]
    fn test_update_slot() {
        let mut pallet = Pallet::create(1, 1001, 50);
        let slot = Slot::new("A1001");
        pallet.update_slot(&slot);
        assert_eq!(pallet.slot, Some(slot));
    }

    #[test]
    fn test_change_to_available() {
        let mut pallet = Pallet::create(1, 1001, 50);
        pallet.change_to_available();
        match pallet.status {
            Status::Available => assert!(true),
            _ => assert!(false, "Pallet status should be Available"),
        }
    }

    #[test]
    fn test_change_to_awaiting_putaway() {
        let mut pallet = Pallet::create(1, 1001, 50);
        let slot = Slot::new("A1001");
        pallet.change_to_awaiting_putaway(slot.clone());
        match &pallet.status {
            Status::AwaitingPutaway(s) => assert_eq!(s, &slot),
            _ => assert!(false, "Pallet status should be AwaitingPutaway"),
        }
    }

    #[test]
    fn test_create_pallet() {
        let pallet = Pallet::create(1, 1001, 50);
        assert_eq!(pallet.id, 1);
        assert_eq!(pallet.sku, 1001);
        assert_eq!(pallet.quantity, 50);
        assert_eq!(pallet.slot, None);
        match pallet.status {
            Status::Available => assert!(true),
            _ => assert!(false, "Newly created pallet should have status Available"),
        }
    }
}
