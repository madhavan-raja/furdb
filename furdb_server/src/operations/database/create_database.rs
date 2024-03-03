use actix_web::{post, web, Responder};
use furdb_core::Database;
use std::error::Error;

use crate::api_response;
use crate::models;

#[post("/{database_id}")]
pub async fn create_database_handler(
    path: web::Path<String>,
    create_database_params: Option<web::Json<models::CreateDatabaseParams>>,
) -> Result<impl Responder, Box<dyn Error>> {
    let database_id = path.into_inner();

    let database_name = create_database_params
        .and_then(|create_database_params| create_database_params.database_name.clone());

    Database::create_database(&database_id, database_name.as_deref())?;

    let res = api_response::ApiResponse::new(models::CreateDatabaseResponse::new());

    Ok(web::Json(res))
}
