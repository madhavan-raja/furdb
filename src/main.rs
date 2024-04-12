use std::error::Error;

use clap::Parser;

use core::furdb::FurDB;

use cli::Cli;
use cli::Commands;

use server::server::Server;

mod cli;
mod core;
mod server;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let args = Cli::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let furdb_config = args.furdb_config;
    let furdb = FurDB::new(&furdb_config)?;

    match args.command {
        Commands::Serve(server_config) => {
            let server = Server::new(server_config, furdb);
            server.start().await?;
        }
    }

    Ok(())
}
