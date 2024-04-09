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
pub enum SuccessResponseType {
    ServerHealth(ServerHealthResponse),
    DatabaseCreated,
    DatabaseInfo(GetDatabaseResponse),
    DatabaseDeleted,
    TableCreated,
    TableInfo(GetTableResponse),
    TableDeleted,
    QueryCreated,
    QueryResult(GetEntriesResponse),
    QueryDeleted,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub status_code: u16,
    pub response: SuccessResponseType,
}

impl Display for SuccessResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Responder for SuccessResponse {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let status_code = StatusCode::from_u16(self.status_code).unwrap_or_default();
        HttpResponse::build(status_code).json(self)
    }
}
