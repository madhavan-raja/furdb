use actix_web::{http::StatusCode, HttpResponse, Responder};
use serde::Serialize;
use std::fmt::{Display, Formatter};

use crate::models;

use models::response::{
    database::get_database_response::GetDatabaseResponse,
    entries::get_entries_response::GetEntriesResponse,
    info::server_health_response::ServerHealthResponse,
    table::get_table_response::GetTableResponse,
};

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct SuccessResponseSerializable {
    pub status_code: u16,
    pub status: String,
    pub response: SuccessResponse,
}

impl Display for SuccessResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Responder for SuccessResponse {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let status_code = match self {
            SuccessResponse::ServerHealth(_) => StatusCode::OK,
            SuccessResponse::DatabaseCreated => StatusCode::CREATED,
            SuccessResponse::DatabaseInfo(_) => StatusCode::OK,
            SuccessResponse::DatabaseDeleted => StatusCode::NO_CONTENT,
            SuccessResponse::TableCreated => StatusCode::CREATED,
            SuccessResponse::TableInfo(_) => StatusCode::OK,
            SuccessResponse::TableDeleted => StatusCode::NO_CONTENT,
            SuccessResponse::EntriesCreated => StatusCode::CREATED,
            SuccessResponse::EntriesResult(_) => StatusCode::OK,
            SuccessResponse::EntriesDeleted => StatusCode::NO_CONTENT,
        };

        let status = status_code.canonical_reason().unwrap_or("").to_string();

        HttpResponse::build(status_code).json(SuccessResponseSerializable {
            status_code: status_code.as_u16(),
            status,
            response: self,
        })
    }
}
