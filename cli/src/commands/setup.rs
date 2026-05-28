use anyhow::{bail, Result};

pub fn run(url: &str, key: &str) -> Result<()> {
    let _ = (url, key);
    bail!("`agent-doctor setup` is not implemented yet — track docs/ROADMAP.md")
}
