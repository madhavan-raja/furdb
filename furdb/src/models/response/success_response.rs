use serde::Serialize;

use crate::models;

use models::response::{
    database::get_database_response::GetDatabaseResponse,
    entries::get_entries_response::GetEntriesResponse,
    info::server_health_response::ServerHealthResponse,
    table::get_table_response::GetTableResponse,
};

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
