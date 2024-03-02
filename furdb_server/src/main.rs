use std::error::Error;

use actix_web::{get, web, App, HttpServer, Responder};

mod api_response;
mod config;
mod database;
mod table;
mod utils;

#[get("/")]
pub(crate) async fn check() -> Result<impl Responder, Box<dyn Error>> {
    let res = { "FurDB" };
    Ok(web::Json(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(check)
            .service(database::create_database_handler)
            .service(database::get_database_info_handler)
            .service(table::create_table_handler)
            .service(table::get_table_info_handler)
            .service(table::get_data_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
