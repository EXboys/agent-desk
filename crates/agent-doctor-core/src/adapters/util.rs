use std::path::PathBuf;
use std::process::Command;

use crate::adapter::AdapterDiscovery;

pub fn home_join(relative: &str) -> PathBuf {
    dirs::home_dir().expect("home directory").join(relative)
}

pub fn find_binary(name: &str) -> Option<PathBuf> {
    find_in_path(name)
        .or_else(|| find_binary_in_dirs(name, &common_binary_dirs()))
        .or_else(|| find_with_where_exe(name))
}

fn find_in_path(name: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    find_binary_in_dirs(name, &std::env::split_paths(&path_var).collect::<Vec<_>>())
}

fn find_binary_in_dirs(name: &str, dirs: &[PathBuf]) -> Option<PathBuf> {
    for dir in dirs {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
        #[cfg(target_os = "windows")]
        {
            let exe_candidate = dir.join(format!("{name}.exe"));
            if exe_candidate.is_file() {
                return Some(exe_candidate);
            }
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

/// Use `where.exe` on Windows to find executables that may be in restricted
/// directories (e.g. WindowsApps) where read_dir() would fail.
fn find_with_where_exe(name: &str) -> Option<PathBuf> {
    let output = Command::new("where").arg(name).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let first_line = stdout.lines().next()?.trim();
    if first_line.is_empty() {
        return None;
    }
    let candidate = PathBuf::from(first_line);
    candidate.is_file().then_some(candidate)
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
    use std::fs;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    fn write_executable(path: &PathBuf) {
        fs::write(path, b"#!/bin/sh\nexit 0\n").unwrap();
        #[cfg(unix)]
        {
            let mut perms = fs::metadata(path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(path, perms).unwrap();
        }
        #[cfg(windows)]
        {
            let _ = fs::metadata(path);
        }
    }

    #[test]
    fn finds_binary_in_supplemental_dirs() {
        let temp = tempfile::tempdir().expect("tempdir");
        let bin = temp.path().join("agent-doctor-probe");
        write_executable(&bin);

        let found = find_binary_in_dirs("agent-doctor-probe", &[temp.path().to_path_buf()]);
        assert_eq!(found, Some(bin));
    }

    #[test]
    fn common_binary_dirs_includes_home_local_bin() {
        let dirs = common_binary_dirs();
        let home = dirs::home_dir().expect("home");
        assert!(dirs.contains(&home.join(".local/bin")));
        assert!(dirs.contains(&PathBuf::from("/usr/local/bin")));
    }
}
