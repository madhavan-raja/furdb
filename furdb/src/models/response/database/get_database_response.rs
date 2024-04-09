use actix_web::http::StatusCode;
use furdb_core::errors::table_errors::table_read_error::TableReadError;
use furdb_core::models as core_models;

use crate::models;

use models::response::success_response::SuccessResponse;
use models::response::success_response::SuccessResponseType;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GetDatabaseResponse {
    database_id: String,
    database_name: String,
    database_tables: Vec<GetDatabaseTableResponse>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct GetDatabaseTableResponse {
    table_id: String,
    table_name: String,
    table_columns: Vec<core_models::column::Column>,
}

impl GetDatabaseResponse {
    pub fn new(database: &core_models::database::Database) -> Result<Self, TableReadError> {
        let database_info = database.get_database_info();

        Ok(Self {
            database_id: database_info.get_database_id(),
            database_name: database_info.get_database_name(),
            database_tables: database
                .get_all_tables()?
                .into_iter()
                .map(|table| GetDatabaseTableResponse::new(&table))
                .collect(),
        })
    }
}

impl GetDatabaseTableResponse {
    pub fn new(table: &core_models::table::Table) -> Self {
        let table_info = table.get_table_info();

        Self {
            table_id: table_info.get_table_id(),
            table_name: table_info.get_table_name(),
            table_columns: table_info.get_table_columns(),
        }
    }
}

impl Into<SuccessResponse> for GetDatabaseResponse {
    fn into(self) -> SuccessResponse {
        SuccessResponse {
            status_code: StatusCode::OK.as_u16(),
            response: SuccessResponseType::DatabaseInfo(self),
        }
    }
}