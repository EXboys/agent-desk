use std::fs;
use std::net::{TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::adapter::RuntimeAdapter;
use crate::adapters::util::{find_all_binaries, read_version_result};
use crate::adapters::{adapter_by_id, all_adapters, HermesAdapter};
use crate::repair::{DiagnosticBundle, DiagnosticFact, SensitivityLevel};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeStatus {
    Pass,
    Warn,
    Fail,
    NotApplicable,
    NotChecked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeCheck {
    pub id: String,
    pub title: String,
    pub status: ProbeStatus,
    pub severity: ProbeSeverity,
    pub message: String,
    pub details: Vec<String>,
    pub sensitivity: SensitivityLevel,
}

impl ProbeCheck {
    fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        status: ProbeStatus,
        severity: ProbeSeverity,
        message: impl Into<String>,
        sensitivity: SensitivityLevel,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            status,
            severity,
            message: message.into(),
            details: Vec::new(),
            sensitivity,
        }
    }

    fn with_details(mut self, details: Vec<String>) -> Self {
        self.details = details;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeProbeReport {
    pub runtime_id: String,
    pub display_name: String,
    pub binary_name: String,
    pub checks: Vec<ProbeCheck>,
    pub facts: Vec<DiagnosticFact>,
}

impl RuntimeProbeReport {
    pub fn to_diagnostic_bundle(&self) -> DiagnosticBundle {
        DiagnosticBundle {
            runtime_id: self.runtime_id.clone(),
            facts: self.facts.clone(),
            notes: self
                .checks
                .iter()
                .filter(|check| matches!(check.status, ProbeStatus::Warn | ProbeStatus::Fail))
                .map(|check| format!("{}: {}", check.title, check.message))
                .collect(),
        }
    }
}

#[derive(Clone, Copy)]
enum ConfigFormat {
    Json,
    Yaml,
    Toml,
    Env,
}

#[derive(Clone, Copy)]
struct RuntimeProbeSpec {
    runtime_id: &'static str,
    binary_name: &'static str,
    config_format: ConfigFormat,
    env_keywords: &'static [&'static str],
}

const OPENCLAW_ENV: &[&str] = &["OPENCLAW", "EVOTOWN", "OPENAI", "ANTHROPIC"];
const CLAUDE_ENV: &[&str] = &["ANTHROPIC", "CLAUDE"];
const CODEX_ENV: &[&str] = &["OPENAI", "CODEX"];
const HERMES_ENV: &[&str] = &["HERMES", "OPENAI", "ANTHROPIC", "DEEPSEEK", "GOOGLE"];

pub fn probe_all_runtimes() -> Vec<RuntimeProbeReport> {
    all_adapters()
        .iter()
        .filter_map(|adapter| probe_adapter(adapter.as_ref()).ok())
        .collect()
}

pub fn probe_runtime(runtime_id: &str) -> Result<RuntimeProbeReport> {
    let adapter =
        adapter_by_id(runtime_id).with_context(|| format!("unknown runtime '{runtime_id}'"))?;
    probe_adapter(adapter.as_ref())
}

fn probe_adapter(adapter: &dyn RuntimeAdapter) -> Result<RuntimeProbeReport> {
    let spec = spec_for_runtime(adapter.id())
        .with_context(|| format!("no probe spec for runtime '{}'", adapter.id()))?;
    let mut checks = Vec::new();
    let mut facts = Vec::new();

    probe_binary(&spec, &mut checks, &mut facts);
    probe_configs(adapter, spec, &mut checks, &mut facts);
    probe_env_conflicts(spec, &mut checks, &mut facts);
    if spec.runtime_id == "hermes" {
        probe_hermes_deep(&mut checks, &mut facts);
    }
    probe_gateway(adapter, &mut checks, &mut facts);

    Ok(RuntimeProbeReport {
        runtime_id: adapter.id().to_string(),
        display_name: adapter.display_name().to_string(),
        binary_name: spec.binary_name.to_string(),
        checks,
        facts,
    })
}

fn spec_for_runtime(runtime_id: &str) -> Option<RuntimeProbeSpec> {
    match runtime_id {
        "openclaw" => Some(RuntimeProbeSpec {
            runtime_id: "openclaw",
            binary_name: "openclaw",
            config_format: ConfigFormat::Json,
            env_keywords: OPENCLAW_ENV,
        }),
        "claude-code" => Some(RuntimeProbeSpec {
            runtime_id: "claude-code",
            binary_name: "claude",
            config_format: ConfigFormat::Json,
            env_keywords: CLAUDE_ENV,
        }),
        "codex" => Some(RuntimeProbeSpec {
            runtime_id: "codex",
            binary_name: "codex",
            config_format: ConfigFormat::Toml,
            env_keywords: CODEX_ENV,
        }),
        "hermes" => Some(RuntimeProbeSpec {
            runtime_id: "hermes",
            binary_name: "hermes",
            config_format: ConfigFormat::Yaml,
            env_keywords: HERMES_ENV,
        }),
        _ => None,
    }
}

fn probe_binary(
    spec: &RuntimeProbeSpec,
    checks: &mut Vec<ProbeCheck>,
    facts: &mut Vec<DiagnosticFact>,
) {
    let binaries = find_all_binaries(spec.binary_name);
    if binaries.is_empty() {
        checks.push(ProbeCheck::new(
            "binary.exists",
            "Binary exists",
            ProbeStatus::Fail,
            ProbeSeverity::Error,
            format!(
                "{} was not found in PATH or common bin directories",
                spec.binary_name
            ),
            SensitivityLevel::Public,
        ));
        facts.push(DiagnosticFact::new(
            "binary.installed",
            "false",
            SensitivityLevel::Public,
        ));
        return;
    }

    let default_binary = binaries[0].display().to_string();
    checks.push(
        ProbeCheck::new(
            "binary.exists",
            "Binary exists",
            ProbeStatus::Pass,
            ProbeSeverity::Info,
            format!("{} was found", spec.binary_name),
            SensitivityLevel::LocalPath,
        )
        .with_details(vec![default_binary.clone()]),
    );
    facts.push(DiagnosticFact::new(
        "binary.path",
        default_binary,
        SensitivityLevel::LocalPath,
    ));

    let conflict_status = if binaries.len() > 1 {
        ProbeStatus::Warn
    } else {
        ProbeStatus::Pass
    };
    checks.push(
        ProbeCheck::new(
            "binary.path_conflict",
            "Multiple installs",
            conflict_status,
            if binaries.len() > 1 {
                ProbeSeverity::Warning
            } else {
                ProbeSeverity::Info
            },
            if binaries.len() > 1 {
                format!("found {} candidate binaries", binaries.len())
            } else {
                "no duplicate install candidates found".to_string()
            },
            SensitivityLevel::LocalPath,
        )
        .with_details(
            binaries
                .iter()
                .map(|path| path.display().to_string())
                .collect(),
        ),
    );

    match read_version_result(&binaries[0]) {
        Ok(Some(version)) => {
            checks.push(ProbeCheck::new(
                "binary.version",
                "Version command",
                ProbeStatus::Pass,
                ProbeSeverity::Info,
                version.clone(),
                SensitivityLevel::Public,
            ));
            facts.push(DiagnosticFact::new(
                "binary.version",
                version,
                SensitivityLevel::Public,
            ));
        }
        Ok(None) => checks.push(ProbeCheck::new(
            "binary.version",
            "Version command",
            ProbeStatus::Warn,
            ProbeSeverity::Warning,
            "version command ran but returned no output",
            SensitivityLevel::Public,
        )),
        Err(error) => checks.push(ProbeCheck::new(
            "binary.version",
            "Version command",
            ProbeStatus::Fail,
            ProbeSeverity::Error,
            error,
            SensitivityLevel::SensitiveLog,
        )),
    }
}

fn probe_configs(
    adapter: &dyn RuntimeAdapter,
    spec: RuntimeProbeSpec,
    checks: &mut Vec<ProbeCheck>,
    facts: &mut Vec<DiagnosticFact>,
) {
    let config_paths = adapter.config_paths();
    if config_paths.is_empty() {
        checks.push(ProbeCheck::new(
            "config.paths",
            "Config paths",
            ProbeStatus::NotApplicable,
            ProbeSeverity::Info,
            "adapter has no config paths",
            SensitivityLevel::Public,
        ));
        return;
    }

    for path in config_paths {
        let path_text = path.display().to_string();
        facts.push(DiagnosticFact::new(
            "config.path",
            path_text.clone(),
            SensitivityLevel::LocalPath,
        ));

        if !path.exists() {
            checks.push(ProbeCheck::new(
                format!("config.exists:{}", path.display()),
                "Config exists",
                ProbeStatus::Warn,
                ProbeSeverity::Warning,
                format!("config file not found at {}", path.display()),
                SensitivityLevel::LocalPath,
            ));
            continue;
        }

        checks.push(ProbeCheck::new(
            format!("config.exists:{}", path.display()),
            "Config exists",
            ProbeStatus::Pass,
            ProbeSeverity::Info,
            format!("config file exists at {}", path.display()),
            SensitivityLevel::LocalPath,
        ));

        let format = config_format_for_path(&path, spec.config_format);
        match fs::read_to_string(&path) {
            Ok(raw) => match parse_config(&raw, format) {
                Ok(parsed) => {
                    checks.push(ProbeCheck::new(
                        format!("config.parse:{}", path.display()),
                        "Config parse",
                        ProbeStatus::Pass,
                        ProbeSeverity::Info,
                        "config parsed successfully",
                        SensitivityLevel::ConfigShape,
                    ));
                    probe_schema(spec.runtime_id, &path, &parsed, checks, facts);
                    probe_path_references(&path, &parsed, checks, facts);
                }
                Err(error) => checks.push(ProbeCheck::new(
                    format!("config.parse:{}", path.display()),
                    "Config parse",
                    ProbeStatus::Fail,
                    ProbeSeverity::Error,
                    error,
                    SensitivityLevel::SensitiveLog,
                )),
            },
            Err(error) => checks.push(ProbeCheck::new(
                format!("config.read:{}", path.display()),
                "Config read",
                ProbeStatus::Fail,
                ProbeSeverity::Error,
                format!("failed to read {}: {}", path.display(), error),
                SensitivityLevel::LocalPath,
            )),
        }
    }
}

enum ParsedConfig {
    Json(serde_json::Value),
    Yaml(serde_yaml::Value),
    Toml(toml::Value),
    Env(EnvFile),
}

#[derive(Debug, Clone)]
struct EnvEntry {
    key: String,
    value_present: bool,
    value_empty: bool,
}

#[derive(Debug, Clone)]
struct EnvFile {
    entries: Vec<EnvEntry>,
    malformed_lines: Vec<usize>,
}

fn parse_config(raw: &str, format: ConfigFormat) -> Result<ParsedConfig, String> {
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

fn parse_env_file(raw: &str) -> EnvFile {
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

fn config_format_for_path(path: &Path, default_format: ConfigFormat) -> ConfigFormat {
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

fn probe_schema(
    runtime_id: &str,
    path: &Path,
    parsed: &ParsedConfig,
    checks: &mut Vec<ProbeCheck>,
    facts: &mut Vec<DiagnosticFact>,
) {
    match (runtime_id, parsed) {
        ("openclaw", ParsedConfig::Json(value)) => {
            probe_openclaw_schema(path, value, checks, facts)
        }
        ("claude-code", ParsedConfig::Json(value)) => probe_claude_schema(path, value, checks),
        ("codex", ParsedConfig::Toml(value)) => probe_codex_schema(path, value, checks, facts),
        ("hermes", ParsedConfig::Yaml(value)) => probe_hermes_schema(path, value, checks, facts),
        ("hermes", ParsedConfig::Env(value)) => probe_hermes_env_schema(path, value, checks),
        _ => {}
    }
}

fn probe_openclaw_schema(
    path: &Path,
    value: &serde_json::Value,
    checks: &mut Vec<ProbeCheck>,
    facts: &mut Vec<DiagnosticFact>,
) {
    if !value.is_object() {
        checks.push(schema_error(
            path,
            "OpenClaw config root must be a JSON object",
        ));
        return;
    }

    let gateway = value
        .pointer("/gateway/url")
        .or_else(|| value.pointer("/evotown/url"))
        .and_then(serde_json::Value::as_str);
    if let Some(url) = gateway {
        facts.push(DiagnosticFact::new(
            "gateway.url",
            url,
            SensitivityLevel::ConfigShape,
        ));
    }

    if let Some(profile) = value
        .pointer("/tools/profile")
        .and_then(serde_json::Value::as_str)
    {
        let allowed = ["minimal", "coding", "messaging", "full"];
        if !allowed.contains(&profile) {
            checks.push(schema_warn(
                path,
                format!("tools.profile has unsupported value '{profile}'"),
            ));
        }
    }

    if value.pointer("/agents/defaults/timeout").is_some() {
        checks.push(schema_warn(
            path,
            "agents.defaults.timeout is legacy; expected timeoutSeconds".to_string(),
        ));
    }

    for pointer in ["/env/vars", "/env/shellEnv"] {
        if value
            .pointer(pointer)
            .is_some_and(serde_json::Value::is_string)
        {
            checks.push(schema_warn(
                path,
                format!("{pointer} is a string; expected object"),
            ));
        }
    }
}

fn probe_claude_schema(path: &Path, value: &serde_json::Value, checks: &mut Vec<ProbeCheck>) {
    if !value.is_object() {
        checks.push(schema_error(
            path,
            "Claude settings root must be a JSON object",
        ));
        return;
    }
    if value.get("env").is_some_and(|env| !env.is_object()) {
        checks.push(schema_warn(path, "env should be an object".to_string()));
    }
}

fn probe_codex_schema(
    path: &Path,
    value: &toml::Value,
    checks: &mut Vec<ProbeCheck>,
    facts: &mut Vec<DiagnosticFact>,
) {
    let provider = value.get("model_provider").and_then(toml::Value::as_str);
    if let Some(provider) = provider {
        facts.push(DiagnosticFact::new(
            "model.provider",
            provider,
            SensitivityLevel::ConfigShape,
        ));
        if value
            .get("model_providers")
            .and_then(|providers| providers.get(provider))
            .is_none()
        {
            checks.push(schema_warn(
                path,
                format!("model_provider '{provider}' has no matching model_providers entry"),
            ));
        }
    }
    if let Some(model) = value.get("model").and_then(toml::Value::as_str) {
        facts.push(DiagnosticFact::new(
            "model.name",
            model,
            SensitivityLevel::ConfigShape,
        ));
    }
}

fn probe_hermes_schema(
    path: &Path,
    value: &serde_yaml::Value,
    checks: &mut Vec<ProbeCheck>,
    facts: &mut Vec<DiagnosticFact>,
) {
    let Some(model) = value.get("model") else {
        checks.push(schema_warn(path, "model section is missing".to_string()));
        return;
    };
    if !model.is_mapping() {
        checks.push(schema_error(path, "model section must be a mapping"));
        return;
    }

    for key in ["provider", "default", "base_url"] {
        match model.get(key).and_then(serde_yaml::Value::as_str) {
            Some(value) if !value.trim().is_empty() => {
                facts.push(DiagnosticFact::new(
                    format!(
                        "{}.{}",
                        if key == "default" { "model" } else { "hermes" },
                        if key == "default" { "name" } else { key }
                    ),
                    value,
                    SensitivityLevel::ConfigShape,
                ));
            }
            Some(_) => checks.push(schema_warn(path, format!("model.{key} is empty"))),
            None => checks.push(schema_warn(path, format!("model.{key} is missing"))),
        }
    }
    if let Some(base_url) = model.get("base_url").and_then(serde_yaml::Value::as_str) {
        if !base_url.starts_with("http://") && !base_url.starts_with("https://") {
            checks.push(schema_warn(
                path,
                "model.base_url should start with http:// or https://".to_string(),
            ));
        }
        facts.push(DiagnosticFact::new(
            "gateway.url",
            base_url,
            SensitivityLevel::ConfigShape,
        ));
    }
}

fn probe_hermes_env_schema(path: &Path, env: &EnvFile, checks: &mut Vec<ProbeCheck>) {
    if env.malformed_lines.is_empty() {
        checks.push(ProbeCheck::new(
            format!("hermes.env.parse:{}", path.display()),
            "Hermes .env parse",
            ProbeStatus::Pass,
            ProbeSeverity::Info,
            "Hermes .env contains valid KEY=value entries",
            SensitivityLevel::ConfigShape,
        ));
    } else {
        checks.push(
            ProbeCheck::new(
                format!("hermes.env.parse:{}", path.display()),
                "Hermes .env parse",
                ProbeStatus::Warn,
                ProbeSeverity::Warning,
                format!(
                    "{} .env lines are not KEY=value assignments",
                    env.malformed_lines.len()
                ),
                SensitivityLevel::ConfigShape,
            )
            .with_details(
                env.malformed_lines
                    .iter()
                    .map(|line| format!("line {line}"))
                    .collect(),
            ),
        );
    }
    probe_hermes_env_permissions(path, checks);
}

#[cfg(unix)]
fn probe_hermes_env_permissions(path: &Path, checks: &mut Vec<ProbeCheck>) {
    use std::os::unix::fs::PermissionsExt;

    if let Ok(metadata) = fs::metadata(path) {
        let mode = metadata.permissions().mode() & 0o777;
        let too_open = mode & 0o077 != 0;
        checks.push(ProbeCheck::new(
            format!("hermes.env.permissions:{}", path.display()),
            "Hermes .env permissions",
            if too_open {
                ProbeStatus::Warn
            } else {
                ProbeStatus::Pass
            },
            if too_open {
                ProbeSeverity::Warning
            } else {
                ProbeSeverity::Info
            },
            if too_open {
                format!(".env permissions are {mode:o}; recommended 600")
            } else {
                format!(".env permissions are {mode:o}")
            },
            SensitivityLevel::LocalPath,
        ));
    }
}

#[cfg(not(unix))]
fn probe_hermes_env_permissions(_path: &Path, _checks: &mut Vec<ProbeCheck>) {}

fn probe_hermes_deep(checks: &mut Vec<ProbeCheck>, facts: &mut Vec<DiagnosticFact>) {
    let provider = facts
        .iter()
        .find(|fact| fact.key == "hermes.provider")
        .map(|fact| fact.value.trim().to_string())
        .filter(|value| !value.is_empty());

    let Some(provider) = provider else {
        checks.push(ProbeCheck::new(
            "hermes.provider",
            "Hermes provider",
            ProbeStatus::Warn,
            ProbeSeverity::Warning,
            "Hermes model.provider is missing; API key requirement cannot be determined",
            SensitivityLevel::ConfigShape,
        ));
        return;
    };

    let api_key_env = HermesAdapter::provider_api_key_env(&provider);
    match api_key_env {
        None => {
            checks.push(ProbeCheck::new(
                "hermes.api_key.required",
                "Hermes API key requirement",
                ProbeStatus::NotApplicable,
                ProbeSeverity::Info,
                format!("provider '{provider}' does not require an API key"),
                SensitivityLevel::ConfigShape,
            ));
            facts.push(DiagnosticFact::new(
                "hermes.api_key.required",
                "false",
                SensitivityLevel::Public,
            ));
        }
        Some(env_key) => {
            facts.push(DiagnosticFact::new(
                "hermes.api_key.env",
                env_key.clone(),
                SensitivityLevel::ConfigShape,
            ));
            facts.push(DiagnosticFact::new(
                "hermes.api_key.required",
                "true",
                SensitivityLevel::Public,
            ));
            probe_hermes_required_key(&env_key, checks, facts);
        }
    }
}

fn probe_hermes_required_key(
    env_key: &str,
    checks: &mut Vec<ProbeCheck>,
    facts: &mut Vec<DiagnosticFact>,
) {
    let env_path = dirs::home_dir()
        .map(|home| home.join(".hermes/.env"))
        .unwrap_or_else(|| PathBuf::from(".hermes/.env"));

    if !env_path.exists() {
        checks.push(ProbeCheck::new(
            "hermes.api_key.configured",
            "Hermes API key configured",
            ProbeStatus::Warn,
            ProbeSeverity::Warning,
            format!("{env_key} is required but ~/.hermes/.env does not exist"),
            SensitivityLevel::LocalPath,
        ));
        facts.push(DiagnosticFact::new(
            "hermes.api_key.configured",
            "false",
            SensitivityLevel::Public,
        ));
        return;
    }

    let raw = match fs::read_to_string(&env_path) {
        Ok(raw) => raw,
        Err(error) => {
            checks.push(ProbeCheck::new(
                "hermes.api_key.configured",
                "Hermes API key configured",
                ProbeStatus::Warn,
                ProbeSeverity::Warning,
                format!("failed to read ~/.hermes/.env: {error}"),
                SensitivityLevel::LocalPath,
            ));
            return;
        }
    };

    let env = parse_env_file(&raw);
    let matches: Vec<_> = env
        .entries
        .iter()
        .filter(|entry| entry.key == env_key)
        .collect();
    let configured = matches
        .iter()
        .any(|entry| entry.value_present && !entry.value_empty);
    facts.push(DiagnosticFact::new(
        "hermes.api_key.configured",
        configured.to_string(),
        SensitivityLevel::Public,
    ));

    if matches.is_empty() {
        checks.push(ProbeCheck::new(
            "hermes.api_key.configured",
            "Hermes API key configured",
            ProbeStatus::Warn,
            ProbeSeverity::Warning,
            format!("{env_key} is missing from ~/.hermes/.env"),
            SensitivityLevel::ConfigShape,
        ));
        return;
    }

    if matches.len() > 1 {
        checks.push(ProbeCheck::new(
            "hermes.api_key.duplicates",
            "Hermes API key duplicates",
            ProbeStatus::Warn,
            ProbeSeverity::Warning,
            format!(
                "{env_key} appears {} times in ~/.hermes/.env",
                matches.len()
            ),
            SensitivityLevel::ConfigShape,
        ));
    }

    checks.push(ProbeCheck::new(
        "hermes.api_key.configured",
        "Hermes API key configured",
        if configured {
            ProbeStatus::Pass
        } else {
            ProbeStatus::Warn
        },
        if configured {
            ProbeSeverity::Info
        } else {
            ProbeSeverity::Warning
        },
        if configured {
            format!("{env_key} is configured in ~/.hermes/.env")
        } else {
            format!("{env_key} exists in ~/.hermes/.env but is empty")
        },
        SensitivityLevel::ConfigShape,
    ));
}

fn schema_warn(path: &Path, message: String) -> ProbeCheck {
    ProbeCheck::new(
        format!("config.schema:{}", path.display()),
        "Config schema",
        ProbeStatus::Warn,
        ProbeSeverity::Warning,
        message,
        SensitivityLevel::ConfigShape,
    )
}

fn schema_error(path: &Path, message: impl Into<String>) -> ProbeCheck {
    ProbeCheck::new(
        format!("config.schema:{}", path.display()),
        "Config schema",
        ProbeStatus::Fail,
        ProbeSeverity::Error,
        message,
        SensitivityLevel::ConfigShape,
    )
}

fn probe_env_conflicts(
    spec: RuntimeProbeSpec,
    checks: &mut Vec<ProbeCheck>,
    facts: &mut Vec<DiagnosticFact>,
) {
    let conflicts = collect_env_conflicts(spec.env_keywords);
    if conflicts.is_empty() {
        checks.push(ProbeCheck::new(
            "env.conflicts",
            "Environment conflicts",
            ProbeStatus::Pass,
            ProbeSeverity::Info,
            "no matching environment variables found in process or common shell files",
            SensitivityLevel::ConfigShape,
        ));
        return;
    }

    for conflict in &conflicts {
        facts.push(DiagnosticFact::new(
            "env.conflict",
            conflict.clone(),
            SensitivityLevel::SensitiveLog,
        ));
    }
    checks.push(
        ProbeCheck::new(
            "env.conflicts",
            "Environment conflicts",
            ProbeStatus::Warn,
            ProbeSeverity::Warning,
            format!(
                "found {} environment entries that may override runtime config",
                conflicts.len()
            ),
            SensitivityLevel::SensitiveLog,
        )
        .with_details(conflicts),
    );
}

fn collect_env_conflicts(keywords: &[&str]) -> Vec<String> {
    let mut conflicts = Vec::new();
    for (key, value) in std::env::vars() {
        if keywords
            .iter()
            .any(|keyword| key.to_uppercase().contains(keyword))
        {
            let visible = if looks_sensitive_env_key(&key) {
                "[REDACTED]".to_string()
            } else {
                value
            };
            conflicts.push(format!("process:{key}={visible}"));
        }
    }

    for path in shell_config_paths() {
        if let Ok(raw) = fs::read_to_string(&path) {
            for (idx, line) in raw.lines().enumerate() {
                let trimmed = line.trim();
                if trimmed.starts_with('#') || !trimmed.contains('=') {
                    continue;
                }
                let assignment = trimmed.strip_prefix("export ").unwrap_or(trimmed);
                let Some((name, _)) = assignment.split_once('=') else {
                    continue;
                };
                let name = name.trim();
                if keywords
                    .iter()
                    .any(|keyword| name.to_uppercase().contains(keyword))
                {
                    conflicts.push(format!("{}:{}:{}", path.display(), idx + 1, name));
                }
            }
        }
    }
    conflicts
}

fn looks_sensitive_env_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    ["key", "token", "secret", "password", "auth"]
        .iter()
        .any(|needle| key.contains(needle))
}

fn shell_config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(home) = dirs::home_dir() {
        paths.extend([
            home.join(".bashrc"),
            home.join(".bash_profile"),
            home.join(".zshrc"),
            home.join(".zprofile"),
            home.join(".profile"),
        ]);
    }
    paths
}

fn probe_gateway(
    adapter: &dyn RuntimeAdapter,
    checks: &mut Vec<ProbeCheck>,
    facts: &mut Vec<DiagnosticFact>,
) {
    let profile = match adapter.read_profile() {
        Ok(profile) => profile,
        Err(error) => {
            checks.push(ProbeCheck::new(
                "gateway.profile_read",
                "Gateway profile",
                ProbeStatus::Warn,
                ProbeSeverity::Warning,
                format!("failed to read gateway profile: {error}"),
                SensitivityLevel::SensitiveLog,
            ));
            return;
        }
    };

    let Some(url) = profile.gateway_url.filter(|url| !url.trim().is_empty()) else {
        checks.push(ProbeCheck::new(
            "gateway.configured",
            "Gateway configured",
            ProbeStatus::NotApplicable,
            ProbeSeverity::Info,
            "no gateway/base_url configured",
            SensitivityLevel::ConfigShape,
        ));
        return;
    };

    facts.push(DiagnosticFact::new(
        "gateway.url",
        url.clone(),
        SensitivityLevel::ConfigShape,
    ));

    match gateway_socket_addr(&url) {
        Some(addr) => match addr.to_socket_addrs() {
            Ok(addrs) => {
                let timeout = Duration::from_millis(750);
                let reachable = addrs
                    .into_iter()
                    .any(|addr| TcpStream::connect_timeout(&addr, timeout).is_ok());
                checks.push(ProbeCheck::new(
                    "gateway.connectivity",
                    "Gateway connectivity",
                    if reachable {
                        ProbeStatus::Pass
                    } else {
                        ProbeStatus::Warn
                    },
                    if reachable {
                        ProbeSeverity::Info
                    } else {
                        ProbeSeverity::Warning
                    },
                    if reachable {
                        "gateway host accepted a TCP connection".to_string()
                    } else {
                        "gateway host did not accept a TCP connection within timeout".to_string()
                    },
                    SensitivityLevel::ConfigShape,
                ));
            }
            Err(error) => checks.push(ProbeCheck::new(
                "gateway.connectivity",
                "Gateway connectivity",
                ProbeStatus::Warn,
                ProbeSeverity::Warning,
                format!("failed to resolve gateway host: {error}"),
                SensitivityLevel::ConfigShape,
            )),
        },
        None => checks.push(ProbeCheck::new(
            "gateway.connectivity",
            "Gateway connectivity",
            ProbeStatus::Warn,
            ProbeSeverity::Warning,
            "gateway URL could not be parsed for connectivity check",
            SensitivityLevel::ConfigShape,
        )),
    }
}

fn gateway_socket_addr(url: &str) -> Option<String> {
    let rest = url
        .strip_prefix("https://")
        .map(|value| (value, 443))
        .or_else(|| url.strip_prefix("http://").map(|value| (value, 80)))?;
    let (host_port_path, default_port) = rest;
    let host_port = host_port_path.split('/').next()?.split('?').next()?;
    if host_port.is_empty() {
        return None;
    }
    if host_port.contains(':') {
        Some(host_port.to_string())
    } else {
        Some(format!("{host_port}:{default_port}"))
    }
}

fn probe_path_references(
    config_path: &Path,
    parsed: &ParsedConfig,
    checks: &mut Vec<ProbeCheck>,
    facts: &mut Vec<DiagnosticFact>,
) {
    let mut refs = Vec::new();
    collect_path_references(parsed, &mut refs);
    if refs.is_empty() {
        checks.push(ProbeCheck::new(
            format!("paths.references:{}", config_path.display()),
            "MCP/Skills path references",
            ProbeStatus::NotChecked,
            ProbeSeverity::Info,
            "no obvious MCP/Skills path references found",
            SensitivityLevel::ConfigShape,
        ));
        return;
    }

    let mut missing = Vec::new();
    for reference in refs {
        facts.push(DiagnosticFact::new(
            "path.reference",
            reference.clone(),
            SensitivityLevel::LocalPath,
        ));
        if !Path::new(&reference).exists() {
            missing.push(reference);
        }
    }

    checks.push(
        ProbeCheck::new(
            format!("paths.references:{}", config_path.display()),
            "MCP/Skills path references",
            if missing.is_empty() {
                ProbeStatus::Pass
            } else {
                ProbeStatus::Warn
            },
            if missing.is_empty() {
                ProbeSeverity::Info
            } else {
                ProbeSeverity::Warning
            },
            if missing.is_empty() {
                "all obvious MCP/Skills path references exist".to_string()
            } else {
                format!("{} obvious path references are missing", missing.len())
            },
            SensitivityLevel::LocalPath,
        )
        .with_details(missing),
    );
}

fn collect_path_references(parsed: &ParsedConfig, out: &mut Vec<String>) {
    match parsed {
        ParsedConfig::Json(value) => collect_json_paths("", value, out),
        ParsedConfig::Yaml(value) => collect_yaml_paths("", value, out),
        ParsedConfig::Toml(value) => collect_toml_paths("", value, out),
        ParsedConfig::Env(_) => {}
    }
}

fn collect_json_paths(key_path: &str, value: &serde_json::Value, out: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, value) in map {
                let next = join_key(key_path, key);
                collect_json_paths(&next, value, out);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                collect_json_paths(key_path, item, out);
            }
        }
        serde_json::Value::String(text)
            if is_interesting_path_key(key_path) && is_path_like(text) =>
        {
            out.push(expand_home(text));
        }
        _ => {}
    }
}

