use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

pub fn resolve_project_path(path: Option<PathBuf>, prefer_git_root: bool) -> Result<PathBuf> {
    let start = path.unwrap_or_else(|| std::env::current_dir().expect("current directory"));
    let absolute = if start.is_absolute() {
        start
    } else {
        std::env::current_dir()
            .context("current directory")?
            .join(start)
    };
    let canonical = absolute
        .canonicalize()
        .with_context(|| format!("invalid project path: {}", absolute.display()))?;

    if prefer_git_root {
        if let Some(root) = find_git_root(&canonical) {
            return Ok(root);
        }
    }
    Ok(canonical)
}

pub fn find_git_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if current.join(".git").exists() {
            return Some(current.canonicalize().unwrap_or(current));
        }
        if !current.pop() {
            return None;
        }
    }
}

pub fn sanitize_workspace_name(input: &str) -> String {
    let mut out = String::new();
    let mut prev_hyphen = false;
    for ch in input.chars() {
        let lower = ch.to_ascii_lowercase();
        if lower.is_ascii_alphanumeric() {
            out.push(lower);
            prev_hyphen = false;
        } else if !prev_hyphen && !out.is_empty() {
            out.push('-');
            prev_hyphen = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        "project".to_string()
    } else {
        out
    }
}

pub fn default_workspace_name(project_path: &Path) -> String {
    project_path
        .file_name()
        .and_then(|name| name.to_str())
        .map(sanitize_workspace_name)
        .unwrap_or_else(|| "project".to_string())
}

pub fn paths_equal(a: &Path, b: &Path) -> bool {
    a.canonicalize()
        .ok()
        .zip(b.canonicalize().ok())
        .map(|(left, right)| left == right)
        .unwrap_or(false)
}

pub fn cwd() -> Result<PathBuf> {
    std::env::current_dir().context("current working directory")
}

pub fn ensure_unique_name(
    name: &str,
    entries: &[(String, PathBuf)],
    path: &Path,
) -> Result<String> {
    let base = sanitize_workspace_name(name);
    if entries.iter().any(|(existing_name, existing_path)| {
        existing_name == &base && !paths_equal(existing_path, path)
    }) {
        bail!("workspace name '{base}' is already used by another project");
    }
    Ok(base)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn sanitize_workspace_name_strips_invalid_chars() {
        assert_eq!(sanitize_workspace_name("My App!"), "my-app");
        assert_eq!(sanitize_workspace_name("---foo---"), "foo");
    }

    #[test]
    fn find_git_root_walks_upward() {
        let temp = TempDir::new().expect("tempdir");
        let repo = temp.path().join("repo");
        let nested = repo.join("packages").join("api");
        fs::create_dir_all(&nested).unwrap();
        fs::create_dir_all(repo.join(".git")).unwrap();
        assert_eq!(find_git_root(&nested), Some(repo.canonicalize().unwrap()));
    }
}
