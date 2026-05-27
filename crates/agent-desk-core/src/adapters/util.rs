use std::path::PathBuf;
use std::process::Command;

use crate::adapter::AdapterDiscovery;

pub fn home_join(relative: &str) -> PathBuf {
    dirs::home_dir()
        .expect("home directory")
        .join(relative)
}

pub fn find_binary(name: &str) -> Option<PathBuf> {
    if let Some(path) = find_in_path(name) {
        return Some(path);
    }

    for dir in common_binary_dirs() {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }

    None
}

fn find_in_path(name: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path_var) {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

fn common_binary_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![
        PathBuf::from("/opt/homebrew/bin"),
        PathBuf::from("/usr/local/bin"),
    ];

    if let Some(home) = dirs::home_dir() {
        dirs.push(home.join(".local/bin"));
        dirs.push(home.join(".cargo/bin"));
        dirs.push(home.join("bin"));
    }

    dirs
}

pub fn discover_binary(name: &str) -> AdapterDiscovery {
    let binary_path = find_binary(name);
    let installed = binary_path.is_some();
    let version = binary_path
        .as_ref()
        .and_then(|path| read_version(path, &["--version", "-V", "version"]));

    AdapterDiscovery {
        installed,
        version,
        binary_path,
    }
}

fn read_version(binary: &PathBuf, flags: &[&str]) -> Option<String> {
    for flag in flags {
        let output = Command::new(binary).arg(flag).output().ok()?;
        if !output.status.success() {
            continue;
        }
        let text = String::from_utf8_lossy(&output.stdout);
        let line = text.lines().next()?.trim();
        if !line.is_empty() {
            return Some(line.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_hermes_in_local_bin_without_shell_path() {
        let path = find_binary("hermes");
        assert!(
            path.is_some(),
            "expected hermes under ~/.local/bin or similar"
        );
    }
}
