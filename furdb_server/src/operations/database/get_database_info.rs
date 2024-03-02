use actix_web::{get, web, Responder};
use furdb_core::Database;
use std::error::Error;

use crate::api_response;

use crate::models;

use crate::utils;

#[get("/{database_id}")]
pub async fn get_database_info_handler(
    path: web::Path<String>,
) -> Result<impl Responder, Box<dyn Error>> {
    let database = Database::get_database(utils::get_database_path(&path.into_inner()))?;

    let database_info = database.get_database_info()?.clone();
    let database_tables = database.get_all_table_ids()?;

    let res = api_response::ApiResponse::new(models::GetDatabaseResponse::new(
        database_info,
        database_tables,
    ));

    Ok(web::Json(res))
}
