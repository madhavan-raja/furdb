use actix_web::delete;
use actix_web::web::{Data, Json, Path};

use crate::core::models::furdb::FurDB;

use crate::server::models::params::delete_entries_params::DeleteEntriesParams;

use crate::server::models::response::error_response::ErrorResponse;
use crate::server::models::response::success_response::SuccessResponse;

#[delete("/{database_id}/{table_id}/data")]
pub async fn delete_entries_handler(
    data: Data<FurDB>,
    path: Path<(String, String)>,
    delete_entries_params: Json<DeleteEntriesParams>,
) -> Result<SuccessResponse, ErrorResponse> {
    let (database_id, table_id) = path.into_inner();
    let indices = delete_entries_params.get_indices();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    table.delete_entries(indices)?;

    Ok(SuccessResponse::EntriesDeleted)
}
