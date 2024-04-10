use clap::Args;

/// FurDB
#[derive(Args)]
pub struct ServerConfig {
    /// Port
    #[arg(short, long, env, default_value_t = 80)]
    pub port: u16,
}
