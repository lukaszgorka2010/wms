use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, thiserror::Error)]
pub enum OrderFillingError {
    #[error("Failed to import order file: {0}")]
    FileReadError(std::io::Error),
    #[error("Failed to deserialize order file: {0}")]
    FileDeserializeError(serde_json::Error),
    #[error("Failed to release order")]
    ReleaseError,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub order_number: u64,
    pub depot_id: u32,
    pub order_lines: HashMap<String, u32>,
}

impl Order {
    pub fn from_file(path: &str) -> Result<Self, OrderFillingError> {
        match fs::read_to_string(path) {
            Ok(json) => match serde_json::from_str::<Order>(&json) {
                Ok(order) => Ok(order),
                Err(why) => Err(OrderFillingError::FileDeserializeError(why)),
            },
            Err(why) => Err(OrderFillingError::FileReadError(why)),
        }
    }
    pub fn release(&mut self) -> Result<(), OrderFillingError>{
        Ok(())
    }
}