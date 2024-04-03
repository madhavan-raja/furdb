use furdb_core::models as core_models;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct GetEntriesResponse {
    result_count: usize,
    results: Vec<Entry>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Entry {
    index: usize,
    data: Vec<u128>,
}

impl GetEntriesResponse {
    pub(crate) fn new(
        entries_result: &core_models::query_result::QueryResult,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            result_count: entries_result.get_result_count(),
            results: entries_result
                .get_results()
                .iter()
                .map(|entry| Entry::new(entry))
                .collect(),
        })
    }
}

impl Entry {
    pub(crate) fn new(entry: &core_models::query_result::Entry) -> Self {
        Self {
            index: entry.get_index(),
            data: entry.get_data(),
        }
    }
}
