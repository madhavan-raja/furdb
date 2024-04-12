use actix_web::get;
use actix_web::web::Data;

use crate::core::furdb::FurDB;

use crate::server::models::response::error_response::ErrorResponse;
use crate::server::models::response::success_response::SuccessResponse;

#[get("/")]
pub async fn info(data: Data<FurDB>) -> Result<SuccessResponse, ErrorResponse> {
    let furdb = data.as_ref();
    let furdb_config = furdb.get_config();

    Ok(SuccessResponse::ServerInfo(furdb_config))
}
