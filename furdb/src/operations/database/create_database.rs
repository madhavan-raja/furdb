use actix_web::{post, web, Responder};
use furdb_core::models as core_models;
use std::error::Error;

use crate::models;

#[post("/{database_id}")]
pub(crate) async fn create_database_handler(
    path: web::Path<String>,
    create_database_params: web::Json<models::params::create_database_params::CreateDatabaseParams>,
) -> Result<impl Responder, Box<dyn Error>> {
    let database_id = path.into_inner();
    let database_name = create_database_params.get_database_name();

    let furdb = core_models::furdb::FurDB::new(core_models::config::Config::new(None)?)?;
    furdb.create_database(&database_id, database_name.as_deref())?;

    let response = models::response::blank_success_response::BlankSuccessResponse::new();
    Ok(web::Json(response))
}
