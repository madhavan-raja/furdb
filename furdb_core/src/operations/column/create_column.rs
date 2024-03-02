use std::error::Error;

use crate::{Column, DataType};

impl Column {
    pub fn new(
        id: &str,
        description: Option<&str>,
        size: u128,
        data_type: DataType,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            id: String::from(id),
            description: String::from(description.unwrap_or(&id)),
            size,
            data_type: data_type,
        })
    }
}
