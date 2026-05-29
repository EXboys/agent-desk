mod merge;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::profile::{agent_profile_path, write_company_profile};
use crate::runtime::all_adapters;

pub const COMPANY_API_KEY_ENV: &str = "AGENT_DOCTOR_COMPANY_API_KEY";
pub const GATEWAY_URL_ENV: &str = "AGENT_DOCTOR_GATEWAY_URL";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupOptions {
    pub gateway_url: String,
    pub api_key: String,
    /// Hermes provider when creating or updating config (default: openai).
    pub hermes_provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSetupResult {
    pub runtime_id: String,
    pub display_name: String,
    pub applied: bool,
    pub config_path: Option<String>,
    pub backup_path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupReport {
    pub profile_env_path: String,
    pub gateway_url: String,
    pub runtimes: Vec<RuntimeSetupResult>,
}

pub fn execute_setup(options: &SetupOptions) -> Result<SetupReport> {
    let gateway_url = normalize_gateway_url(&options.gateway_url)?;
    let api_key = options.api_key.trim();
    if api_key.is_empty() {
        bail!("--key must not be empty");
    }

    let profile_path = agent_profile_path().context("could not resolve config directory")?;
    write_company_profile(&profile_path, &gateway_url, api_key)?;

    let mut runtimes = Vec::new();
    for adapter in all_adapters() {
        let result = match adapter.id() {
            "openclaw" => merge::apply_openclaw(&gateway_url, api_key),
            "hermes" => merge::apply_hermes(&gateway_url, api_key, &options.hermes_provider),
            "claude-code" => merge::apply_claude_code(&gateway_url, api_key),
            "codex" => merge::apply_codex(&gateway_url, api_key),
            other => Ok(RuntimeSetupResult {
                runtime_id: other.to_string(),
                display_name: adapter.display_name().to_string(),
                applied: false,
                config_path: None,
                backup_path: None,
                message: "no company setup merge for this runtime yet".to_string(),
            }),
        }?;
        runtimes.push(result);
    }

    Ok(SetupReport {
        profile_env_path: profile_path.display().to_string(),
        gateway_url,
        runtimes,
    })
}

pub fn normalize_gateway_url(url: &str) -> Result<String> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        bail!("--url must not be empty");
    }
    if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
        bail!("--url must start with http:// or https://");
    }
    Ok(trimmed.trim_end_matches('/').to_string())
}

pub(crate) fn backup_file(path: &Path) -> Result<Option<PathBuf>> {
    if !path.exists() {
        return Ok(None);
    }
    let original = fs::read_to_string(path)?;
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let backup_path = path.with_extension(format!(
        "{}.bak.{ts}",
        path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("bak")
    ));
    std::fs::write(&backup_path, original)?;
    Ok(Some(backup_path))
}

pub(crate) fn ensure_parent(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_gateway_url() {
        assert_eq!(
            normalize_gateway_url("https://gw.example/v1/").unwrap(),
            "https://gw.example/v1"
        );
    }

    #[test]
    fn rejects_invalid_gateway_url() {
        assert!(normalize_gateway_url("").is_err());
        assert!(normalize_gateway_url("ftp://x").is_err());
    }
}
