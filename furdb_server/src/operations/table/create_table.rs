use actix_web::{post, web, Responder};
use furdb_core::models as core_models;
use std::error::Error;

use crate::models;

#[post("/{database_id}/{table_id}")]
pub async fn create_table_handler(
    path: web::Path<(String, String)>,
    create_table_params: web::Json<models::CreateTableParams>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();

    let database = core_models::database::Database::get_database(&database_id)?;

    let table_name = &create_table_params.table_name;
    let table_columns = &create_table_params.table_columns;

    database.create_table(&table_id, table_name.as_deref(), table_columns.to_vec())?;

    let res = models::BlankSuccessResponse::new();

    Ok(web::Json(res))
}
