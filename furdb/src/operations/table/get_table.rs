use actix_web::{get, web, Responder};
use furdb_core::models as core_models;
use std::error::Error;

use crate::models;

#[get("/{database_id}/{table_id}")]
pub(crate) async fn get_table_handler(
    data: web::Data<core_models::furdb::FurDB>,
    path: web::Path<(String, String)>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    let res = models::response::get_table_response::GetTableResponse::new(&table)?;

    Ok(web::Json(res))
}
