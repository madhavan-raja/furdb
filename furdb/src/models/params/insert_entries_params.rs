#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct InsertEntriesParams {
    data: Vec<Vec<u128>>,
}

impl InsertEntriesParams {
    pub(crate) fn get_data(&self) -> Vec<Vec<u128>> {
        self.data.to_vec()
    }
}
