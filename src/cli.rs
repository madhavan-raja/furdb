use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use crate::core::FurDBConfig;
use crate::server::ServerConfig;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub furdb_config: FurDBConfig,

    /// Verbosity
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the server
    Serve(ServerConfig),
}
