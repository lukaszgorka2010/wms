use std::{collections::HashMap, error::Error, fmt};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ImportError();
impl Error for ImportError {}
impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Import failed")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub order_number: u64,
    pub depot_id: u32,
    pub order_lines: HashMap<String, u32>,
}

impl Order {
     pub fn import() -> Result<Self, ImportError> {
        todo!()
     }
 }