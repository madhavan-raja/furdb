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
            .service(operations::entry::insert_entries::insert_entries_handler)
            .service(operations::entry::query::query_handler)
            .service(operations::entry::query::get_entries_handler) // Remove later
            .service(operations::entry::delete_entries::delete_entries_handler)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
