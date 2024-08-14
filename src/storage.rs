use std::collections::HashMap;

use crate::{pallet::{Pallet, self}, slots::{Slots, Slot}};

const DUMMY_SLOT_ID: &str = "X9999";

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Failed to perform putaway: {0}")]
    Putaway(String),
}

#[derive(Debug, Default)]
pub struct Storage {
    pallets: HashMap<u64, Pallet>,
    next_id: u64,
    depot_id: u64,
}
impl Storage {
    pub fn new(depot_id: u64) -> Self {
       Self {
        pallets: HashMap::new(),
        next_id: 0,
        depot_id: depot_id,
        }
    }

    pub fn add_pallet(&mut self, sku: u32, quantity: u16) -> u64 {
        let id = self.get_free_id();
        self.pallets.insert(id, Pallet::create(id, sku, quantity));
        id
    }

    pub fn receiving(&mut self, slots: &mut Slots, sku: u32, quantity: u16) {
        let id = self.add_pallet(sku, quantity);
        let slot = slots.get_empty().unwrap_or(Slot::new(DUMMY_SLOT_ID));
        slots.assign_pallet(slot.to_owned(), id);
        self.pallets.get_mut(&id).unwrap().change_to_awaiting_putaway(slot);
    }
    pub fn putaway(&mut self, pallet_id: u64) -> Result<(), StorageError> {
        let pallet_status = self.pallets.get(&pallet_id)
                                            .map(|p| p.status().clone())
                                            .ok_or(StorageError::Putaway(
                                                "Failed to obtain pallet status"
                                                .to_string()))?;

        if let pallet::Status::AwaitingPutaway(slot) = pallet_status {
            if let Some(pallet) = self.pallets.get_mut(&pallet_id) {
                pallet.update_slot(&slot);
                pallet.change_to_available();
                return Ok(());
            }
        }
        Err(StorageError::Putaway("Putaway failed".to_string()))
    }

    pub fn pallets(&self) -> &HashMap<u64, Pallet> {
        &self.pallets
    }

    fn get_free_id(&mut self) -> u64 {
        let depot_id = self.depot_id;
        let candidate_id= depot_id * 10_u64.pow(14) + self.next_id;
        self.next_id += 1;
        candidate_id
    }
        
}