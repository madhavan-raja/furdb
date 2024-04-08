#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Column {
    name: String,
    size: u128,
}

impl Column {
    pub fn new(name: &str, size: u128) -> Self {
        Self {
            name: String::from(name),
            size,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn get_size(&self) -> u128 {
        self.size
    }
}
