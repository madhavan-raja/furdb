use actix_web::{HttpResponse, Responder};
use serde::Serialize;

use crate::core::furdb_config::FurDBConfig;

use crate::core::models;

use models::database::DatabaseInfo;
use models::entries_result::EntriesResult;
use models::table::TableInfo;

use super::api_response::ApiResponseSerializable;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum SuccessResponse {
    ServerInfo(FurDBConfig),
    DatabaseCreated,
    DatabaseInfo(DatabaseInfo),
    DatabaseDeleted,
    TableCreated,
    TableInfo(TableInfo),
    TableDeleted,
    EntriesCreated,
    EntriesResult(EntriesResult),
    EntriesDeleted,
}

impl Responder for SuccessResponse {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let (response, status_code) = ApiResponseSerializable::generate_success(&self);
        HttpResponse::build(status_code).json(response)
    }
}
