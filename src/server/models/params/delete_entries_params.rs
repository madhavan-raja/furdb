use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum DeleteEntriesType {
    All,
    ByIndices(Vec<u64>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeleteEntriesParams {
    entries: DeleteEntriesType,
}

impl DeleteEntriesParams {
    pub fn get_entries(&self) -> DeleteEntriesType {
        self.entries.to_owned()
    }
}
