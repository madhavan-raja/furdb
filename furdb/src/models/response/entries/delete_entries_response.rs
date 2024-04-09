use actix_web::http::StatusCode;

use crate::models;

use models::response::success_response::SuccessResponse;
use models::response::success_response::SuccessResponseType;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DeleteEntriesResponse {}

impl DeleteEntriesResponse {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<SuccessResponse> for DeleteEntriesResponse {
    fn into(self) -> SuccessResponse {
        SuccessResponse {
            status_code: StatusCode::OK,
            response: SuccessResponseType::QueryDeleted,
        }
    }
}
