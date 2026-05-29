use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::setup::{COMPANY_API_KEY_ENV, GATEWAY_URL_ENV};

/// Default company/agent profile env file written by `agent-doctor setup`.
pub fn agent_profile_path() -> Option<PathBuf> {
    dirs::config_dir().map(|base| base.join("agent-doctor").join("profile.env"))
}

#[derive(Debug, Clone, Default)]
pub struct CompanyProfile {
    pub gateway_url: Option<String>,
    pub api_key: Option<String>,
}

pub fn write_company_profile(path: &Path, gateway_url: &str, api_key: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::File::create(path).context("failed to create profile.env")?;
    writeln!(
        file,
        "# Agent Doctor company profile — written by agent-doctor setup"
    )?;
    writeln!(
        file,
        "# Source before running agents: set -a && source \"{}\" && set +a",
        path.display()
    )?;
    writeln!(file, "{GATEWAY_URL_ENV}={gateway_url}")?;
    writeln!(file, "{COMPANY_API_KEY_ENV}={api_key}")?;
    writeln!(file, "COMPANY_API_KEY={api_key}")?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    }

    Ok(())
}

pub fn read_company_profile() -> Result<Option<CompanyProfile>> {
    let Some(path) = agent_profile_path() else {
        return Ok(None);
    };
    if !path.exists() {
        return Ok(None);
    }

    let raw = fs::read_to_string(&path)?;
    let mut profile = CompanyProfile::default();
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match key.trim() {
            GATEWAY_URL_ENV => profile.gateway_url = Some(value.trim().to_string()),
            COMPANY_API_KEY_ENV => profile.api_key = Some(value.trim().to_string()),
            _ => {}
        }
    }

    if profile.gateway_url.is_none() && profile.api_key.is_none() {
        Ok(None)
    } else {
        Ok(Some(profile))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn writes_and_reads_company_profile() {
        let temp = TempDir::new().expect("tempdir");
        let path = temp.path().join("profile.env");
        write_company_profile(&path, "https://gateway.example/v1", "sk-test").unwrap();
        let profile = read_company_profile_from_path(&path).unwrap();
        assert_eq!(
            profile.gateway_url.as_deref(),
            Some("https://gateway.example/v1")
        );
        assert_eq!(profile.api_key.as_deref(), Some("sk-test"));
    }

    fn read_company_profile_from_path(path: &Path) -> Result<CompanyProfile> {
        let raw = fs::read_to_string(path)?;
        let mut profile = CompanyProfile::default();
        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            match key.trim() {
                GATEWAY_URL_ENV => profile.gateway_url = Some(value.trim().to_string()),
                COMPANY_API_KEY_ENV => profile.api_key = Some(value.trim().to_string()),
                _ => {}
            }
        }
        Ok(profile)
    }
}
