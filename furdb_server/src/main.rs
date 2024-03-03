use actix_web::{get, web, App, HttpServer, Responder};
use std::error::Error;

mod models;
mod operations;

mod api_response;

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
            .service(operations::create_database_handler)
            .service(operations::get_database_info_handler)
            .service(operations::create_table_handler)
            .service(operations::get_table_handler)
            .service(operations::insert_row_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
