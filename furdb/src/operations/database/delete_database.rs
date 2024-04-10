use actix_web::delete;
use actix_web::web::{Data, Path};

use furdb_core::models::furdb::FurDB;

use crate::models::response::error_response::ErrorResponse;
use crate::models::response::success_response::SuccessResponse;

#[delete("/{database_id}")]
pub async fn delete_database_handler(
    data: Data<FurDB>,
    path: Path<String>,
) -> Result<SuccessResponse, ErrorResponse> {
    let database_id = path.into_inner();

    let furdb = data.as_ref();
    furdb.delete_database(&database_id)?;

    Ok(SuccessResponse::DatabaseDeleted)
}
