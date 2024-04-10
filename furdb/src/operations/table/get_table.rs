use actix_web::{get, web};
use furdb_core::models as core_models;

use crate::models;

#[get("/{database_id}/{table_id}")]
pub async fn get_table_handler(
    data: web::Data<core_models::furdb::FurDB>,
    path: web::Path<(String, String)>,
) -> Result<
    models::response::success_response::SuccessResponse,
    models::response::error_response::ErrorResponse,
> {
    let (database_id, table_id) = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    let response = models::response::table::get_table_response::GetTableResponse::new(&table);

    Ok(models::response::success_response::SuccessResponse::TableInfo(response))
}
