use std::{collections::HashMap};
use rand::Rng;
use crate::{pallet::Pallet, DID};

#[derive(Debug, Default)]
pub struct Storage(HashMap<u64, Pallet>);

impl Storage {
    pub fn add_pallet(&mut self, sku: u32, quantity: u16){
        let id = self.get_free_id();
        self.0.insert(id, Pallet::create(id, sku, quantity));
    }

    //pub fn receiving

    fn get_free_id(&self) -> u64 {
        let mut rng = rand::thread_rng();
        loop{
            let tail: u64 = rng.gen_range(0..=99999999999999);
            let candidate_id: u64 = DID * 10_u64.pow(15) + tail;
            if !self.0.contains_key(&candidate_id) {
                return candidate_id;
            }
        }
        
    }
}