use actix_web::delete;
use actix_web::web::{Data, Path};

use crate::core::FurDB;

use crate::server::models::response::ErrorResponse;
use crate::server::models::response::SuccessResponse;

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
