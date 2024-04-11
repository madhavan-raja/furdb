use clap::Args;

/// Server
#[derive(Args)]
pub struct ServerConfig {
    /// Port
    #[arg(short, long, env, default_value_t = 5678)]
    pub port: u16,
}
