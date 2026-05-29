use std::process::Output;

use anyhow::{bail, Context, Result};

pub(crate) fn run_shell_command(command_line: &str) -> Result<()> {
    use std::process::Command;

    #[cfg(unix)]
    let output = Command::new("bash")
        .arg("-c")
        .arg(command_line)
        .output()
        .context("failed to start install shell")?;

    #[cfg(windows)]
    let output = Command::new("cmd")
        .args(["/C", command_line])
        .output()
        .context("failed to start install shell")?;

    finish_lifecycle_output(&output)
}

pub(crate) fn finish_lifecycle_output(output: &Output) -> Result<()> {
    if output.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let raw = if stderr.trim().is_empty() {
        stdout.trim()
    } else {
        stderr.trim()
    };
    let detail = last_lines(raw, 8);
    if detail.is_empty() {
        bail!("installer exited with status {:?}", output.status.code());
    }
    bail!("{detail}");
}

fn last_lines(text: &str, n: usize) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines.len().saturating_sub(n);
    lines[start..].join("\n")
}
