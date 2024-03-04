#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct GetRowParams {
    index: u64,
}

impl GetRowParams {
    pub(crate) fn get_index(&self) -> u64 {
        self.index
    }
}
