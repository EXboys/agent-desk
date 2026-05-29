//! Hermes Agent install/update via official installers (aligned with CC Switch).
//!
//! Uses NousResearch install scripts instead of `pip install` to avoid Python 3.11+
//! and pyenv shim issues on macOS.

use anyhow::{Context, Result};

use super::runner::run_shell_command;

pub const HERMES_INSTALL_SCRIPT_URL: &str =
    "https://raw.githubusercontent.com/NousResearch/hermes-agent/main/scripts/install.sh";

#[cfg(target_os = "windows")]
pub const HERMES_INSTALL_PS1_URL: &str =
    "https://raw.githubusercontent.com/NousResearch/hermes-agent/main/scripts/install.ps1";

/// Unix install: curl to temp file, then bash (not `curl | bash` — safer under WSL/sub-shells).
const HERMES_INSTALL_UNIX: &str = "bash -c 'tmp=$(mktemp) && curl -fsSL \
    https://raw.githubusercontent.com/NousResearch/hermes-agent/main/scripts/install.sh \
    -o $tmp && bash $tmp; status=$?; rm -f $tmp; exit $status'";

const HERMES_UPDATE_UNIX: &str = "hermes update || bash -c 'tmp=$(mktemp) && curl -fsSL \
    https://raw.githubusercontent.com/NousResearch/hermes-agent/main/scripts/install.sh \
    -o $tmp && bash $tmp; status=$?; rm -f $tmp; exit $status'";

#[cfg(target_os = "windows")]
const HERMES_INSTALL_WINDOWS: &str = r#"powershell -NoProfile -ExecutionPolicy Bypass -Command "irm https://raw.githubusercontent.com/NousResearch/hermes-agent/main/scripts/install.ps1 | iex""#;

#[cfg(target_os = "windows")]
const HERMES_UPDATE_WINDOWS: &str = r#"hermes update || powershell -NoProfile -ExecutionPolicy Bypass -Command "irm https://raw.githubusercontent.com/NousResearch/hermes-agent/main/scripts/install.ps1 | iex""#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HermesLifecycleAction {
    Install,
    Update,
}

/// Shell command line for install or update on the current platform.
pub fn hermes_shell_command(action: HermesLifecycleAction) -> String {
    match action {
        HermesLifecycleAction::Install => hermes_install_shell_command(),
        HermesLifecycleAction::Update => hermes_update_shell_command(),
    }
}

pub fn hermes_install_shell_command() -> String {
    #[cfg(target_os = "windows")]
    {
        HERMES_INSTALL_WINDOWS.to_string()
    }
    #[cfg(not(target_os = "windows"))]
    {
        HERMES_INSTALL_UNIX.to_string()
    }
}

pub fn hermes_update_shell_command() -> String {
    #[cfg(target_os = "windows")]
    {
        HERMES_UPDATE_WINDOWS.to_string()
    }
    #[cfg(not(target_os = "windows"))]
    {
        HERMES_UPDATE_UNIX.to_string()
    }
}

/// Run the official Hermes install or update script and return on success.
pub fn run_hermes_lifecycle(action: HermesLifecycleAction) -> Result<()> {
    let command_line = hermes_shell_command(action);
    run_shell_command(&command_line).with_context(|| {
        format!(
            "Hermes {} failed",
            match action {
                HermesLifecycleAction::Install => "install",
                HermesLifecycleAction::Update => "update",
            }
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix_install_uses_temp_file_not_pipe() {
        let cmd = hermes_install_shell_command();
        assert!(cmd.contains("mktemp"));
        assert!(cmd.contains("install.sh"));
        assert!(!cmd.contains("curl -fsSL") || cmd.contains("-o $tmp"));
    }

    #[test]
    fn unix_update_tries_cli_first() {
        let cmd = hermes_update_shell_command();
        assert!(cmd.starts_with("hermes update"));
        assert!(cmd.contains("||"));
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn install_command_matches_cc_switch_posix_shape() {
        assert_eq!(
            hermes_install_shell_command(),
            HERMES_INSTALL_UNIX.to_string()
        );
    }
}
