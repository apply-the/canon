use std::path::{Path, PathBuf};

use thiserror::Error;

pub const CANON_ROOT_ENV_VAR: &str = "CANON_ROOT";
pub const CANON_REPO_ROOT_ENV_VAR: &str = "CANON_REPO_ROOT";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepoRootRequirement {
    Optional,
    #[cfg_attr(not(test), allow(dead_code))]
    Required,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootResolutionOptions {
    pub current_working_directory: Option<PathBuf>,
    pub canon_root_override: Option<PathBuf>,
    pub repo_root_override: Option<PathBuf>,
    pub repo_root_requirement: RepoRootRequirement,
}

impl RootResolutionOptions {
    pub fn new(repo_root_requirement: RepoRootRequirement) -> Self {
        Self {
            current_working_directory: None,
            canon_root_override: None,
            repo_root_override: None,
            repo_root_requirement,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRoots {
    pub current_working_directory: PathBuf,
    pub canon_root: Option<PathBuf>,
    pub repo_root: Option<PathBuf>,
}

impl ResolvedRoots {
    pub fn canon_root_for_runtime(&self) -> PathBuf {
        self.canon_root
            .clone()
            .or_else(|| self.repo_root.clone())
            .unwrap_or_else(|| self.current_working_directory.clone())
    }

    pub fn effective_repo_root(&self) -> &Path {
        self.repo_root.as_deref().unwrap_or(&self.current_working_directory)
    }
}

#[derive(Debug, Error)]
pub enum WorkspaceResolutionError {
    #[error("failed to determine current directory: {0}")]
    CurrentDir(#[from] std::io::Error),
    #[error("explicit Canon workspace root is not a directory: {0}")]
    CanonRootNotDirectory(PathBuf),
    #[error("explicit repository root is not a directory: {0}")]
    RepoRootNotDirectory(PathBuf),
    #[error(
        "repo-scoped command requires a Git repository root; rerun from inside the target repository or pass --repo-root <path>"
    )]
    MissingRepoRoot,
    #[error(
        "Canon workspace root {canon_root} does not contain repository root {repo_root}; use a parent Canon workspace or pass matching --canon-root/--repo-root values"
    )]
    CanonRootDoesNotContainRepo { canon_root: PathBuf, repo_root: PathBuf },
}

#[cfg(test)]
pub fn resolve_repo_root() -> Result<PathBuf, WorkspaceResolutionError> {
    let roots = resolve_roots(RootResolutionOptions::new(RepoRootRequirement::Required))?;
    roots.repo_root.ok_or(WorkspaceResolutionError::MissingRepoRoot)
}

pub fn resolve_roots(
    options: RootResolutionOptions,
) -> Result<ResolvedRoots, WorkspaceResolutionError> {
    let current_working_directory = match options.current_working_directory {
        Some(path) => normalize_directory_path(path)?,
        None => std::env::current_dir()?,
    };
    let current_working_directory =
        current_working_directory.canonicalize().unwrap_or(current_working_directory);

    let repo_root = match options.repo_root_override {
        Some(path) => {
            Some(normalize_directory_path(resolve_override_path(&current_working_directory, path))?)
        }
        None => discover_git_repo_root(&current_working_directory),
    };
    let canon_root = match options.canon_root_override {
        Some(path) => {
            Some(normalize_directory_path(resolve_override_path(&current_working_directory, path))?)
        }
        None => discover_canon_root(&current_working_directory),
    };

    if matches!(options.repo_root_requirement, RepoRootRequirement::Required) && repo_root.is_none()
    {
        return Err(WorkspaceResolutionError::MissingRepoRoot);
    }

    if let (Some(canon_root), Some(repo_root)) = (&canon_root, &repo_root)
        && !repo_root.starts_with(canon_root)
        && canon_root != repo_root
    {
        return Err(WorkspaceResolutionError::CanonRootDoesNotContainRepo {
            canon_root: canon_root.clone(),
            repo_root: repo_root.clone(),
        });
    }

    Ok(ResolvedRoots { current_working_directory, canon_root, repo_root })
}

pub fn discover_canon_root(start: &Path) -> Option<PathBuf> {
    search_upward_directory(start, ".canon")
}

pub fn discover_git_repo_root(start: &Path) -> Option<PathBuf> {
    search_upward_entry(start, ".git")
}

pub fn env_path(name: &str) -> Option<PathBuf> {
    std::env::var_os(name).map(PathBuf::from)
}

fn resolve_override_path(current_working_directory: &Path, path: PathBuf) -> PathBuf {
    if path.is_absolute() { path } else { current_working_directory.join(path) }
}

fn normalize_directory_path(path: PathBuf) -> Result<PathBuf, WorkspaceResolutionError> {
    let resolved = path.canonicalize().unwrap_or_else(|_| path.clone());
    if resolved.is_dir() {
        Ok(resolved)
    } else if path.file_name().is_some_and(|name| name == ".canon") {
        Err(WorkspaceResolutionError::CanonRootNotDirectory(resolved))
    } else {
        Err(WorkspaceResolutionError::RepoRootNotDirectory(resolved))
    }
}

fn search_upward_entry(start: &Path, target: &str) -> Option<PathBuf> {
    let mut current = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());
    loop {
        if current.join(target).exists() {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

fn search_upward_directory(start: &Path, target: &str) -> Option<PathBuf> {
    let mut current = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());
    loop {
        if current.join(target).is_dir() {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::sync::{Mutex, OnceLock};

    use tempfile::tempdir;

    use super::{
        RepoRootRequirement, RootResolutionOptions, WorkspaceResolutionError, discover_canon_root,
        discover_git_repo_root, resolve_repo_root, resolve_roots,
    };

    fn cwd_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn discover_canon_root_returns_nearest_parent_with_canon_runtime() {
        let temp = tempdir().unwrap();
        let git_root = temp.path().join("repo");
        let canon_root = git_root.join("nested");
        let child = canon_root.join("src/subdir");
        fs::create_dir_all(canon_root.join(".canon")).unwrap();
        fs::create_dir_all(git_root.join(".git")).unwrap();
        fs::create_dir_all(&child).unwrap();

        let resolved = discover_canon_root(&child).unwrap();

        assert_eq!(resolved, canon_root.canonicalize().unwrap());
    }

    #[test]
    fn discover_git_repo_root_accepts_git_worktree_marker_files() {
        let temp = tempdir().unwrap();
        let git_root = temp.path().join("repo");
        let child = git_root.join("crates/canon-cli");
        fs::create_dir_all(&child).unwrap();
        fs::write(git_root.join(".git"), "gitdir: /tmp/worktree\n").unwrap();

        let resolved = discover_git_repo_root(&child).unwrap();

        assert_eq!(resolved, git_root.canonicalize().unwrap());
    }

    #[test]
    fn resolve_roots_uses_parent_canon_when_repo_has_no_local_runtime() {
        let temp = tempdir().unwrap();
        let workspace_root = temp.path().join("workspace");
        let repo_root = workspace_root.join("boundline");
        let child = repo_root.join("src/nested");
        fs::create_dir_all(workspace_root.join(".canon")).unwrap();
        fs::create_dir_all(repo_root.join(".git")).unwrap();
        fs::create_dir_all(&child).unwrap();

        let resolved = resolve_roots(RootResolutionOptions {
            current_working_directory: Some(child.clone()),
            canon_root_override: None,
            repo_root_override: None,
            repo_root_requirement: RepoRootRequirement::Required,
        })
        .unwrap();

        assert_eq!(resolved.canon_root.unwrap(), workspace_root.canonicalize().unwrap());
        assert_eq!(resolved.repo_root.unwrap(), repo_root.canonicalize().unwrap());
    }

    #[test]
    fn resolve_roots_prefers_nearest_canon_root() {
        let temp = tempdir().unwrap();
        let workspace_root = temp.path().join("workspace");
        let nested_workspace = workspace_root.join("boundline");
        let child = nested_workspace.join("src");
        fs::create_dir_all(workspace_root.join(".canon")).unwrap();
        fs::create_dir_all(nested_workspace.join(".canon")).unwrap();
        fs::create_dir_all(&child).unwrap();

        let resolved = resolve_roots(RootResolutionOptions {
            current_working_directory: Some(child.clone()),
            canon_root_override: None,
            repo_root_override: None,
            repo_root_requirement: RepoRootRequirement::Optional,
        })
        .unwrap();

        assert_eq!(resolved.canon_root.unwrap(), nested_workspace.canonicalize().unwrap());
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

    #[test]
    fn resolve_roots_reports_missing_repo_root_for_repo_scoped_commands() {
        let temp = tempdir().unwrap();
        let workspace_root = temp.path().join("workspace");
        fs::create_dir_all(workspace_root.join(".canon")).unwrap();

        let error = resolve_roots(RootResolutionOptions {
            current_working_directory: Some(workspace_root.clone()),
            canon_root_override: None,
            repo_root_override: None,
            repo_root_requirement: RepoRootRequirement::Required,
        })
        .expect_err("repo root should be required");

        assert!(matches!(error, WorkspaceResolutionError::MissingRepoRoot));
    }

    #[test]
    fn resolve_roots_supports_explicit_repo_root_override() {
        let temp = tempdir().unwrap();
        let workspace_root = temp.path().join("workspace");
        let repo_root = workspace_root.join("boundline");
        fs::create_dir_all(workspace_root.join(".canon")).unwrap();
        fs::create_dir_all(repo_root.join(".git")).unwrap();

        let resolved = resolve_roots(RootResolutionOptions {
            current_working_directory: Some(workspace_root.clone()),
            canon_root_override: None,
            repo_root_override: Some(repo_root.clone()),
            repo_root_requirement: RepoRootRequirement::Required,
        })
        .unwrap();

        assert_eq!(resolved.canon_root.unwrap(), workspace_root.canonicalize().unwrap());
        assert_eq!(resolved.repo_root.unwrap(), repo_root.canonicalize().unwrap());
    }

    #[test]
    fn resolve_roots_supports_explicit_canon_root_override() {
        let temp = tempdir().unwrap();
        let workspace_root = temp.path().join("workspace");
        let repo_root = workspace_root.join("boundline");
        let alternate_workspace = temp.path().join("shared-canon-root");
        fs::create_dir_all(workspace_root).unwrap();
        fs::create_dir_all(repo_root.join(".git")).unwrap();
        fs::create_dir_all(&alternate_workspace).unwrap();

        let resolved = resolve_roots(RootResolutionOptions {
            current_working_directory: Some(repo_root.clone()),
            canon_root_override: Some(alternate_workspace.clone()),
            repo_root_override: Some(repo_root.clone()),
            repo_root_requirement: RepoRootRequirement::Required,
        })
        .expect_err("repo outside canon root should fail");

        assert!(matches!(resolved, WorkspaceResolutionError::CanonRootDoesNotContainRepo { .. }));
    }

    #[test]
    fn resolve_roots_keeps_nested_subdirectory_as_current_working_directory() {
        let temp = tempdir().unwrap();
        let workspace_root = temp.path().join("workspace");
        let repo_root = workspace_root.join("canon");
        let child = repo_root.join("crates/canon-cli/src");
        fs::create_dir_all(workspace_root.join(".canon")).unwrap();
        fs::create_dir_all(repo_root.join(".git")).unwrap();
        fs::create_dir_all(&child).unwrap();

        let resolved = resolve_roots(RootResolutionOptions {
            current_working_directory: Some(child.clone()),
            canon_root_override: None,
            repo_root_override: None,
            repo_root_requirement: RepoRootRequirement::Required,
        })
        .unwrap();

        assert_eq!(resolved.current_working_directory, child.canonicalize().unwrap());
        assert_eq!(resolved.repo_root.unwrap(), repo_root.canonicalize().unwrap());
        assert_eq!(resolved.canon_root.unwrap(), workspace_root.canonicalize().unwrap());
    }
}
