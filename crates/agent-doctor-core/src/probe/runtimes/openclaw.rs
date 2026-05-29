use std::path::Path;

use crate::repair::{DiagnosticFact, SensitivityLevel};

use super::super::config::ParsedConfig;
use super::super::schema::{schema_error, schema_warn};
use super::super::{ProbeCheck};

pub(crate) fn probe_schema(
    path: &Path,
    parsed: &ParsedConfig,
    checks: &mut Vec<ProbeCheck>,
    facts: &mut Vec<DiagnosticFact>,
) {
    let ParsedConfig::Json(value) = parsed else {
        return;
    };

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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn detects_openclaw_schema_warnings() {
        let value = serde_json::json!({
            "tools": { "profile": "bad" },
            "agents": { "defaults": { "timeout": 10 } },
            "env": { "vars": "{\"OPENAI_API_KEY\":\"x\"}" }
        });
        let mut checks = Vec::new();
        let mut facts = Vec::new();
        probe_schema(
            Path::new("/tmp/openclaw.json"),
            &ParsedConfig::Json(value),
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
}
