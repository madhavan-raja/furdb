use std::error::Error;

use crate::Column;

impl Column {
    pub fn new(name: &str, size: u128) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            name: String::from(name),
            size,
        })
    }
}
