use std::path::Path;

use crate::runtime::ConfigFormat;

#[derive(Debug)]
pub(crate) enum ParsedConfig {
    Json(serde_json::Value),
    Yaml(serde_yaml::Value),
    Toml(toml::Value),
    Env(EnvFile),
}

#[derive(Debug, Clone)]
pub(crate) struct EnvEntry {
    pub key: String,
    pub value_present: bool,
    pub value_empty: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct EnvFile {
    pub entries: Vec<EnvEntry>,
    pub malformed_lines: Vec<usize>,
}

pub(crate) fn parse_config(raw: &str, format: ConfigFormat) -> Result<ParsedConfig, String> {
    match format {
        ConfigFormat::Json => serde_json::from_str(raw)
            .map(ParsedConfig::Json)
            .map_err(|error| format!("invalid JSON: {error}")),
        ConfigFormat::Yaml => serde_yaml::from_str(raw)
            .map(ParsedConfig::Yaml)
            .map_err(|error| format!("invalid YAML: {error}")),
        ConfigFormat::Toml => toml::from_str(raw)
            .map(ParsedConfig::Toml)
            .map_err(|error| format!("invalid TOML: {error}")),
        ConfigFormat::Env => Ok(ParsedConfig::Env(parse_env_file(raw))),
    }
}

pub(crate) fn parse_env_file(raw: &str) -> EnvFile {
    let mut entries = Vec::new();
    let mut malformed_lines = Vec::new();
    for (idx, line) in raw.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let assignment = trimmed.strip_prefix("export ").unwrap_or(trimmed);
        let Some((key, value)) = assignment.split_once('=') else {
            malformed_lines.push(idx + 1);
            continue;
        };
        let key = key.trim();
        if key.is_empty() {
            malformed_lines.push(idx + 1);
            continue;
        }
        let value = value.trim().trim_matches('"').trim_matches('\'');
        entries.push(EnvEntry {
            key: key.to_string(),
            value_present: true,
            value_empty: value.is_empty(),
        });
    }
    EnvFile {
        entries,
        malformed_lines,
    }
}

pub(crate) fn config_format_for_path(path: &Path, default_format: ConfigFormat) -> ConfigFormat {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
    {
        "json" => ConfigFormat::Json,
        "toml" => ConfigFormat::Toml,
        "yaml" | "yml" => ConfigFormat::Yaml,
        "env" => ConfigFormat::Env,
        _ if file_name == ".env" => ConfigFormat::Env,
        _ => default_format,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_env_file_and_tracks_malformed_lines() {
        let env = parse_env_file(
            r#"
DEEPSEEK_API_KEY=sk-test
EMPTY_KEY=
not-an-assignment
"#,
        );
        assert_eq!(env.entries.len(), 2);
        assert_eq!(env.entries[0].key, "DEEPSEEK_API_KEY");
        assert!(!env.entries[0].value_empty);
        assert_eq!(env.entries[1].key, "EMPTY_KEY");
        assert!(env.entries[1].value_empty);
        assert_eq!(env.malformed_lines, vec![4]);
    }

    #[test]
    fn hermes_required_key_detects_empty_and_duplicate_entries() {
        let env = parse_env_file("DEEPSEEK_API_KEY=\nDEEPSEEK_API_KEY=sk-test\n");
        let matches: Vec<_> = env
            .entries
            .iter()
            .filter(|entry| entry.key == "DEEPSEEK_API_KEY")
            .collect();
        assert_eq!(matches.len(), 2);
        assert!(matches.iter().any(|entry| !entry.value_empty));
    }
}
