use std::error::Error;

use actix_web::{get, web, App, HttpServer, Responder};

mod config;
mod database;
mod table;

#[get("/")]
pub(crate) async fn check() -> Result<impl Responder, Box<dyn Error>> {
    let res = { "FurDB" };
    Ok(web::Json(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(check)
            .service(database::get_info_handler)
            .service(table::get_info_handler)
            .service(table::get_data_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
