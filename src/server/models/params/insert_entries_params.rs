use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertEntriesParams {
    data: Vec<Vec<u128>>,
}

impl InsertEntriesParams {
    pub fn get_data(&self) -> Vec<Vec<u128>> {
        self.data.to_vec()
    }
}
