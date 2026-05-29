pub mod hermes;
pub mod openclaw;
mod runner;

pub use hermes::{
    hermes_install_shell_command, hermes_shell_command, run_hermes_lifecycle, HermesLifecycleAction,
};
pub use openclaw::{
    openclaw_install_shell_command, openclaw_shell_command, run_openclaw_lifecycle,
    OpenClawLifecycleAction,
};
pub use runner::{run_shell_command_capturing, write_install_log, ShellCapture};
