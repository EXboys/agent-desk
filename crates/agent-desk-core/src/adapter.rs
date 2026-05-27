use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterDiscovery {
    pub installed: bool,
    pub version: Option<String>,
    pub binary_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeProfile {
    pub gateway_url: Option<String>,
    pub key_source: Option<String>,
}

pub trait RuntimeAdapter: Send + Sync {
    fn id(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
    fn discover(&self) -> AdapterDiscovery;
    fn config_paths(&self) -> Vec<PathBuf>;
    fn read_profile(&self) -> anyhow::Result<RuntimeProfile>;
}
