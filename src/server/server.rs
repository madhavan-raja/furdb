use std::error::Error;

use actix_web::{middleware, web, App, HttpServer};

use crate::core::furdb::FurDB;
use crate::server::server_config::ServerConfig;

use crate::server::operations;

#[derive(Clone)]
pub struct Server {
    server_config: ServerConfig,
    furdb: FurDB,
}

impl Server {
    pub fn new(server_config: ServerConfig, furdb: FurDB) -> Self {
        Self {
            server_config,
            furdb,
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        let server_config = self.server_config.to_owned();
        let furdb = self.furdb.to_owned();

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(furdb.to_owned()))
                .wrap(middleware::Logger::default())
                .service(operations::info::info::info)
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
}
