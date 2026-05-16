use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkspaceResolutionError {
    #[error("failed to determine current directory: {0}")]
    CurrentDir(#[from] std::io::Error),
    #[error("ambiguous Canon workspace: multiple .canon/ directories found above {0}")]
    Ambiguous(PathBuf),
}

pub fn resolve_repo_root() -> Result<PathBuf, WorkspaceResolutionError> {
    let cwd = std::env::current_dir()?;
    discover_repo_root(&cwd)
}

pub fn discover_repo_root(start: &Path) -> Result<PathBuf, WorkspaceResolutionError> {
    let start = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());

    let canon_candidates = search_upward_all_dirs(&start, ".canon");
    if canon_candidates.len() > 1 {
        return Err(WorkspaceResolutionError::Ambiguous(start));
    }
    if let Some(found) = canon_candidates.into_iter().next() {
        return Ok(found);
    }

    if let Some(found) = search_upward_entry(&start, ".git") {
        return Ok(found);
    }

    Ok(start)
}

fn search_upward_entry(start: &Path, target: &str) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if current.join(target).exists() {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

fn search_upward_all_dirs(start: &Path, target: &str) -> Vec<PathBuf> {
    let mut current = start.to_path_buf();
    let mut matches = Vec::new();
    loop {
        if current.join(target).is_dir() {
            matches.push(current.clone());
        }
        if !current.pop() {
            return matches;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::discover_repo_root;

    #[test]
    fn discover_repo_root_prefers_canon_root_over_git_root() {
        let temp = tempdir().unwrap();
        let git_root = temp.path().join("repo");
        let canon_root = git_root.join("nested");
        let child = canon_root.join("src/subdir");
        fs::create_dir_all(canon_root.join(".canon")).unwrap();
        fs::create_dir_all(git_root.join(".git")).unwrap();
        fs::create_dir_all(&child).unwrap();

        let resolved = discover_repo_root(&child).unwrap();

        assert_eq!(resolved, canon_root.canonicalize().unwrap());
    }

    #[test]
    fn discover_repo_root_accepts_git_worktree_marker_files() {
        let temp = tempdir().unwrap();
        let git_root = temp.path().join("repo");
        let child = git_root.join("crates/canon-cli");
        fs::create_dir_all(&child).unwrap();
        fs::write(git_root.join(".git"), "gitdir: /tmp/worktree\n").unwrap();

        let resolved = discover_repo_root(&child).unwrap();

        assert_eq!(resolved, git_root.canonicalize().unwrap());
    }
}
