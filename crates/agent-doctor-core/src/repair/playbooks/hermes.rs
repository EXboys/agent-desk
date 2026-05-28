use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde_yaml::{Mapping, Value};

use crate::adapters::util::home_join;
use crate::presets::{load_profiles, HermesProfilePreset};
use crate::probe::{ProbeStatus, RuntimeProbeReport};
use crate::repair::{SkippedRepairAction, SuggestedRepair};

#[derive(Debug, Default)]
pub struct PlaybookApplyResult {
    pub executed: Vec<String>,
    pub skipped: Vec<SkippedRepairAction>,
}

pub fn suggest_hermes_repairs(probe: &RuntimeProbeReport) -> Vec<SuggestedRepair> {
    let mut items = Vec::new();
    let has_profile = active_hermes_preset().is_ok();

    for check in &probe.checks {
        if check.id.starts_with("hermes.env.permissions:") && check.status == ProbeStatus::Warn {
            items.push(SuggestedRepair {
                id: "fix-hermes-env-permissions".to_string(),
                title: "Tighten ~/.hermes/.env permissions".to_string(),
                description: "Set .env to mode 600.".to_string(),
                auto_fixable: cfg!(unix),
            });
        }

        if check.id == "hermes.api_key.duplicates" && check.status == ProbeStatus::Warn {
            items.push(SuggestedRepair {
                id: "fix-hermes-api-key-duplicates".to_string(),
                title: "Deduplicate API key env entries".to_string(),
                description: "Keep the last non-empty API key assignment.".to_string(),
                auto_fixable: true,
            });
        }

        if check.id.starts_with("config.schema:")
            && check.status == ProbeStatus::Warn
            && check.message.contains("model.")
        {
            items.push(SuggestedRepair {
                id: "fix-hermes-config-from-profile".to_string(),
                title: "Fill Hermes model fields from active profile".to_string(),
                description: if has_profile {
                    "Apply provider, model, and base_url from the active Agent Doctor preset."
                        .to_string()
                } else {
                    "Run `agent-doctor profile init` and `profile use <name>` first.".to_string()
                },
                auto_fixable: has_profile,
            });
        }

        if check.id == "hermes.api_key.configured" && check.status == ProbeStatus::Warn {
            items.push(SuggestedRepair {
                id: "fix-hermes-api-key-manual".to_string(),
                title: "Configure API key in ~/.hermes/.env".to_string(),
                description: check.message.clone(),
                auto_fixable: false,
            });
        }
    }

    items
}

pub fn apply_hermes_playbook(probe: &RuntimeProbeReport) -> Result<PlaybookApplyResult> {
    let mut result = PlaybookApplyResult::default();

    for check in &probe.checks {
        if check.id.starts_with("hermes.env.permissions:") && check.status == ProbeStatus::Warn {
            match tighten_env_permissions() {
                Ok(()) => result
                    .executed
                    .push("fix-hermes-env-permissions".to_string()),
                Err(error) => result.skipped.push(SkippedRepairAction {
                    id: "fix-hermes-env-permissions".to_string(),
                    reason: error.to_string(),
                }),
            }
        }

        if check.id == "hermes.api_key.duplicates" && check.status == ProbeStatus::Warn {
            match dedupe_api_key_env(probe) {
                Ok(()) => result
                    .executed
                    .push("fix-hermes-api-key-duplicates".to_string()),
                Err(error) => result.skipped.push(SkippedRepairAction {
                    id: "fix-hermes-api-key-duplicates".to_string(),
                    reason: error.to_string(),
                }),
            }
        }
    }

    if probe.checks.iter().any(|check| {
        check.id.starts_with("config.schema:")
            && check.status == ProbeStatus::Warn
            && check.message.contains("model.")
    }) {
        match apply_hermes_config_from_profile() {
            Ok(()) => result
                .executed
                .push("fix-hermes-config-from-profile".to_string()),
            Err(error) => result.skipped.push(SkippedRepairAction {
                id: "fix-hermes-config-from-profile".to_string(),
                reason: error.to_string(),
            }),
        }
    }

    Ok(result)
}

fn hermes_config_path() -> PathBuf {
    home_join(".hermes/config.yaml")
}

fn hermes_env_path() -> PathBuf {
    home_join(".hermes/.env")
}

fn active_hermes_preset() -> Result<HermesProfilePreset> {
    let doc = load_profiles().context("failed to load profiles")?;
    let active = doc
        .active
        .with_context(|| "no active profile — run `agent-doctor profile use <name>`")?;
    let entry = doc
        .profiles
        .get(&active)
        .with_context(|| format!("active profile '{active}' not found"))?;
    entry
        .hermes
        .clone()
        .or_else(|| entry.models.first().cloned())
        .with_context(|| format!("profile '{active}' has no Hermes model preset"))
}

