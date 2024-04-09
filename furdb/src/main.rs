use actix_web::{middleware, web, App, HttpServer};
use clap::Parser;
use furdb_core::models as core_models;
use std::error::Error;

mod errors;
mod models;
mod operations;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let server_config = models::server_config::ServerConfig::parse();

    env_logger::Builder::new()
        .filter_level(server_config.verbose.log_level_filter())
        .init();

    let config = core_models::config::Config::new(&server_config.workdir);
    let furdb = core_models::furdb::FurDB::new(&config)?;

    HttpServer::new(move || {
        let furdb = furdb.clone();

        App::new()
            .app_data(web::Data::new(furdb))
            .wrap(middleware::Logger::default())
            .service(operations::info::health::health)
            .service(operations::database::create_database::create_database_handler)
            .service(operations::database::get_database::get_database_handler)
            .service(operations::database::delete_database::delete_database_handler)
            .service(operations::table::create_table::create_table_handler)
            .service(operations::table::get_table::get_table_handler)
            .service(operations::table::delete_table::delete_table_handler)
            .service(operations::entry::insert_entries::insert_entries_handler)
            .service(operations::entry::get_entries::get_entries_handler)
            .service(operations::entry::delete_entries::delete_entries_handler)
    })
    .bind(("0.0.0.0", server_config.port))?
    .run()
    .await?;

    Ok(())
}
