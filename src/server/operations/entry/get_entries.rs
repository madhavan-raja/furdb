use actix_web::get;
use actix_web::web::{Data, Json, Path};

use crate::core::FurDB;

use crate::server::models::params::{GetEntriesParams, GetEntriesType};

use crate::server::models::response::ErrorResponse;
use crate::server::models::response::SuccessResponse;

#[get("/{database_id}/{table_id}/data")]
pub async fn get_entries_handler(
    data: Data<FurDB>,
    path: Path<(String, String)>,
    get_entries_params: Json<GetEntriesParams>,
) -> Result<SuccessResponse, ErrorResponse> {
    let (database_id, table_id) = path.into_inner();

    let furdb = data.as_ref();
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    let entries_result = match &get_entries_params.get_entries() {
        GetEntriesType::All => table.get_all_entries(),
        GetEntriesType::Indices(indices) => table.get_entries(indices.to_vec()),
        GetEntriesType::Value(get_entries_by_value_params) => table.query(
            get_entries_by_value_params.get_column_index(),
            get_entries_by_value_params.get_value(),
        ),
    }?;

    Ok(SuccessResponse::EntriesResult(entries_result))
}
