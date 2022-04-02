use std::error::Error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FurDBInfo {
    name: String,
}

impl FurDBInfo {
    pub fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        let name = String::from(name);

        Ok(Self { name })
    }
}
