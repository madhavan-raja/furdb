use actix_web::{get, web};
use furdb_core::models as core_models;

use crate::models::{
    self,
    response::{api_response::ApiResponse, success_response::SuccessResponse},
};

#[get("/health")]
pub async fn health(
    data: web::Data<core_models::furdb::FurDB>,
) -> Result<models::response::api_response::ApiResponse, models::response::api_response::ApiResponse>
{
    let furdb = data.as_ref();
    let config = furdb.get_config();

    let response =
        models::response::info::server_health_response::ServerHealthResponse::new(&config);

    Ok(ApiResponse::Success(SuccessResponse::ServerHealth(
        response,
    )))
}
