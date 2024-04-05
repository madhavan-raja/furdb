use clap::Parser;
use clap_verbosity_flag::{Verbosity, InfoLevel};

/// FurDB
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub(crate) struct ServerConfig {
    /// Port
    #[arg(short, long, env, default_value_t = 80)]
    pub port: u16,

    /// Working Directory
    #[arg(short, long, env)]
    pub workdir: String,

    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}
