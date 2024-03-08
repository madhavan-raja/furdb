use actix_web::{post, web, Responder};
use furdb_core::models as core_models;
use std::error::Error;

use crate::models;

#[post("/{database_id}/{table_id}/data")]
pub(crate) async fn insert_rows_handler(
    path: web::Path<(String, String)>,
    insert_rows_params: web::Json<models::params::insert_rows_params::InsertRowsParams>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();

    let furdb = core_models::furdb::FurDB::new(core_models::config::Config::new(None)?)?;
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    table.insert_rows(&insert_rows_params.get_data())?;

    let res = models::response::blank_success_response::BlankSuccessResponse::new();

    Ok(web::Json(res))
}
