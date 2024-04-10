use actix_web::delete;
use actix_web::web::{Data, Path};

use crate::core::models::furdb::FurDB;

use crate::server::models::response::error_response::ErrorResponse;
use crate::server::models::response::success_response::SuccessResponse;

#[delete("/{database_id}/{table_id}")]
pub async fn delete_table_handler(
    data: Data<FurDB>,
    path: Path<(String, String)>,
) -> Result<SuccessResponse, ErrorResponse> {
    let (database_id, table_id) = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;

    database.delete_table(&table_id)?;

    Ok(SuccessResponse::TableDeleted)
}
