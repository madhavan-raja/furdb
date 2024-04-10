use actix_web::{delete, web};
use furdb_core::models as core_models;

use crate::models::{self, response::api_response::ApiResponse};

#[delete("/{database_id}")]
pub async fn delete_database_handler(
    data: web::Data<core_models::furdb::FurDB>,
    path: web::Path<String>,
) -> Result<models::response::api_response::ApiResponse, models::response::api_response::ApiResponse>
{
    let database_id = path.into_inner();

    let furdb = data.as_ref();
    furdb.delete_database(&database_id)?;

    Ok(ApiResponse::Success(
        models::response::success_response::SuccessResponse::DatabaseDeleted,
    ))
}
