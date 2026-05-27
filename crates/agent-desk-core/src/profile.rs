use std::path::PathBuf;

/// Default company/agent profile env file written by `agent-desk setup`.
pub fn agent_profile_path() -> Option<PathBuf> {
    dirs::config_dir().map(|base| base.join("agent-desk").join("profile.env"))
}
