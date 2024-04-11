use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    name: String,
    size: u128,
}

impl Column {
    pub fn get_size(&self) -> u128 {
        self.size
    }
}
