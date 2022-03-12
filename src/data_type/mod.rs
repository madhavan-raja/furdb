use std::error::Error;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FurDataType {
    id: String,
    converter_endpoint_override: Option<String>,
}

mod operations;
mod utils;

impl FurDataType {
    pub fn new(
        id: &str,
        converter_endpoint_override: Option<&str>,
    ) -> Result<FurDataType, Box<dyn Error>> {
        let id = String::from(id);
        let converter_endpoint_override = converter_endpoint_override.map(str::to_string);

        Ok(FurDataType {
            id,
            converter_endpoint_override,
        })
    }
}
