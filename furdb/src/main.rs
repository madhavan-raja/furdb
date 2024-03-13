use actix_web::{App, HttpServer};
use std::error::Error;

mod models;
mod operations;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let port = std::env::var("FURDB_PORT")?.parse::<u16>()?;

    HttpServer::new(|| {
        App::new()
            .service(operations::info::health::health)
            .service(operations::database::create_database::create_database_handler)
            .service(operations::database::get_database::get_database_handler)
            .service(operations::database::delete_database::delete_database_handler)
            .service(operations::table::create_table::create_table_handler)
            .service(operations::table::get_table::get_table_handler)
            .service(operations::table::delete_table::delete_table_handler)
            .service(operations::table_row::insert_rows::insert_rows_handler)
            .service(operations::table_row::query::query_handler)
            .service(operations::table_row::query::get_rows_handler) // Remove later
            .service(operations::table_row::delete_rows::delete_rows_handler)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
