use actix_web::{post, web, Responder};
use furdb_core::{Database, DatabaseInfo};
use std::error::Error;

use crate::api_response;
use crate::models;
use crate::utils;

#[post("/{database_id}")]
pub async fn create_database_handler(
    path: web::Path<String>,
    create_database_params: Option<web::Json<models::CreateDatabaseParams>>,
) -> Result<impl Responder, Box<dyn Error>> {
    let database_id = path.into_inner();

    let database_name = create_database_params
        .and_then(|create_database_params| create_database_params.database_name.clone())
        .unwrap_or(database_id.clone());

    Database::create_database(
        utils::get_database_path(&database_id),
        DatabaseInfo::new(&database_name)?,
    )?;

    let res = api_response::ApiResponse::new(models::CreateDatabaseResponse::new());

    Ok(web::Json(res))
}
