use actix_web::{get, web, Responder};
use std::error::Error;

#[get("/health")]
pub(crate) async fn health() -> Result<impl Responder, Box<dyn Error>> {
    let res = { "Server is running." };
    Ok(web::Json(res))
}
