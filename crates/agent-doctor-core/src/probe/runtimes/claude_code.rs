use std::path::Path;

use crate::repair::DiagnosticFact;

use super::super::config::ParsedConfig;
use super::super::schema::{schema_error, schema_warn};
use super::super::ProbeCheck;

pub(crate) fn probe_schema(
    path: &Path,
    parsed: &ParsedConfig,
    checks: &mut Vec<ProbeCheck>,
    _facts: &mut Vec<DiagnosticFact>,
) {
    let ParsedConfig::Json(value) = parsed else {
        return;
    };

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
