use actix_web::http::StatusCode;

use crate::models;

use models::response::success_response::SuccessResponse;
use models::response::success_response::SuccessResponseType;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub(crate) struct DeleteTableResponse {}

impl DeleteTableResponse {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Into<SuccessResponse> for DeleteTableResponse {
    fn into(self) -> SuccessResponse {
        SuccessResponse {
            status_code: StatusCode::OK.as_u16(),
            response: SuccessResponseType::TableDeleted,
        }
    }
}
