use actix_web::{HttpResponse, Responder};
use serde::Serialize;

use crate::core::FurDBConfig;

use crate::core::models;

use models::DatabaseInfo;
use models::DatabaseInfoExtra;

use models::TableInfo;

use models::EntriesResult;

use crate::server::models::response::ApiResponseSerializable;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum SuccessResponse {
    ServerInfo(FurDBConfig),
    DatabaseCreated(DatabaseInfo),
    DatabaseInfo(DatabaseInfoExtra),
    DatabaseDeleted,
    TableCreated(TableInfo),
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
