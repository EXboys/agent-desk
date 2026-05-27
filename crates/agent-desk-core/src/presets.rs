use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::adapter::{ApplyReport, RuntimeModelPreset};
use crate::adapters::{adapter_by_id, all_adapters};

const PROFILES_FILE: &str = "profiles.yaml";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfilesDocument {
    #[serde(default)]
    pub active: Option<String>,
    #[serde(default)]
    pub profiles: BTreeMap<String, ProfileEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileEntry {
    #[serde(default)]
    pub hermes: Option<HermesProfilePreset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HermesProfilePreset {
    pub provider: String,
    pub model: String,
    pub base_url: String,
}

impl From<HermesProfilePreset> for RuntimeModelPreset {
    fn from(value: HermesProfilePreset) -> Self {
        RuntimeModelPreset {
            provider: value.provider,
            model: value.model,
            base_url: value.base_url,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseProfileReport {
    pub profile: String,
    pub applied: Vec<ApplyReport>,
    pub skipped: Vec<String>,
}

pub fn profiles_path() -> Result<PathBuf> {
    dirs::config_dir()
        .map(|dir| dir.join("agent-desk").join(PROFILES_FILE))
        .context("could not resolve config directory")
}

pub fn load_profiles() -> Result<ProfilesDocument> {
    let path = profiles_path()?;
    if !path.exists() {
        return Ok(ProfilesDocument {
            active: None,
            profiles: BTreeMap::new(),
        });
    }

    let raw = fs::read_to_string(&path)?;
    let doc: ProfilesDocument = serde_yaml::from_str(&raw)
        .with_context(|| format!("failed to parse {}", path.display()))?;
    Ok(doc)
}

pub fn save_profiles(doc: &ProfilesDocument) -> Result<PathBuf> {
    let path = profiles_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let raw = serde_yaml::to_string(doc)?;
    fs::write(&path, raw)?;
    Ok(path)
}

pub fn init_example_profiles() -> Result<PathBuf> {
    let path = profiles_path()?;
    if path.exists() {
        bail!(
            "profiles file already exists at {} — remove it first or edit manually",
            path.display()
        );
    }

    let mut profiles = BTreeMap::new();
    profiles.insert(
        "work".to_string(),
        ProfileEntry {
            hermes: Some(HermesProfilePreset {
                provider: "deepseek".to_string(),
                model: "deepseek-v4-flash".to_string(),
                base_url: "https://api.deepseek.com/v1".to_string(),
            }),
        },
    );
    profiles.insert(
        "personal".to_string(),
        ProfileEntry {
            hermes: Some(HermesProfilePreset {
                provider: "openai".to_string(),
                model: "gpt-4o".to_string(),
                base_url: "https://api.openai.com/v1".to_string(),
            }),
        },
    );

    let doc = ProfilesDocument {
        active: Some("work".to_string()),
        profiles,
    };
    save_profiles(&doc)
}

pub fn use_profile(name: &str) -> Result<UseProfileReport> {
    let mut doc = load_profiles()?;
    let entry = doc
        .profiles
        .get(name)
        .with_context(|| format!("profile '{name}' not found"))?
        .clone();

    doc.active = Some(name.to_string());
    save_profiles(&doc)?;

    let mut applied = Vec::new();
    let mut skipped = Vec::new();

    if let Some(hermes) = entry.hermes {
        match adapter_by_id("hermes") {
            Some(adapter) => {
                let discovery = adapter.discover();
                if !discovery.installed {
                    skipped.push("hermes: not installed".to_string());
                } else {
                    applied.push(adapter.apply_model(&hermes.into())?);
                }
            }
            None => skipped.push("hermes: adapter missing".to_string()),
        }
    }

    for adapter in all_adapters() {
        if adapter.id() == "hermes" {
            continue;
        }
        if !adapter.discover().installed {
            continue;
        }
        skipped.push(format!("{}: no preset in profile '{name}'", adapter.id()));
    }

    Ok(UseProfileReport {
        profile: name.to_string(),
        applied,
        skipped,
    })
}

pub fn show_config(runtime_id: &str) -> Result<String> {
    let adapter =
        adapter_by_id(runtime_id).with_context(|| format!("unknown runtime '{runtime_id}'"))?;
    let model = adapter
        .read_model()?
        .context(format!("{} does not expose model settings yet", runtime_id))?;
    Ok(serde_json::to_string_pretty(&model)?)
}

pub fn ensure_profiles_dir() -> Result<PathBuf> {
    let path = profiles_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(path)
}

pub fn default_profiles_example_path() -> &'static str {
    "docs/examples/profiles.example.yaml"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example_profiles_document() {
        let raw = include_str!("../../../docs/examples/profiles.example.yaml");
        let doc: ProfilesDocument = serde_yaml::from_str(raw).expect("parse example");
        assert!(doc.profiles.contains_key("work"));
        assert_eq!(doc.active.as_deref(), Some("work"));
    }
}
