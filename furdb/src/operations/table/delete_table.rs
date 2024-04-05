use actix_web::{delete, web, Responder};
use furdb_core::models as core_models;
use std::error::Error;

use crate::models;

#[delete("/{database_id}/{table_id}")]
pub(crate) async fn delete_table_handler(
    data: web::Data<core_models::furdb::FurDB>,
    path: web::Path<(String, String)>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;

    database.delete_table(&table_id)?;

    let res = models::response::blank_success_response::BlankSuccessResponse::new();

    Ok(web::Json(res))
}
