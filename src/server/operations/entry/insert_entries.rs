use actix_web::post;
use actix_web::web::{Data, Json, Path};

use crate::core::FurDB;

use crate::server::models::params::InsertEntriesParams;

use crate::server::models::response::ErrorResponse;
use crate::server::models::response::SuccessResponse;

#[post("/{database_id}/{table_id}/data")]
pub async fn insert_entries_handler(
    data: Data<FurDB>,
    path: Path<(String, String)>,
    insert_entries_params: Json<InsertEntriesParams>,
) -> Result<SuccessResponse, ErrorResponse> {
    let (database_id, table_id) = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    table.insert_entries(&insert_entries_params.get_data())?;

    Ok(SuccessResponse::EntriesCreated)
}
