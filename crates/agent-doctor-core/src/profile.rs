use std::path::PathBuf;

/// Default company/agent profile env file written by `agent-doctor setup`.
pub fn agent_profile_path() -> Option<PathBuf> {
    dirs::config_dir().map(|base| base.join("agent-doctor").join("profile.env"))
}
