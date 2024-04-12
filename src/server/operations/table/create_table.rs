use actix_web::post;
use actix_web::web::{Data, Json, Path};

use crate::core::furdb::FurDB;

use crate::server::models::params::create_table_params::CreateTableParams;

use crate::server::models::response::error_response::ErrorResponse;
use crate::server::models::response::success_response::SuccessResponse;

#[post("/{database_id}/{table_id}")]
pub async fn create_table_handler(
    data: Data<FurDB>,
    path: Path<(String, String)>,
    create_table_params: Json<CreateTableParams>,
) -> Result<SuccessResponse, ErrorResponse> {
    let (database_id, table_id) = path.into_inner();

    let table_columns = create_table_params.get_table_columns();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;

    let table = database.create_table(&table_id, table_columns.to_vec())?;

    let table_info = table.get_table_info();

    Ok(SuccessResponse::TableCreated(table_info))
}
