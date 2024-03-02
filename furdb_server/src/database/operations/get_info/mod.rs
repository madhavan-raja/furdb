use actix_web::{get, post, web, HttpRequest, Responder};
use std::error::Error;

mod utils;

mod response;
use response::DatabaseResponse;

mod params;

#[post("/{db}")]
pub(crate) async fn create_database_handler(
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<impl Responder, Box<dyn Error>> {
    let database_id = path.into_inner();
    let params =
        web::Query::<params::CreateDatabaseParams>::from_query(req.query_string()).unwrap();

    let database_name = params.db_name.clone().unwrap_or(database_id.clone());

    let database = utils::create_database(&database_id, &database_name)?;
    let db_tables = database.get_all_table_ids()?;

    let info = database.get_info()?.clone();
    let res = DatabaseResponse::new(info, db_tables);

    Ok(web::Json(res))
}

#[get("/{db}")]
pub(crate) async fn get_info_handler(
    path: web::Path<String>,
) -> Result<impl Responder, Box<dyn Error>> {
    let database_id = path.into_inner();

    let database = utils::get_database(&database_id)?;
    let db_tables = database.get_all_table_ids()?;

    let info = database.get_info()?.clone();
    let res = DatabaseResponse::new(info, db_tables);

    Ok(web::Json(res))
}
