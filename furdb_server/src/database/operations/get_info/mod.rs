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

    let db_path = utils::get_database_path(&database_id);

    let db_info = DatabaseInfo::new(&database_name)?;

    Database::create_database(db_path.clone(), db_info)?;

    // Maybe we can remove everything after this and just send a 201 status code?

    let database = Database::get_database(db_path)?;

    let info = database.get_info()?.clone();
    let db_tables = database.get_all_table_ids()?;

    let res = response::DatabaseResponse::new(info, db_tables);

    Ok(web::Json(res))
}

#[get("/{db}")]
pub(crate) async fn get_info_handler(
    path: web::Path<String>,
) -> Result<impl Responder, Box<dyn Error>> {
    let database = Database::get_database(utils::get_database_path(&path.into_inner()))?;

    let info = database.get_info()?.clone();
    let db_tables = database.get_all_table_ids()?;

    let res = response::DatabaseResponse::new(info, db_tables);

    let res = api_response::ApiResponse::new(res);

    Ok(web::Json(res))
}
