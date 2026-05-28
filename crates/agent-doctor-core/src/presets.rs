use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::adapter::{ApplyReport, RuntimeModelPreset};
use crate::adapters::{adapter_by_id, all_adapters, HermesAdapter};

const PROFILES_FILE: &str = "profiles.yaml";
const OLLAMA_DEFAULT_BASE_URL: &str = "http://127.0.0.1:11434/v1";
const OLLAMA_DEFAULT_MODEL: &str = "llama3.2";

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
    /// Quick-switch model options within this scene (layer 2).
    #[serde(default)]
    pub models: Vec<HermesProfilePreset>,
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

pub fn default_local_hermes_preset() -> HermesProfilePreset {
    HermesProfilePreset {
        provider: "ollama".to_string(),
        model: OLLAMA_DEFAULT_MODEL.to_string(),
        base_url: OLLAMA_DEFAULT_BASE_URL.to_string(),
    }
}

pub fn default_work_models() -> Vec<HermesProfilePreset> {
    vec![
        HermesProfilePreset {
            provider: "deepseek".to_string(),
            model: "deepseek-v4-flash".to_string(),
            base_url: "https://api.deepseek.com/v1".to_string(),
        },
        HermesProfilePreset {
            provider: "openai".to_string(),
            model: "gpt-4o".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
        },
        HermesProfilePreset {
            provider: "anthropic".to_string(),
            model: "claude-sonnet-4-20250514".to_string(),
            base_url: "https://api.anthropic.com/v1".to_string(),
        },
    ]
}

pub fn effective_models(entry: &ProfileEntry) -> Vec<HermesProfilePreset> {
    if !entry.models.is_empty() {
        return entry.models.clone();
    }
    entry.hermes.clone().into_iter().collect()
}

fn ensure_profile_models(entry: &mut ProfileEntry, defaults: Vec<HermesProfilePreset>) -> bool {
    if entry.models.is_empty() && !defaults.is_empty() {
        entry.models = defaults;
        return true;
    }
    false
}

/// Adds built-in presets that may be missing from older profiles files.
pub fn merge_builtin_profiles(doc: &mut ProfilesDocument) -> bool {
    let mut changed = false;
    if !doc.profiles.contains_key("local") {
        doc.profiles.insert(
            "local".to_string(),
            ProfileEntry {
                hermes: Some(default_local_hermes_preset()),
                models: vec![default_local_hermes_preset()],
            },
        );
        changed = true;
    }
    if let Some(entry) = doc.profiles.get_mut("work") {
        changed |= ensure_profile_models(entry, default_work_models());
    }
    if let Some(entry) = doc.profiles.get_mut("personal") {
        changed |= ensure_profile_models(
            entry,
            vec![HermesProfilePreset {
                provider: "openai".to_string(),
                model: "gpt-4o".to_string(),
                base_url: "https://api.openai.com/v1".to_string(),
            }],
        );
    }
    if let Some(entry) = doc.profiles.get_mut("local") {
        changed |= ensure_profile_models(entry, vec![default_local_hermes_preset()]);
    }
    changed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseProfileReport {
    pub profile: String,
    pub applied: Vec<ApplyReport>,
    pub skipped: Vec<String>,
}

pub fn profiles_path() -> Result<PathBuf> {
    dirs::config_dir()
        .map(|dir| dir.join("agent-doctor").join(PROFILES_FILE))
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
    let mut doc: ProfilesDocument = serde_yaml::from_str(&raw)
        .with_context(|| format!("failed to parse {}", path.display()))?;
    if merge_builtin_profiles(&mut doc) {
        save_profiles(&doc)?;
    }
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
            models: default_work_models(),
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
            models: vec![HermesProfilePreset {
                provider: "openai".to_string(),
                model: "gpt-4o".to_string(),
                base_url: "https://api.openai.com/v1".to_string(),
            }],
        },
    );
    profiles.insert(
        "local".to_string(),
        ProfileEntry {
            hermes: Some(default_local_hermes_preset()),
            models: vec![default_local_hermes_preset()],
        },
    );

    let doc = ProfilesDocument {
        active: Some("local".to_string()),
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

/// Switch Hermes model within the active scene without changing the active profile name.
pub fn apply_profile_model(profile: &str, preset: HermesProfilePreset) -> Result<ApplyReport> {
    let mut doc = load_profiles()?;
    let entry = doc
        .profiles
        .get_mut(profile)
        .with_context(|| format!("profile '{profile}' not found"))?;

    let report = set_runtime_model("hermes", preset.clone().into(), None)?;
    entry.hermes = Some(preset.clone());
    let exists = entry.models.iter().any(|item| {
        item.provider == preset.provider
            && item.model == preset.model
            && item.base_url == preset.base_url
    });
    if !exists {
        entry.models.push(preset);
    }
    save_profiles(&doc)?;
    Ok(report)
}

pub fn show_config(runtime_id: &str) -> Result<String> {
    let adapter =
        adapter_by_id(runtime_id).with_context(|| format!("unknown runtime '{runtime_id}'"))?;
    let model = adapter
        .read_model()?
        .context(format!("{} does not expose model settings yet", runtime_id))?;
    Ok(serde_json::to_string_pretty(&model)?)
}

pub fn set_runtime_model(
    runtime_id: &str,
    preset: RuntimeModelPreset,
    api_key: Option<&str>,
) -> Result<ApplyReport> {
    let adapter =
        adapter_by_id(runtime_id).with_context(|| format!("unknown runtime '{runtime_id}'"))?;
    if !adapter.discover().installed {
        anyhow::bail!("{} is not installed", adapter.display_name());
    }
    if runtime_id == "hermes" {
        return HermesAdapter.apply_settings(&preset, api_key);
    }
    if api_key.is_some() {
        anyhow::bail!(
            "{} does not support API key updates yet",
            adapter.display_name()
        );
    }
    adapter.apply_model(&preset)
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
        assert!(doc.profiles.contains_key("local"));
        assert_eq!(doc.active.as_deref(), Some("local"));
    }

    #[test]
    fn merge_builtin_profiles_adds_local_and_work_models() {
        let mut doc = ProfilesDocument {
            active: Some("work".to_string()),
            profiles: BTreeMap::from([(
                "work".to_string(),
                ProfileEntry {
                    hermes: Some(HermesProfilePreset {
                        provider: "deepseek".to_string(),
                        model: "deepseek-v4-flash".to_string(),
                        base_url: "https://api.deepseek.com/v1".to_string(),
                    }),
                    models: Vec::new(),
                },
            )]),
        };
        assert!(merge_builtin_profiles(&mut doc));
        let work = doc.profiles.get("work").unwrap();
        assert_eq!(work.models.len(), 3);
        let local = doc.profiles.get("local").unwrap().hermes.as_ref().unwrap();
        assert_eq!(local.provider, "ollama");
        assert_eq!(local.base_url, OLLAMA_DEFAULT_BASE_URL);
    }
}
