use actix_web::http::StatusCode;
use furdb_core::models as core_models;

use crate::models::response::success_response::{SuccessResponse, SuccessResponseType};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GetEntriesResponse {
    result_count: usize,
    results: Vec<Entry>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Entry {
    index: usize,
    data: Vec<u128>,
}

impl GetEntriesResponse {
    pub fn new(entries_result: &core_models::query_result::QueryResult) -> Self {
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
    pub fn new(entry: &core_models::query_result::Entry) -> Self {
        Self {
            index: entry.get_index(),
            data: entry.get_data(),
        }
    }
}

impl Into<SuccessResponse> for GetEntriesResponse {
    fn into(self) -> SuccessResponse {
        SuccessResponse {
            status_code: StatusCode::OK.as_u16(),
            response: SuccessResponseType::QueryResult(self),
        }
    }
}
