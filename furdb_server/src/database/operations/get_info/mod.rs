use actix_web::{get, post, web, HttpRequest, Responder};
use std::error::Error;

mod utils;
use utils::get_db;

mod response;
use response::DatabaseResponse;

mod params;
use params::DatabaseParams;

#[post("/{db}")]
pub(crate) async fn create_table_handler(
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<impl Responder, Box<dyn Error>> {
    let db_path = path.into_inner();
    let params = web::Query::<DatabaseParams>::from_query(req.query_string()).unwrap();

    todo!("Create database");

    let db = get_db(&db_path, params.db_name.clone())?;
    let db_tables = db.get_all_table_ids()?;

    let info = db.get_info()?.clone();
    let res = DatabaseResponse::new(info, db_tables);

    Ok(web::Json(res))
}

#[get("/{db}")]
pub(crate) async fn get_info_handler(
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<impl Responder, Box<dyn Error>> {
    let db_path = path.into_inner();
    let params = web::Query::<DatabaseParams>::from_query(req.query_string()).unwrap();

    let db = get_db(&db_path, params.db_name.clone())?;
    let db_tables = db.get_all_table_ids()?;

    let info = db.get_info()?.clone();
    let res = DatabaseResponse::new(info, db_tables);

    Ok(web::Json(res))
}
