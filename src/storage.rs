use std::collections::HashMap;

use crate::{pallet::{Pallet, self}, slots::{Slots, Slot}};

const DUMMY_SLOT_ID: &str = "X9999";

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Failed to perform putaway")]
    Putaway,
}

#[derive(Debug, Default)]
pub struct Storage {
    pallets: HashMap<u64, Pallet>,
    next_id: u64,
}

impl Storage {
    pub fn add_pallet(&mut self, sku: u32, quantity: u16, did: u32) -> u64 {
        let id = self.get_free_id(did);
        self.pallets.insert(id, Pallet::create(id, sku, quantity));
        id
    }

    pub fn receiving(&mut self, slots: &mut Slots, sku: u32, quantity: u16, did: u32) {
        let id = self.add_pallet(sku, quantity, did);
        let slot = slots.get_empty().unwrap_or(Slot::new(DUMMY_SLOT_ID));
        slots.assign_pallet(slot.to_owned(), id);
        self.pallets.get_mut(&id).unwrap().change_to_awaiting_putaway(slot);
    }
    pub fn putaway(&mut self, pallet_id: u64) -> Result<(), StorageError> {
        let pallet_status = self.pallets.get(&pallet_id)
                                            .map(|p| p.status().clone())
                                            .ok_or(StorageError::Putaway)?;

        if let pallet::Status::AwaitingPutaway(slot) = pallet_status {
            if let Some(pallet) = self.pallets.get_mut(&pallet_id) {
                pallet.update_slot(&slot);
                pallet.change_to_available();
                return Ok(());
            }
        }
        Err(StorageError::Putaway)
    }

    fn get_free_id(&mut self, did: u32) -> u64 {
        let candidate_id= did as u64 * 10_u64.pow(15) + self.next_id;
        self.next_id += 1;
        candidate_id
    }
        
}