use actix_web::{post, web};
use furdb_core::models as core_models;

use crate::models::{self, response::api_response::ApiResponse};

#[post("/{database_id}/{table_id}")]
pub async fn create_table_handler(
    data: web::Data<core_models::furdb::FurDB>,
    path: web::Path<(String, String)>,
    create_table_params: web::Json<models::params::create_table_params::CreateTableParams>,
) -> Result<models::response::api_response::ApiResponse, models::response::api_response::ApiResponse>
{
    let (database_id, table_id) = path.into_inner();

    let table_name = create_table_params.get_table_name();
    let table_columns = create_table_params.get_table_columns();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;

    database.create_table(&table_id, table_name.as_deref(), table_columns.to_vec())?;

    Ok(ApiResponse::Success(
        models::response::success_response::SuccessResponse::TableCreated,
    ))
}
