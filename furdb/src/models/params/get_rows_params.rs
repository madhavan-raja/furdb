#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct GetRowParams {
    indices: Option<Vec<u64>>,
}

impl GetRowParams {
    pub(crate) fn get_indices(&self) -> Option<Vec<u64>> {
        self.indices.to_owned()
    }
}
