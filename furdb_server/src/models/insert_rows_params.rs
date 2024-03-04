#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct InsertRowsParams {
    data: Vec<Vec<u128>>,
}

impl InsertRowsParams {
    pub(crate) fn get_data(&self) -> Vec<Vec<u128>> {
        self.data.to_vec()
    }
}
