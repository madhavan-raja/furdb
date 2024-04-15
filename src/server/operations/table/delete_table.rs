use actix_web::delete;
use actix_web::web::{Data, Path};

use crate::core::FurDB;

use crate::server::models::response::ErrorResponse;
use crate::server::models::response::SuccessResponse;

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
