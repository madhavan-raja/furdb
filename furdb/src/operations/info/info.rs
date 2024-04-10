use actix_web::get;
use actix_web::web::Data;

use furdb_core::models::furdb::FurDB;

use crate::models::response::info::server_info_response::ServerInfoResponse;

use crate::models::response::error_response::ErrorResponse;
use crate::models::response::success_response::SuccessResponse;

#[get("/")]
pub async fn info(data: Data<FurDB>) -> Result<SuccessResponse, ErrorResponse> {
    let furdb = data.as_ref();
    let config = furdb.get_config();

    let response = ServerInfoResponse::new(&config);

    Ok(SuccessResponse::ServerInfo(response))
}
