use actix_web::{get, web, Responder};
use furdb_core::models as core_models;

use crate::models::{self, response::error_response::ErrorResponse};

#[get("/health")]
pub(crate) async fn health(
    data: web::Data<core_models::furdb::FurDB>,
) -> Result<impl Responder, ErrorResponse> {
    let furdb = data.as_ref();
    let config = furdb.get_config();

    let response =
        models::response::info::server_health_response::ServerHealthResponse::new(&config);

    Ok(web::Json(response))
}
