use actix_web::{get, post, web, HttpRequest, Responder};
use furdb_core::{Database, DatabaseInfo};
use std::error::Error;

use crate::api_response;

mod params;
mod response;
mod utils;

#[post("/{db}")]
pub(crate) async fn create_database_handler(
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<impl Responder, Box<dyn Error>> {
    let database_id = path.into_inner();
    let params =
        web::Query::<params::CreateDatabaseParams>::from_query(req.query_string()).unwrap();

    let database_name = params.db_name.clone().unwrap_or(database_id.clone());

    Database::create_database(
        utils::get_database_path(&database_id),
        DatabaseInfo::new(&database_name)?,
    )?;

    let res = api_response::ApiResponse::new(response::CreateDatabaseResponse::new());

    Ok(web::Json(res))
}

#[get("/{db}")]
pub(crate) async fn get_info_handler(
    path: web::Path<String>,
) -> Result<impl Responder, Box<dyn Error>> {
    let database = Database::get_database(utils::get_database_path(&path.into_inner()))?;

    let database_info = database.get_info()?.clone();
    let database_tables = database.get_all_table_ids()?;

    let res = api_response::ApiResponse::new(response::GetDatabaseResponse::new(
        database_info,
        database_tables,
    ));

    Ok(web::Json(res))
}
