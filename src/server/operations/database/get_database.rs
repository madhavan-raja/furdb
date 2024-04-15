use actix_web::get;
use actix_web::web::{Data, Path};

use crate::core::FurDB;

use crate::server::models::response::ErrorResponse;
use crate::server::models::response::SuccessResponse;

#[get("/{database_id}")]
pub async fn get_database_handler(
    data: Data<FurDB>,
    path: Path<String>,
) -> Result<SuccessResponse, ErrorResponse> {
    let database_id = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;

    let database_info_full = database.get_database_info_full()?;

    Ok(SuccessResponse::DatabaseInfo(database_info_full))
}
