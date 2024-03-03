use actix_web::{get, web, Responder};
use furdb_core::Database;
use std::error::Error;

use crate::models;

#[get("/{database_id}")]
pub async fn get_database_info_handler(
    path: web::Path<String>,
) -> Result<impl Responder, Box<dyn Error>> {
    let database_id = path.into_inner();
    let database = Database::get_database(&database_id)?;

    let response = models::GetDatabaseResponse::new(&database)?;

    Ok(web::Json(response))
}
