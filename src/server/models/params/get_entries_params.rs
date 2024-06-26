use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetEntriesByValuesParams {
    column_index: u64,
    value: u128,
}

impl GetEntriesByValuesParams {
    pub fn get_column_index(&self) -> u64 {
        self.column_index.to_owned()
    }

    pub fn get_value(&self) -> u128 {
        self.value.to_owned()
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GetEntriesType {
    All,
    Indices(Vec<u64>),
    Value(GetEntriesByValuesParams),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetEntriesParams {
    entries: GetEntriesType,
}

impl GetEntriesParams {
    pub fn get_entries(&self) -> GetEntriesType {
        self.entries.to_owned()
    }
}
