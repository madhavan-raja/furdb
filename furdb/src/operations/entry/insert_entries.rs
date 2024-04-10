use actix_web::{post, web};
use furdb_core::models as core_models;

use crate::models::{self, response::api_response::ApiResponse};

#[post("/{database_id}/{table_id}/data")]
pub async fn insert_entries_handler(
    data: web::Data<core_models::furdb::FurDB>,
    path: web::Path<(String, String)>,
    insert_entries_params: web::Json<models::params::insert_entries_params::InsertEntriesParams>,
) -> Result<models::response::api_response::ApiResponse, models::response::api_response::ApiResponse>
{
    let (database_id, table_id) = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    table.insert_entries(&insert_entries_params.get_data())?;

    Ok(ApiResponse::Success(
        models::response::success_response::SuccessResponse::EntriesCreated,
    ))
}
