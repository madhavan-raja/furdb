use actix_web::{delete, web, Responder};
use furdb_core::models as core_models;
use std::error::Error;

use crate::models;

#[delete("/{database_id}/{table_id}/data")]
pub(crate) async fn delete_rows_handler(
    path: web::Path<(String, String)>,
    delete_rows_params: web::Json<models::delete_rows_params::DeleteRowsParams>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();
    let indices = delete_rows_params.get_indices();

    let database = core_models::database::Database::get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    table.delete_rows(indices)?;

    let response = models::blank_success_response::BlankSuccessResponse::new();

    Ok(web::Json(response))
}
