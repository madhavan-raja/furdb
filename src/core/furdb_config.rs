use serde::{Deserialize, Serialize};

use clap::Args;
use std::path::PathBuf;

/// FurDB
#[derive(Args, Clone, Serialize, Deserialize)]
pub struct FurDBConfig {
    /// Working Directory
    #[arg(short, long, env)]
    pub workdir: PathBuf,
}
