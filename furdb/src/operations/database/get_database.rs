use actix_web::{get, web};
use furdb_core::models as core_models;

use crate::models::{self, response::api_response::ApiResponse};

#[get("/{database_id}")]
pub async fn get_database_handler(
    data: web::Data<core_models::furdb::FurDB>,
    path: web::Path<String>,
) -> Result<models::response::api_response::ApiResponse, models::response::api_response::ApiResponse>
{
    let database_id = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;

    let response =
        models::response::database::get_database_response::GetDatabaseResponse::new(&database)?;

    Ok(ApiResponse::Success(
        models::response::success_response::SuccessResponse::DatabaseInfo(response),
    ))
}
