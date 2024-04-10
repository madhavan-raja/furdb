use actix_web::{delete, web};
use furdb_core::models as core_models;

use crate::models::{self, response::api_response::ApiResponse};

#[delete("/{database_id}/{table_id}")]
pub async fn delete_table_handler(
    data: web::Data<core_models::furdb::FurDB>,
    path: web::Path<(String, String)>,
) -> Result<models::response::api_response::ApiResponse, models::response::api_response::ApiResponse>
{
    let (database_id, table_id) = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;

    database.delete_table(&table_id)?;

    Ok(ApiResponse::Success(
        models::response::success_response::SuccessResponse::TableDeleted,
    ))
}
