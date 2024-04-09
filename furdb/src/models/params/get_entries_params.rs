#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetEntryParams {
    indices: Option<Vec<u64>>,
}

impl GetEntryParams {
    pub fn get_indices(&self) -> Option<Vec<u64>> {
        self.indices.to_owned()
    }
}
