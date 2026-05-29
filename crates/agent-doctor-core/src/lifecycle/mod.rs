pub mod hermes;
pub mod openclaw;
mod runner;

pub use hermes::{hermes_shell_command, run_hermes_lifecycle, HermesLifecycleAction};
pub use openclaw::{openclaw_shell_command, run_openclaw_lifecycle, OpenClawLifecycleAction};
