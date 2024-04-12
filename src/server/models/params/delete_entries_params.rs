use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DeleteEntriesType {
    All,
    Indices(Vec<u64>),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteEntriesParams {
    entries: DeleteEntriesType,
}

impl DeleteEntriesParams {
    pub fn get_entries(&self) -> DeleteEntriesType {
        self.entries.to_owned()
    }
}
