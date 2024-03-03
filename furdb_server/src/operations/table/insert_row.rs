use actix_web::{post, web, Responder};
use furdb_core::models as core_models;
use std::error::Error;

use crate::models;

#[post("/{database_id}/{table_id}/data")]
pub(crate) async fn insert_row_handler(
    path: web::Path<(String, String)>,
    insert_row_params: web::Json<models::insert_row_params::InsertRowParams>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();

    let database = core_models::database::Database::get_database(&database_id)?;
    let mut table = database.get_table(&table_id)?;

    table.insert_row(&insert_row_params.get_data())?;

    let res = models::blank_success_response::BlankSuccessResponse::new();

    Ok(web::Json(res))
}
