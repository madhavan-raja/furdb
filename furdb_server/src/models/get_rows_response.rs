use furdb_core::models as core_models;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct GetRowsResponse {
    result_count: usize,
    results: Vec<Row>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Row {
    index: usize,
    data: Vec<u128>,
}

impl GetRowsResponse {
    pub(crate) fn new(
        rows_result: &core_models::get_rows_result::GetRowsResult,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            result_count: rows_result.get_result_count(),
            results: rows_result
                .get_results()
                .iter()
                .map(|row| Row::new(row))
                .collect(),
        })
    }
}

impl Row {
    pub(crate) fn new(row: &core_models::get_rows_result::Row) -> Self {
        Self {
            index: row.get_index(),
            data: row.get_data(),
        }
    }
}
