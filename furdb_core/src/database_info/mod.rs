use std::error::Error;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DatabaseInfo {
    name: String,
}

impl DatabaseInfo {
    pub fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        let name = String::from(name);

        Ok(Self { name })
    }
}
