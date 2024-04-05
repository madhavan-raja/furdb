use actix_web::{App, HttpServer};
use std::error::Error;

mod models;
mod operations;

use clap::Parser;

/// FurDB
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port
    #[arg(short, long, env, default_value_t = 8080)]
    port: u16,

    /// Working Directory
    #[arg(short, long, env)]
    workdir: String,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let args = Args::parse();

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
            .service(operations::entry::query::get_entries_handler)
            .service(operations::entry::delete_entries::delete_entries_handler)
    })
    .bind(("0.0.0.0", args.port))?
    .run()
    .await?;

    Ok(())
}
