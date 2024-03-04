use actix_web::{get, web, Responder};
use furdb_core::models as core_models;
use std::error::Error;

use crate::models;

#[get("/{database_id}/{table_id}/data")]
pub(crate) async fn get_rows_handler(
    path: web::Path<(String, String)>,
    get_row_params: web::Json<models::get_rows_params::GetRowParams>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();
    let indices = get_row_params.get_indices();

    let database = core_models::database::Database::get_database(&database_id)?;
    let mut table = database.get_table(&table_id)?;

    let data = table.get_rows(indices)?;

    let response = models::get_row_response::GetRowResponse::new(&data)?;

    Ok(web::Json(response))
}
