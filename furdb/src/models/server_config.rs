use clap::Parser;

/// FurDB
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub(crate) struct ServerConfig {
    /// Port
    #[arg(short, long, env, default_value_t = 8080)]
    pub port: u16,

    /// Working Directory
    #[arg(short, long, env)]
    pub workdir: String,
}
