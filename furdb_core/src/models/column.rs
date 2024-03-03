#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Column {
    pub(crate) name: String,
    pub(crate) size: u128,
}

impl Column {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_size(&self) -> u128 {
        self.size
    }
}
