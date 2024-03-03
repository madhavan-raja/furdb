use actix_web::{get, web, Responder};
use furdb_core::models as core_models;
use std::error::Error;

use crate::models;

#[get("/{database_id}/{table_id}")]
pub async fn get_table_handler(
    path: web::Path<(String, String)>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();

    let database = core_models::database::Database::get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    let res = models::GetTableResponse::new(&table)?;

    Ok(web::Json(res))
}
