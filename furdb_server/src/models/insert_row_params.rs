#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct InsertRowParams {
    data: Vec<u128>,
}

impl InsertRowParams {
    pub(crate) fn get_data(&self) -> Vec<u128> {
        self.data.to_vec()
    }
}
