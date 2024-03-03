use actix_web::{get, web, Responder};
use furdb_core::Database;
use std::error::Error;

use crate::models;

#[get("/{database_id}/{table_id}")]
pub async fn get_table_handler(
    path: web::Path<(String, String)>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();

    let database = Database::get_database(&database_id)?;

    let table = database.get_table(&table_id)?;

    let res = models::GetTableResponse::new(
        &table.get_table_id(),
        &table.get_table_name(),
        table.get_table_columns(),
    );

    Ok(web::Json(res))
}
