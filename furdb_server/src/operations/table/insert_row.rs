use actix_web::{post, web, Responder};
use furdb_core::Database;
use std::error::Error;

use crate::models;

#[post("/{database_id}/{table_id}/data")]
pub async fn insert_row_handler(
    path: web::Path<(String, String)>,
    insert_row_params: web::Json<models::InsertRowParams>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();

    let database = Database::get_database(&database_id)?;
    let mut table = database.get_table(&table_id)?;

    table.insert_row(&insert_row_params.data)?;

    let res = models::BlankSuccessResponse::new();

    Ok(web::Json(res))
}
