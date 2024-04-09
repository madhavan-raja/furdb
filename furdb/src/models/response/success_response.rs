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

#[derive(Debug)]
pub struct SuccessResponse {
    pub status_code: StatusCode,
    pub response: SuccessResponseType,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponseSerializable {
    pub status_code: u16,
    pub status: String,
    pub response: SuccessResponseType,
}

impl Into<SuccessResponseSerializable> for SuccessResponse {
    fn into(self) -> SuccessResponseSerializable {
        SuccessResponseSerializable {
            status_code: self.status_code.as_u16(),
            status: self
                .status_code
                .canonical_reason()
                .unwrap_or_default()
                .to_string(),
            response: self.response,
        }
    }
}

impl Display for SuccessResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Responder for SuccessResponse {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let status_code = self.status_code;
        HttpResponse::build(status_code).json(Into::<SuccessResponseSerializable>::into(self))
    }
}
