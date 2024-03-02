use actix_web::{post, web, HttpRequest, Responder};
use furdb_core::{Database, DatabaseInfo};
use std::error::Error;

use crate::api_response;
use crate::models;
use crate::utils;

#[post("/{database_id}")]
pub async fn create_database_handler(
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<impl Responder, Box<dyn Error>> {
    let database_id = path.into_inner();
    let params =
        web::Query::<models::CreateDatabaseParams>::from_query(req.query_string()).unwrap();

    let database_name = params.db_name.clone().unwrap_or(database_id.clone());

    Database::create_database(
        utils::get_database_path(&database_id),
        DatabaseInfo::new(&database_name)?,
    )?;

    let res = api_response::ApiResponse::new(models::CreateDatabaseResponse::new());

    Ok(web::Json(res))
}
