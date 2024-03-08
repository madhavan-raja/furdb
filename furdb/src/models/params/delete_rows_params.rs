#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct DeleteRowsParams {
    indices: Option<Vec<u64>>,
}

impl DeleteRowsParams {
    pub(crate) fn get_indices(&self) -> Option<Vec<u64>> {
        self.indices.to_owned()
    }
}
