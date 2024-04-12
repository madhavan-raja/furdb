use actix_web::post;
use actix_web::web::{Data, Json, Path};

use crate::core::furdb::FurDB;

use crate::server::models::params::create_database_params::CreateDatabaseParams;

use crate::server::models::response::error_response::ErrorResponse;
use crate::server::models::response::success_response::SuccessResponse;

#[post("/{database_id}")]
pub async fn create_database_handler(
    data: Data<FurDB>,
    path: Path<String>,
    create_database_params: Json<CreateDatabaseParams>,
) -> Result<SuccessResponse, ErrorResponse> {
    let database_id = path.into_inner();
    let database_name = create_database_params.get_database_name();

    let furdb = data.as_ref();
    let database = furdb.create_database(&database_id, database_name.as_deref())?;

    let database_info = database.get_database_info();

    Ok(SuccessResponse::DatabaseCreated(database_info))
}
