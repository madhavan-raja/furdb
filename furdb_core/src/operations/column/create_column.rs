use std::error::Error;

use crate::models;

impl models::column::Column {
    pub fn new(name: &str, size: u128) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            name: String::from(name),
            size,
        })
    }
}
