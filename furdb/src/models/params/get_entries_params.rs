#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct GetEntryParams {
    indices: Option<Vec<u64>>,
}

impl GetEntryParams {
    pub(crate) fn get_indices(&self) -> Option<Vec<u64>> {
        self.indices.to_owned()
    }
}
