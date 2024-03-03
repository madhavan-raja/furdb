use actix_web::{get, web, Responder};
use furdb_core::Database;
use std::error::Error;

use crate::api_response;

use crate::models;

#[get("/{database_id}")]
pub async fn get_database_info_handler(
    path: web::Path<String>,
) -> Result<impl Responder, Box<dyn Error>> {
    let database_id = path.into_inner();

    let database = Database::get_database(&database_id)?;
    let database_tables = database.get_all_tables()?;

    let res = api_response::ApiResponse::new(models::GetDatabaseResponse::new(
        &database.get_database_id(),
        &database.get_database_name(),
        database_tables,
    ));

    Ok(web::Json(res))
}
