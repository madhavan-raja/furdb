use actix_web::{get, web};
use furdb_core::models as core_models;

use crate::models;

#[get("/{database_id}")]
pub(crate) async fn get_database_handler(
    data: web::Data<core_models::furdb::FurDB>,
    path: web::Path<String>,
) -> Result<
    models::response::success_response::SuccessResponse,
    models::response::error_response::ErrorResponse,
> {
    let database_id = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;

    let response =
        models::response::database::get_database_response::GetDatabaseResponse::new(&database)?;

    Ok(response.into())
}
