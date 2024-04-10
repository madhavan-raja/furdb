use actix_web::post;
use actix_web::web::{Data, Json, Path};

use furdb_core::models::furdb::FurDB;

use crate::models::params::create_table_params::CreateTableParams;

use crate::models::response::error_response::ErrorResponse;
use crate::models::response::success_response::SuccessResponse;

#[post("/{database_id}/{table_id}")]
pub async fn create_table_handler(
    data: Data<FurDB>,
    path: Path<(String, String)>,
    create_table_params: Json<CreateTableParams>,
) -> Result<SuccessResponse, ErrorResponse> {
    let (database_id, table_id) = path.into_inner();

    let table_name = create_table_params.get_table_name();
    let table_columns = create_table_params.get_table_columns();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;

    database.create_table(&table_id, table_name.as_deref(), table_columns.to_vec())?;

    Ok(SuccessResponse::TableCreated)
}
