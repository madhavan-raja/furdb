use actix_web::{middleware, web, App, HttpServer};

use crate::core::FurDB;
use crate::server::server_config::ServerConfig;

use crate::server::operations;

use crate::error::ApplicationError;

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

    pub async fn start(&self) -> Result<(), ApplicationError> {
        let server_config = self.server_config.to_owned();
        let furdb = self.furdb.to_owned();

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(furdb.to_owned()))
                .wrap(middleware::Logger::default())
                .service(operations::info_handler)
                .service(operations::create_database_handler)
                .service(operations::get_database_handler)
                .service(operations::delete_database_handler)
                .service(operations::create_table_handler)
                .service(operations::get_table_handler)
                .service(operations::delete_table_handler)
                .service(operations::insert_entries_handler)
                .service(operations::get_entries_handler)
                .service(operations::delete_entries_handler)
        })
        .bind(("0.0.0.0", server_config.port))
        .map_err(|_| ApplicationError::ServerStart)?
        .run()
        .await
        .map_err(|e| ApplicationError::Other(e.to_string()))?;

        Ok(())
    }
}
