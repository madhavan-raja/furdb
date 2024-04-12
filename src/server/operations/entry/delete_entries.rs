use actix_web::delete;
use actix_web::web::{Data, Json, Path};

use crate::core::furdb::FurDB;

use crate::server::models::params::delete_entries_params::{
    DeleteEntriesParams, DeleteEntriesType,
};

use crate::server::models::response::error_response::ErrorResponse;
use crate::server::models::response::success_response::SuccessResponse;

#[delete("/{database_id}/{table_id}/data")]
pub async fn delete_entries_handler(
    data: Data<FurDB>,
    path: Path<(String, String)>,
    delete_entries_params: Json<DeleteEntriesParams>,
) -> Result<SuccessResponse, ErrorResponse> {
    let (database_id, table_id) = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    match &delete_entries_params.get_entries() {
        DeleteEntriesType::All => table.delete_all_entries(),
        DeleteEntriesType::Indices(indices) => table.delete_entries(indices.to_vec()),
    }?;

    Ok(SuccessResponse::EntriesDeleted)
}
