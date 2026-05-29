use std::path::Path;

use crate::repair::{DiagnosticFact, SensitivityLevel};

use super::super::config::ParsedConfig;
use super::super::schema::schema_warn;
use super::super::ProbeCheck;

pub(crate) fn probe_schema(
    path: &Path,
    parsed: &ParsedConfig,
    checks: &mut Vec<ProbeCheck>,
    facts: &mut Vec<DiagnosticFact>,
) {
    let ParsedConfig::Toml(value) = parsed else {
        return;
    };

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
