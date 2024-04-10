use actix_web::get;
use actix_web::web::{Data, Path};

use furdb_core::models::furdb::FurDB;

use crate::models::response::database::get_database_response::GetDatabaseResponse;

use crate::models::response::error_response::ErrorResponse;
use crate::models::response::success_response::SuccessResponse;

#[get("/{database_id}")]
pub async fn get_database_handler(
    data: Data<FurDB>,
    path: Path<String>,
) -> Result<SuccessResponse, ErrorResponse> {
    let database_id = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;

    let response = GetDatabaseResponse::new(&database)?;

    Ok(SuccessResponse::DatabaseInfo(response))
}
