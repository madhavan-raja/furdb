use actix_web::{HttpResponse, Responder};
use serde::Serialize;

use crate::models;

use models::response::{
    database::get_database_response::GetDatabaseResponse,
    entries::get_entries_response::GetEntriesResponse,
    info::server_health_response::ServerHealthResponse,
    table::get_table_response::GetTableResponse,
};

use super::api_response::ApiResponseSerializable;

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum SuccessResponse {
    ServerHealth(ServerHealthResponse),
    DatabaseCreated,
    DatabaseInfo(GetDatabaseResponse),
    DatabaseDeleted,
    TableCreated,
    TableInfo(GetTableResponse),
    TableDeleted,
    EntriesCreated,
    EntriesResult(GetEntriesResponse),
    EntriesDeleted,
}

impl Responder for SuccessResponse {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let (response, status_code) = ApiResponseSerializable::generate_success(&self);
        HttpResponse::build(status_code).json(response)
    }
}
