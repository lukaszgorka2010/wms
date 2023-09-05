use std::{collections::HashMap, error::Error, fmt, fs};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug)]
pub struct ImportError(String);
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
     pub fn from_file(path: &str) -> Result<Self, ImportError> {
        match fs::read_to_string(path) {
            Ok(json) => {
                match serde_json::from_str::<Order>(&json){
                    Ok(order) => Ok(order),
                    Err(why) => Err(ImportError(why.to_string())),
                }},
            Err(why) => Err(ImportError(why.to_string())),
        }
     }
 }