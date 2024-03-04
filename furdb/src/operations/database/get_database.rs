use actix_web::{get, web, Responder};
use furdb_core::models as core_models;
use std::error::Error;

use crate::models;

#[get("/{database_id}")]
pub(crate) async fn get_database_handler(
    path: web::Path<String>,
) -> Result<impl Responder, Box<dyn Error>> {
    let database_id = path.into_inner();

    let furdb = core_models::furdb::FurDB::new(core_models::config::Config::new(None)?)?;
    let database = furdb.get_database(&database_id)?;

    let response = models::response::get_database_response::GetDatabaseResponse::new(&database)?;

    Ok(web::Json(response))
}
