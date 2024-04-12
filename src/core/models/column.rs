use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    size: u128,
}

impl Column {
    pub fn get_size(&self) -> u128 {
        self.size
    }
}
