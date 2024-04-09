use actix_web::http::StatusCode;
use furdb_core::models as core_models;

use crate::models;

use models::response::success_response::SuccessResponse;
use models::response::success_response::SuccessResponseType;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GetTableResponse {
    database_id: String,
    table_id: String,
    table_name: String,
    table_columns: Vec<core_models::column::Column>,
}

impl GetTableResponse {
    pub fn new(table: &core_models::table::Table) -> Self {
        let table_info = table.get_table_info();

        Self {
            database_id: table_info.get_database_id(),
            table_id: table_info.get_table_id(),
            table_name: table_info.get_table_name(),
            table_columns: table_info.get_table_columns(),
        }
    }
}

impl Into<SuccessResponse> for GetTableResponse {
    fn into(self) -> SuccessResponse {
        SuccessResponse {
            status_code: StatusCode::OK,
            response: SuccessResponseType::TableInfo(self),
        }
    }
}
