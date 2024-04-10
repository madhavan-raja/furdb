use actix_web::get;
use actix_web::web::{Data, Path};

use crate::core::models::furdb::FurDB;

use crate::server::models::response::table::get_table_response::GetTableResponse;

use crate::server::models::response::error_response::ErrorResponse;
use crate::server::models::response::success_response::SuccessResponse;

#[get("/{database_id}/{table_id}")]
pub async fn get_table_handler(
    data: Data<FurDB>,
    path: Path<(String, String)>,
) -> Result<SuccessResponse, ErrorResponse> {
    let (database_id, table_id) = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    let response = GetTableResponse::new(&table);

    Ok(SuccessResponse::TableInfo(response))
}
