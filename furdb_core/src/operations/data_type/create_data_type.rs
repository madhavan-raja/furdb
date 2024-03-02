use std::error::Error;

use crate::DataType;

impl DataType {
    pub fn new(
        id: &str,
        converter_endpoint_override: Option<&str>,
    ) -> Result<Self, Box<dyn Error>> {
        let id = String::from(id);
        let converter_endpoint_override = converter_endpoint_override.map(str::to_string);

        Ok(Self {
            id,
            converter_endpoint_override,
        })
    }
}
