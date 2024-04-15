use actix_web::get;
use actix_web::web::{Data, Path};

use crate::core::FurDB;

use crate::server::models::response::ErrorResponse;
use crate::server::models::response::SuccessResponse;

#[get("/{database_id}/{table_id}")]
pub async fn get_table_handler(
    data: Data<FurDB>,
    path: Path<(String, String)>,
) -> Result<SuccessResponse, ErrorResponse> {
    let (database_id, table_id) = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    let table_info = table.get_table_info();

    Ok(SuccessResponse::TableInfo(table_info))
}
