use actix_web::post;
use actix_web::web::{Data, Path};

use crate::core::FurDB;

use crate::server::models::response::ErrorResponse;
use crate::server::models::response::SuccessResponse;

#[post("/{database_id}")]
pub async fn create_database_handler(
    data: Data<FurDB>,
    path: Path<String>,
) -> Result<SuccessResponse, ErrorResponse> {
    let database_id = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.create_database(&database_id)?;

    let database_info = database.get_database_info();

    Ok(SuccessResponse::DatabaseCreated(database_info))
}
