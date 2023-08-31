use std::{collections::HashMap, error::Error, fmt};

#[derive(Debug)]
pub struct ImportError();
impl Error for ImportError {}
impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Import failed")
    }
}

pub struct Order {
    depot_id: u32,
    order_number: u64,
    order_lines: HashMap<u32, u32>,
}
 impl Order {
     pub fn import() -> Result<Self, ImportError> {
        todo!();
     }
 }