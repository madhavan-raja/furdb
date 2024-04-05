use actix_web::{get, web, Responder};
use furdb_core::models as core_models;
use std::error::Error;

use crate::models;

#[get("/health")]
pub(crate) async fn health(
    data: web::Data<core_models::furdb::FurDB>,
) -> Result<impl Responder, Box<dyn Error>> {
    let furdb = data.as_ref();
    let config = furdb.get_config();

    let response = models::response::server_health_response::ServerHealthResponse::new(&config)?;

    Ok(web::Json(response))
}
