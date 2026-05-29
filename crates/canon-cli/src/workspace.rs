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
    use std::env;
    use std::fs;
    use std::sync::{Mutex, OnceLock};

    use tempfile::tempdir;

    use super::{WorkspaceResolutionError, discover_repo_root, resolve_repo_root};

    fn cwd_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

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

    #[test]
    fn discover_repo_root_returns_start_path_when_no_repo_markers_exist() {
        let temp = tempdir().unwrap();
        let child = temp.path().join("scratch/nested");
        fs::create_dir_all(&child).unwrap();

        let resolved = discover_repo_root(&child).unwrap();

        assert_eq!(resolved, child.canonicalize().unwrap());
    }

    #[test]
    fn discover_repo_root_rejects_ambiguous_nested_canon_roots() {
        let temp = tempdir().unwrap();
        let repo = temp.path().join("repo");
        let nested = repo.join("nested");
        let child = nested.join("src");
        fs::create_dir_all(repo.join(".canon")).unwrap();
        fs::create_dir_all(nested.join(".canon")).unwrap();
        fs::create_dir_all(&child).unwrap();

        let error = discover_repo_root(&child).expect_err("multiple .canon roots should fail");

        match error {
            WorkspaceResolutionError::Ambiguous(path) => {
                assert_eq!(path, child.canonicalize().unwrap());
            }
            other => panic!("expected ambiguous workspace error, got {other:?}"),
        }
    }

    #[test]
    fn resolve_repo_root_uses_current_directory_search() {
        let _guard = cwd_lock().lock().unwrap();
        let original = env::current_dir().unwrap();

        let temp = tempdir().unwrap();
        let repo = temp.path().join("repo");
        let child = repo.join("src/bin");
        fs::create_dir_all(&child).unwrap();
        fs::create_dir_all(repo.join(".git")).unwrap();

        env::set_current_dir(&child).unwrap();
        let resolved = resolve_repo_root();
        env::set_current_dir(original).unwrap();

        assert_eq!(resolved.unwrap(), repo.canonicalize().unwrap());
    }
}