fn collect_yaml_paths(key_path: &str, value: &serde_yaml::Value, out: &mut Vec<String>) {
    match value {
        serde_yaml::Value::Mapping(map) => {
            for (key, value) in map {
                let key = key.as_str().unwrap_or_default();
                let next = join_key(key_path, key);
                collect_yaml_paths(&next, value, out);
            }
        }
        serde_yaml::Value::Sequence(items) => {
            for item in items {
                collect_yaml_paths(key_path, item, out);
            }
        }
        serde_yaml::Value::String(text)
            if is_interesting_path_key(key_path) && is_path_like(text) =>
        {
            out.push(expand_home(text));
        }
        _ => {}
    }
}

fn collect_toml_paths(key_path: &str, value: &toml::Value, out: &mut Vec<String>) {
    match value {
        toml::Value::Table(map) => {
            for (key, value) in map {
                let next = join_key(key_path, key);
                collect_toml_paths(&next, value, out);
            }
        }
        toml::Value::Array(items) => {
            for item in items {
                collect_toml_paths(key_path, item, out);
            }
        }
        toml::Value::String(text) if is_interesting_path_key(key_path) && is_path_like(text) => {
            out.push(expand_home(text));
        }
        _ => {}
    }
}

fn join_key(base: &str, key: &str) -> String {
    if base.is_empty() {
        key.to_string()
    } else {
        format!("{base}.{key}")
    }
}

