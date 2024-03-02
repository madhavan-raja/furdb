use actix_web::{post, web, HttpRequest, Responder};
use furdb_core::Database;
use std::{error::Error, path::PathBuf};

mod utils;
use utils::{generate_table_info, get_db};

use crate::utils::get_database_path;

mod response;
use response::TableResponse;

mod params;
use params::TableParams;

mod request;
use request::TableGenerator;

#[post("/{database_id}/{table_id}")]
pub(crate) async fn create_table_handler(
    path: web::Path<(String, String)>,
    req: HttpRequest,
    table_generatable: Option<web::Json<TableGenerator>>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();
    let params = web::Query::<TableParams>::from_query(req.query_string()).unwrap();

    // let working_dir = params.working_dir.as_ref().map(|wd| PathBuf::from(wd));
    // let db = get_db(working_dir, &database_id, params.db_name.clone())?;

    let database = Database::get_database(get_database_path(&database_id))?;

    let table_info = table_generatable
        .map(|table_generatable| generate_table_info(table_generatable.clone()).unwrap());

    let tb = database.get_table(&table_id, table_info)?;

    let info = tb.get_info()?.clone();
    let res = TableResponse::new(info);

    Ok(web::Json(res))
}
