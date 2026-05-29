use std::io::Write;
use std::path::PathBuf;
use std::process::Output;

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct ShellCapture {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

impl ShellCapture {
    pub fn combined_output(&self) -> String {
        if self.stderr.trim().is_empty() {
            self.stdout.clone()
        } else if self.stdout.trim().is_empty() {
            self.stderr.clone()
        } else {
            format!("{}\n{}", self.stdout.trim(), self.stderr.trim())
        }
    }
}

pub(crate) fn run_shell_command(command_line: &str) -> Result<()> {
    match run_shell_command_capturing(command_line) {
        Ok(capture) if capture.success => Ok(()),
        Ok(capture) => Err(finish_lifecycle_error(&capture)),
        Err(error) => Err(error),
    }
}

pub fn run_shell_command_capturing(command_line: &str) -> Result<ShellCapture> {
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

    Ok(capture_from_output(&output))
}

fn capture_from_output(output: &Output) -> ShellCapture {
    ShellCapture {
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        exit_code: output.status.code(),
    }
}

fn finish_lifecycle_error(capture: &ShellCapture) -> anyhow::Error {
    let raw = capture.combined_output();
    let detail = last_lines(&raw, 8);
    if detail.is_empty() {
        anyhow::anyhow!("installer exited with status {:?}", capture.exit_code)
    } else {
        anyhow::anyhow!("{detail}")
    }
}

fn last_lines(text: &str, n: usize) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines.len().saturating_sub(n);
    lines[start..].join("\n")
}

pub fn write_install_log(runtime_id: &str, capture: &ShellCapture) -> Result<PathBuf> {
    let root = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("agent-doctor")
        .join("logs");
    std::fs::create_dir_all(&root).context("failed to create log directory")?;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    let path = root.join(format!("install-{runtime_id}-{timestamp}.log"));

    let mut file = std::fs::File::create(&path).context("failed to create install log")?;
    writeln!(file, "exit_code={:?}", capture.exit_code)?;
    writeln!(file, "--- stdout ---")?;
    write!(file, "{}", capture.stdout)?;
    writeln!(file, "--- stderr ---")?;
    write!(file, "{}", capture.stderr)?;

    Ok(path)
}
