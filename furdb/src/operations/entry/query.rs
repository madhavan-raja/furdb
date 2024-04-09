use actix_web::{get, web};
use furdb_core::models as core_models;

use crate::models;

#[get("/{database_id}/{table_id}/query")]
pub async fn query_handler(
    data: web::Data<core_models::furdb::FurDB>,
    path: web::Path<(String, String)>,
    query_params: web::Json<models::params::query_params::QueryParams>,
) -> Result<
    models::response::success_response::SuccessResponse,
    models::response::error_response::ErrorResponse,
> {
    let (database_id, table_id) = path.into_inner();
    let index = query_params.get_column_index();
    let value = query_params.get_value();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    let data = table.query(index, value)?;

    let response = models::response::entries::get_entries_response::GetEntriesResponse::new(&data);

    Ok(response.into())
}

#[get("/{database_id}/{table_id}/data")]
pub async fn get_entries_handler(
    data: web::Data<core_models::furdb::FurDB>,
    path: web::Path<(String, String)>,
    get_entry_params: web::Json<models::params::get_entries_params::GetEntryParams>,
) -> Result<
    models::response::success_response::SuccessResponse,
    models::response::error_response::ErrorResponse,
> {
    let (database_id, table_id) = path.into_inner();
    let indices = get_entry_params.get_indices();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    let data = table.get_entries(indices)?;

    let response = models::response::entries::get_entries_response::GetEntriesResponse::new(&data);

    Ok(response.into())
}
