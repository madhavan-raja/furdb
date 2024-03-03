use actix_web::{post, web, Responder};
use furdb_core::models as core_models;
use std::error::Error;

use crate::models;

#[post("/{database_id}")]
pub async fn create_database_handler(
    path: web::Path<String>,
    create_database_params: Option<web::Json<models::CreateDatabaseParams>>,
) -> Result<impl Responder, Box<dyn Error>> {
    let database_id = path.into_inner();

    let database_name = create_database_params
        .and_then(|create_database_params| create_database_params.database_name.clone());

    core_models::database::Database::create_database(&database_id, database_name.as_deref())?;

    let response = models::BlankSuccessResponse::new();
    Ok(web::Json(response))
}
