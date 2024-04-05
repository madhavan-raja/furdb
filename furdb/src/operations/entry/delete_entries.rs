use actix_web::{delete, web, Responder};
use furdb_core::models as core_models;
use std::error::Error;

use crate::models;

#[delete("/{database_id}/{table_id}/data")]
pub(crate) async fn delete_entries_handler(
    data: web::Data<core_models::furdb::FurDB>,
    path: web::Path<(String, String)>,
    delete_entries_params: web::Json<models::params::delete_entries_params::DeleteEntriesParams>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();
    let indices = delete_entries_params.get_indices();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    table.delete_entries(indices)?;

    let response = models::response::blank_success_response::BlankSuccessResponse::new();

    Ok(web::Json(response))
}
