use serde::{Deserialize, Serialize};

use crate::core::models::query_result::Entry as CoreEntry;
use crate::core::models::query_result::QueryResult;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetEntriesResponse {
    result_count: usize,
    results: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    index: usize,
    data: Vec<u128>,
}

impl GetEntriesResponse {
    pub fn new(entries_result: &QueryResult) -> Self {
        Self {
            result_count: entries_result.get_result_count(),
            results: entries_result
                .get_results()
                .iter()
                .map(|entry| Entry::new(entry))
                .collect(),
        }
    }
}

impl Entry {
    pub fn new(entry: &CoreEntry) -> Self {
        Self {
            index: entry.get_index(),
            data: entry.get_data(),
        }
    }
}