fn is_interesting_path_key(key_path: &str) -> bool {
    let key_path = key_path.to_ascii_lowercase();
    ["mcp", "skill", "skills", "manifest", "path", "command"]
        .iter()
        .any(|needle| key_path.contains(needle))
}

fn is_path_like(value: &str) -> bool {
    value.starts_with('/')
        || value.starts_with("~/")
        || value.starts_with("./")
        || value.starts_with("../")
}

fn expand_home(value: &str) -> String {
    if let Some(rest) = value.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest).display().to_string();
        }
    }
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_gateway_socket_addr() {
        assert_eq!(
            gateway_socket_addr("https://gateway.example/v1").as_deref(),
            Some("gateway.example:443")
        );
        assert_eq!(
            gateway_socket_addr("http://127.0.0.1:11434/v1").as_deref(),
            Some("127.0.0.1:11434")
        );
    }

    #[test]
    fn detects_openclaw_schema_warnings() {
        let value = serde_json::json!({
            "tools": { "profile": "bad" },
            "agents": { "defaults": { "timeout": 10 } },
            "env": { "vars": "{\"OPENAI_API_KEY\":\"x\"}" }
        });
        let mut checks = Vec::new();
        let mut facts = Vec::new();
        probe_openclaw_schema(
            Path::new("/tmp/openclaw.json"),
            &value,
            &mut checks,
            &mut facts,
        );
        assert!(checks
            .iter()
            .any(|check| check.message.contains("unsupported")));
        assert!(checks.iter().any(|check| check.message.contains("legacy")));
        assert!(checks
            .iter()
            .any(|check| check.message.contains("expected object")));
    }

    #[test]
    fn collects_path_references_from_interesting_keys() {
        let value = serde_json::json!({
            "mcp": { "servers": [{ "path": "~/missing-mcp" }] },
            "ordinary": "/not/collected"
        });
        let mut refs = Vec::new();
        collect_json_paths("", &value, &mut refs);
        assert_eq!(refs.len(), 1);
        assert!(refs[0].contains("missing-mcp"));
    }

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
