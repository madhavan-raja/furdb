#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct DeleteEntriesParams {
    indices: Option<Vec<u64>>,
}

impl DeleteEntriesParams {
    pub(crate) fn get_indices(&self) -> Option<Vec<u64>> {
        self.indices.to_owned()
    }
}
