use anyhow::{bail, Result};

pub fn run(runtime: &str) -> Result<()> {
    let _ = runtime;
    bail!("`agent-doctor repair <runtime>` is not implemented yet — planned flow: backup, diagnose, apply confirmed fixes, then verify")
}
