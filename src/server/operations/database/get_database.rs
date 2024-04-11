use actix_web::get;
use actix_web::web::{Data, Path};

use crate::core::furdb::FurDB;

use crate::server::models::response::database::get_database_response::GetDatabaseResponse;

use crate::server::models::response::error_response::ErrorResponse;
use crate::server::models::response::success_response::SuccessResponse;

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
