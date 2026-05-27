use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use serde_yaml::{Mapping, Value};

use crate::adapter::{
    AdapterDiscovery, ApplyReport, RuntimeAdapter, RuntimeModelPreset, RuntimeModelState,
    RuntimeProfile,
};
use crate::adapters::util::{discover_binary, home_join};

pub struct HermesAdapter;

impl HermesAdapter {
    fn config_path() -> PathBuf {
        home_join(".hermes/config.yaml")
    }

    fn load_config_value(path: &PathBuf) -> Result<Value> {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        serde_yaml::from_str(&raw).with_context(|| format!("failed to parse {}", path.display()))
    }
}

impl RuntimeAdapter for HermesAdapter {
    fn id(&self) -> &'static str {
        "hermes"
    }

    fn display_name(&self) -> &'static str {
        "Hermes Agent"
    }

    fn discover(&self) -> AdapterDiscovery {
        discover_binary("hermes")
    }

    fn config_paths(&self) -> Vec<PathBuf> {
        vec![Self::config_path()]
    }

    fn read_profile(&self) -> Result<RuntimeProfile> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(RuntimeProfile {
                gateway_url: None,
                key_source: None,
            });
        }

        let model = self.read_model()?.unwrap_or(RuntimeModelState {
            provider: None,
            model: None,
            base_url: None,
        });

        Ok(RuntimeProfile {
            gateway_url: model.base_url,
            key_source: Some(path.display().to_string()),
        })
    }

    fn read_model(&self) -> Result<Option<RuntimeModelState>> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(None);
        }

        let value = Self::load_config_value(&path)?;
        let model = value.get("model");
        Ok(Some(RuntimeModelState {
            provider: model
                .and_then(|m| m.get("provider"))
                .and_then(Value::as_str)
                .map(str::to_string),
            model: model
                .and_then(|m| m.get("default"))
                .and_then(Value::as_str)
                .map(str::to_string),
            base_url: model
                .and_then(|m| m.get("base_url"))
                .and_then(Value::as_str)
                .filter(|url| !url.is_empty())
                .map(str::to_string),
        }))
    }

    fn apply_model(&self, preset: &RuntimeModelPreset) -> Result<ApplyReport> {
        let path = Self::config_path();
        if !path.exists() {
            anyhow::bail!(
                "Hermes config not found at {} — run Hermes once or create the file first",
                path.display()
            );
        }

        let raw = fs::read_to_string(&path)?;
        let mut root = Self::load_config_value(&path)?;
        let model = root
            .as_mapping_mut()
            .context("Hermes config root must be a mapping")?;

        let model_map = model
            .entry(Value::from("model"))
            .or_insert_with(|| Value::Mapping(Mapping::new()));
        let model_map = model_map
            .as_mapping_mut()
            .context("Hermes config model section must be a mapping")?;

        model_map.insert(
            Value::from("provider"),
            Value::from(preset.provider.as_str()),
        );
        model_map.insert(Value::from("default"), Value::from(preset.model.as_str()));
        model_map.insert(
            Value::from("base_url"),
            Value::from(preset.base_url.as_str()),
        );

        let backup_path = backup_config(&path, &raw)?;
        let updated = serde_yaml::to_string(&root)?;
        fs::write(&path, updated)?;

        Ok(ApplyReport {
            runtime_id: self.id().to_string(),
            config_path: path.display().to_string(),
            backup_path: Some(backup_path.display().to_string()),
            restart_hint: "Restart Hermes or start a new session for model changes to take effect."
                .to_string(),
        })
    }
}

fn backup_config(path: &Path, original: &str) -> Result<PathBuf> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system time before UNIX epoch")?
        .as_secs();
    let backup_path = path.with_extension(format!("yaml.bak.{ts}"));
    fs::write(&backup_path, original)?;
    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn apply_model_updates_hermes_yaml() {
        let temp = TempDir::new().expect("tempdir");
        let config_path = temp.path().join("config.yaml");
        fs::write(
            &config_path,
            "model:\n  default: old-model\n  provider: old\n  base_url: https://old.example/v1\n",
        )
        .unwrap();

        let preset = RuntimeModelPreset {
            provider: "deepseek".to_string(),
            model: "deepseek-v4-flash".to_string(),
            base_url: "https://api.deepseek.com/v1".to_string(),
        };

        let mut root: Value =
            serde_yaml::from_str(&fs::read_to_string(&config_path).unwrap()).unwrap();
        let model = root.as_mapping_mut().unwrap();
        let model_map = model
            .entry(Value::from("model"))
            .or_insert_with(|| Value::Mapping(Mapping::new()));
        let model_map = model_map.as_mapping_mut().unwrap();
        model_map.insert(
            Value::from("provider"),
            Value::from(preset.provider.as_str()),
        );
        model_map.insert(Value::from("default"), Value::from(preset.model.as_str()));
        model_map.insert(
            Value::from("base_url"),
            Value::from(preset.base_url.as_str()),
        );
        fs::write(&config_path, serde_yaml::to_string(&root).unwrap()).unwrap();

        let updated = fs::read_to_string(&config_path).unwrap();
        assert!(updated.contains("deepseek-v4-flash"));
        assert!(updated.contains("https://api.deepseek.com/v1"));
    }
}
