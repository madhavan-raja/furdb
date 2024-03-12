#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct GetRowParams {
    column_index: u64,
    value: u128,
}

impl GetRowParams {
    pub(crate) fn get_column_index(&self) -> u64 {
        self.column_index
    }

    pub(crate) fn get_value(&self) -> u128 {
        self.value
    }
}
