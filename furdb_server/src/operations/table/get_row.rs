use actix_web::{get, web, Responder};
use furdb_core::models as core_models;
use std::error::Error;

use crate::models;

#[get("/{database_id}/{table_id}/data")]
pub(crate) async fn get_row_handler(
    path: web::Path<(String, String)>,
    get_row_params: web::Json<models::get_row_params::GetRowParams>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();

    let database = core_models::database::Database::get_database(&database_id)?;
    let mut table = database.get_table(&table_id)?;

    let row = table.get_row(get_row_params.get_index())?;

    let response = models::get_row_response::GetRowResponse::new(&row)?;

    Ok(web::Json(response))
}
