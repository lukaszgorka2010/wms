use rand::Rng;
use std::collections::HashMap;
use crate::{pallet::{Pallet, self}, slots::{Slots, Slot}};

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Failed to perform putaway")]
    Putaway,
}

#[derive(Debug, Default)]
pub struct Storage(HashMap<u64, Pallet>);

impl Storage {
    pub fn add_pallet(&mut self, sku: u32, quantity: u16, did: u32) -> u64 {
        let id = self.get_free_id(did);
        self.0.insert(id, Pallet::create(id, sku, quantity));
        id
    }

    pub fn receiving(&mut self, slots: &mut Slots, sku: u32, quantity: u16, did: u32) {
        let id = self.add_pallet(sku, quantity, did);
        let slot = slots.get_empty().unwrap_or(Slot::new("X9999".to_string()));
        slots.assign_pallet(slot.to_owned(), id);
        self.0.get_mut(&id).unwrap().change_to_awaiting_putaway(slot);
    }
    pub fn putaway(&mut self, pallet_id: u64) -> Result<(), StorageError> {
        let pallet = self.0.get_mut(&pallet_id).unwrap();
        match pallet.clone().status() {
            pallet::Status::AwaitingPutaway(slot) => {
                pallet.update_slot(slot);
                pallet.change_to_available();
                Ok(())
            },
            _ => Err(StorageError::Putaway),
        }
    }

    fn get_free_id(&self, did: u32) -> u64 {
        let mut rng = rand::thread_rng();
        loop{
            let tail: u64 = rng.gen_range(0..=99999999999999);
            let candidate_id: u64 = did as u64 * 10_u64.pow(15) + tail;
            if !self.0.contains_key(&candidate_id) {
                return candidate_id;
            }
        }
        
    }
}