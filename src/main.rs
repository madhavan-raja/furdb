use std::error::Error;

use clap::Parser;

use core::furdb_config::FurDBConfig;
use core::models::furdb::FurDB;

use cli::Cli;
use cli::Commands;

use server::start_server::start_server;

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

    let config = FurDBConfig::new(&args.workdir);
    let furdb = FurDB::new(&config)?;

    match args.command {
        Commands::Serve(server_config) => {
            start_server(server_config, furdb).await?;
        }
    }

    Ok(())
}