fn apply_hermes_config_from_profile() -> Result<()> {
    let preset = active_hermes_preset()?;
    let path = hermes_config_path();
    if !path.exists() {
        anyhow::bail!("Hermes config not found at {}", path.display());
    }

    let raw = fs::read_to_string(&path)?;
    let mut root: Value = serde_yaml::from_str(&raw)
        .with_context(|| format!("failed to parse {}", path.display()))?;
    let mapping = root
        .as_mapping_mut()
        .context("Hermes config root must be a mapping")?;
    let model = mapping
        .entry(Value::from("model"))
        .or_insert_with(|| Value::Mapping(Mapping::new()));
    let model_map = model
        .as_mapping_mut()
        .context("Hermes model section must be a mapping")?;

    let mut changed = false;
    changed |= set_model_field(model_map, "provider", &preset.provider);
    changed |= set_model_field(model_map, "default", &preset.model);
    changed |= set_model_field(model_map, "base_url", &preset.base_url);

    if let Some(url) = model_map
        .get(Value::from("base_url"))
        .and_then(Value::as_str)
    {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            model_map.insert(
                Value::from("base_url"),
                Value::from(preset.base_url.as_str()),
            );
            changed = true;
        }
    }

    if !changed {
        return Ok(());
    }

    let updated = serde_yaml::to_string(&root)?;
    fs::write(&path, updated).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

fn set_model_field(model: &mut Mapping, key: &str, value: &str) -> bool {
    let key_value = Value::from(key);
    let needs = match model.get(&key_value).and_then(Value::as_str) {
        None => true,
        Some(current) => current.trim().is_empty(),
    };
    if needs && !value.trim().is_empty() {
        model.insert(key_value, Value::from(value));
        return true;
    }
    false
}

fn tighten_env_permissions() -> Result<()> {
    let path = hermes_env_path();
    if !path.exists() {
        anyhow::bail!(".env does not exist");
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(&path)?;
        let mode = metadata.permissions().mode() & 0o777;
        if mode & 0o077 == 0 {
            return Ok(());
        }
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
            .with_context(|| format!("failed to chmod 600 {}", path.display()))?;
        return Ok(());
    }

    #[cfg(not(unix))]
    anyhow::bail!("permission tightening is only supported on Unix")
}

fn dedupe_api_key_env(probe: &RuntimeProbeReport) -> Result<()> {
    let env_key = probe
        .facts
        .iter()
        .find(|fact| fact.key == "hermes.api_key.env")
        .map(|fact| fact.value.clone())
        .context("missing hermes.api_key.env fact for duplicate cleanup")?;

    let path = hermes_env_path();
    let raw =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let updated = dedupe_env_key_lines(&raw, &env_key)?;
    if updated == raw {
        return Ok(());
    }
    fs::write(&path, updated).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

/// Keep the last non-empty assignment for `key`; drop earlier duplicates.
pub fn dedupe_env_key_lines(raw: &str, key: &str) -> Result<String> {
    let mut preserved: Vec<String> = Vec::new();
    let mut last_value: Option<String> = None;

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            preserved.push(line.to_string());
            continue;
        }
        let assignment = trimmed.strip_prefix("export ").unwrap_or(trimmed);
        let Some((name, value)) = assignment.split_once('=') else {
            preserved.push(line.to_string());
            continue;
        };
        if name.trim() == key {
            last_value = Some(
                value
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string(),
            );
            continue;
        }
        preserved.push(line.to_string());
    }

    let Some(value) = last_value else {
        anyhow::bail!("{key} was not found while deduplicating");
    };
    if value.is_empty() {
        anyhow::bail!("{key} has no non-empty value to keep");
    }

    preserved.push(format!("{key}={value}"));
    Ok(format!("{}\n", preserved.join("\n")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn dedupe_env_key_lines_keeps_last_non_empty_value() {
        let raw = "FOO=bar\nDEEPSEEK_API_KEY=old\nDEEPSEEK_API_KEY=new\n";
        let updated = dedupe_env_key_lines(raw, "DEEPSEEK_API_KEY").expect("dedupe env");
        assert!(updated.contains("DEEPSEEK_API_KEY=new"));
        assert!(!updated.contains("DEEPSEEK_API_KEY=old"));
        assert!(updated.contains("FOO=bar"));
    }

    #[test]
    fn set_model_field_fills_missing_provider() {
        let mut model = Mapping::new();
        assert!(set_model_field(&mut model, "provider", "deepseek"));
        assert_eq!(
            model.get(Value::from("provider")).and_then(Value::as_str),
            Some("deepseek")
        );
    }

    #[test]
    fn tighten_env_permissions_sets_600_on_unix() {
        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join(".env");
        let mut file = fs::File::create(&path).expect("create");
        writeln!(file, "DEEPSEEK_API_KEY=test").expect("write");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).expect("chmod");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).expect("chmod");
            let metadata = fs::metadata(&path).expect("metadata");
            assert_eq!(metadata.permissions().mode() & 0o777, 0o600);
        }
    }
}
