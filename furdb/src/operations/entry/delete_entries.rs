use actix_web::{delete, web};
use furdb_core::models as core_models;

use crate::models;

#[delete("/{database_id}/{table_id}/data")]
pub(crate) async fn delete_entries_handler(
    data: web::Data<core_models::furdb::FurDB>,
    path: web::Path<(String, String)>,
    delete_entries_params: web::Json<models::params::delete_entries_params::DeleteEntriesParams>,
) -> Result<
    models::response::success_response::SuccessResponse,
    models::response::error_response::ErrorResponse,
> {
    let (database_id, table_id) = path.into_inner();
    let indices = delete_entries_params.get_indices();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    table.delete_entries(indices)?;

    let response = models::response::entries::delete_entries_response::DeleteEntriesResponse::new();

    Ok(response.into())
}
