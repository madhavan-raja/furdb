use actix_web::{get, web};
use furdb_core::models as core_models;

use crate::models::{self, params::get_entries_params::GetEntriesType};

#[get("/{database_id}/{table_id}/data")]
pub async fn get_entries_handler(
    data: web::Data<core_models::furdb::FurDB>,
    path: web::Path<(String, String)>,
    get_entries_params: web::Json<models::params::get_entries_params::GetEntriesParams>,
) -> Result<
    models::response::success_response::SuccessResponse,
    models::response::error_response::ErrorResponse,
> {
    let (database_id, table_id) = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    let data = match &get_entries_params.get_entries() {
        GetEntriesType::All => table.get_entries(None),
        GetEntriesType::ByIndices(get_entries_by_indices_params) => {
            table.get_entries(Some(get_entries_by_indices_params.to_vec()))
        }
        GetEntriesType::ByValue(get_entries_by_value_params) => table.query(
            get_entries_by_value_params.get_column_index(),
            get_entries_by_value_params.get_value(),
        ),
    }?;

    let response = models::response::entries::get_entries_response::GetEntriesResponse::new(&data);

    Ok(models::response::success_response::SuccessResponse::EntriesResult(response))
}
