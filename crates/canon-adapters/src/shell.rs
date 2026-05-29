use std::path::Path;
use std::process::Command;

use time::OffsetDateTime;

use crate::dispatcher;
use crate::{
    AdapterError, AdapterInvocation, AdapterKind, AdapterRequest, CapabilityKind,
    InvocationOrientation, LineageClass, SideEffectClass, TrustBoundaryKind,
};

/// The result of executing a shell command via [`ShellAdapter`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellOutput {
    /// The process exit status code.
    pub status_code: i32,
    /// Captured standard output.
    pub stdout: String,
    /// Captured standard error.
    pub stderr: String,
    /// The invocation audit record for this command.
    pub invocation: AdapterInvocation,
}

/// The result of a `git diff` operation, including changed file listings and the raw patch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitDiff {
    /// The base ref the diff is computed against.
    pub base_ref: String,
    /// The head ref being compared.
    pub head_ref: String,
    /// Paths of all changed files.
    pub changed_files: Vec<String>,
    /// Newline-separated list of changed file paths as a single string.
    pub changed_files_text: String,
    /// The full unified diff patch.
    pub patch: String,
    /// Invocation audit records for the git commands that produced this diff.
    pub invocations: Vec<AdapterInvocation>,
}

/// Adapter that wraps `std::process::Command` for governed shell command execution.
///
/// All methods return either a [`ShellOutput`] or a [`GitDiff`] alongside the
/// invocation audit records required for the evidence bundle.
#[derive(Debug, Default)]
pub struct ShellAdapter;

impl ShellAdapter {
    /// Returns a read-only shell command request with the given purpose.
    pub fn read_only_request(&self, purpose: &str) -> AdapterRequest {
        AdapterRequest {
            adapter: AdapterKind::Shell,
            capability: CapabilityKind::RunCommand,
            purpose: purpose.to_string(),
            orientation: Some(InvocationOrientation::Context),
            trust_boundary: Some(TrustBoundaryKind::LocalProcess),
            lineage: Some(LineageClass::NonGenerative),
            side_effect: SideEffectClass::ReadOnly,
        }
    }

    /// Returns a workspace-mutation shell command request with the given purpose.
    pub fn mutating_request(&self, purpose: &str) -> AdapterRequest {
        AdapterRequest {
            adapter: AdapterKind::Shell,
            capability: CapabilityKind::ExecuteBoundedTransformation,
            purpose: purpose.to_string(),
            orientation: Some(InvocationOrientation::Generation),
            trust_boundary: Some(TrustBoundaryKind::LocalProcess),
            lineage: Some(LineageClass::NonGenerative),
            side_effect: SideEffectClass::WorkspaceMutation,
        }
    }

    /// Returns a validation shell command request with the given purpose.
    pub fn validation_request(&self, purpose: &str) -> AdapterRequest {
        AdapterRequest {
            adapter: AdapterKind::Shell,
            capability: CapabilityKind::ValidateWithTool,
            purpose: purpose.to_string(),
            orientation: Some(InvocationOrientation::Validation),
            trust_boundary: Some(TrustBoundaryKind::LocalProcess),
            lineage: Some(LineageClass::NonGenerative),
            side_effect: SideEffectClass::ReadOnly,
        }
    }

    /// Returns an inspect-diff shell command request with the given purpose.
    pub fn inspect_diff_request(&self, purpose: &str) -> AdapterRequest {
        AdapterRequest {
            adapter: AdapterKind::Shell,
            capability: CapabilityKind::InspectDiff,
            purpose: purpose.to_string(),
            orientation: Some(InvocationOrientation::Context),
            trust_boundary: Some(TrustBoundaryKind::LocalProcess),
            lineage: Some(LineageClass::NonGenerative),
            side_effect: SideEffectClass::ReadOnly,
        }
    }

    /// Executes a shell command, enforcing the mutation policy, and returns the output.
    pub fn run(
        &self,
        request: &AdapterRequest,
        program: &str,
        args: &[&str],
        cwd: Option<&Path>,
        allow_mutation: bool,
    ) -> Result<ShellOutput, AdapterError> {
        dispatcher::enforce_mutation_policy(request, allow_mutation)?;

        let mut command = Command::new(program);
        command.args(args);
        if let Some(cwd) = cwd {
            command.current_dir(cwd);
        }

        let output = command.output().map_err(AdapterError::Process)?;
        Ok(ShellOutput {
            status_code: output.status.code().unwrap_or_default(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            invocation: AdapterInvocation {
                adapter: request.adapter,
                capability: request.capability,
                purpose: request.purpose.clone(),
                side_effect: request.side_effect,
                allowed: true,
                occurred_at: OffsetDateTime::now_utc(),
            },
        })
    }

    /// Runs a `git diff` between `base_ref` and `head_ref`, returning changed files and the patch.
    pub fn git_diff(
        &self,
        base_ref: &str,
        head_ref: &str,
        cwd: &Path,
    ) -> Result<GitDiff, AdapterError> {
        let list_request = self.inspect_diff_request("collect changed surfaces for pr-review");
        let names_output = self.run_checked(
            &list_request,
            "git",
            &["diff", "--name-only", base_ref, head_ref],
            Some(cwd),
            false,
        )?;

        let diff_request = self.inspect_diff_request("collect diff patch for pr-review");
        let patch_output = self.run_checked(
            &diff_request,
            "git",
            &["diff", "--unified=0", base_ref, head_ref],
            Some(cwd),
            false,
        )?;

        let changed_files = names_output
            .stdout
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        Ok(GitDiff {
            base_ref: base_ref.to_string(),
            head_ref: head_ref.to_string(),
            changed_files,
            changed_files_text: names_output.stdout,
            patch: patch_output.stdout,
            invocations: vec![names_output.invocation, patch_output.invocation],
        })
    }

    /// Diff the working tree (staged + unstaged) against a base ref.
    pub fn git_diff_worktree(&self, base_ref: &str, cwd: &Path) -> Result<GitDiff, AdapterError> {
        let list_request =
            self.inspect_diff_request("collect changed surfaces for pr-review worktree diff");
        let names_output = self.run_checked(
            &list_request,
            "git",
            &["diff", "--name-only", base_ref],
            Some(cwd),
            false,
        )?;

        let diff_request =
            self.inspect_diff_request("collect diff patch for pr-review worktree diff");
        let patch_output = self.run_checked(
            &diff_request,
            "git",
            &["diff", "--unified=0", base_ref],
            Some(cwd),
            false,
        )?;

        let changed_files = names_output
            .stdout
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        Ok(GitDiff {
            base_ref: base_ref.to_string(),
            head_ref: "WORKTREE".to_string(),
            changed_files,
            changed_files_text: names_output.stdout,
            patch: patch_output.stdout,
            invocations: vec![names_output.invocation, patch_output.invocation],
        })
    }

    /// Check whether the working tree has uncommitted changes (staged or unstaged).
    pub fn has_uncommitted_changes(&self, cwd: &Path) -> Result<bool, AdapterError> {
        let request =
            self.inspect_diff_request("check for uncommitted changes in the working tree");
        let output =
            self.run_checked(&request, "git", &["status", "--porcelain"], Some(cwd), false)?;
        Ok(!output.stdout.trim().is_empty())
    }

    fn run_checked(
        &self,
        request: &AdapterRequest,
        program: &str,
        args: &[&str],
        cwd: Option<&Path>,
        allow_mutation: bool,
    ) -> Result<ShellOutput, AdapterError> {
        let output = self.run(request, program, args, cwd, allow_mutation)?;
        if output.status_code != 0 {
            return Err(AdapterError::Process(std::io::Error::other(format!(
                "{program} {:?} failed with code {}: {}",
                args,
                output.status_code,
                output.stderr.trim()
            ))));
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::process::Command as ProcessCommand;

    use tempfile::tempdir;

    use super::ShellAdapter;
    use crate::{
        AdapterError, CapabilityKind, InvocationOrientation, LineageClass, SideEffectClass,
        TrustBoundaryKind,
    };

    fn git(cwd: &Path, args: &[&str]) {
        let output = ProcessCommand::new("git")
            .args(args)
            .current_dir(cwd)
            .output()
            .expect("git command should run");

        assert!(
            output.status.success(),
            "git {:?} failed: stdout=`{}` stderr=`{}`",
            args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn init_repo() -> tempfile::TempDir {
        let repo = tempdir().expect("temp repo");
        git(repo.path(), &["init", "-b", "main"]);
        git(repo.path(), &["config", "user.name", "Coverage Tester"]);
        git(repo.path(), &["config", "user.email", "coverage@example.com"]);
        repo
    }

    fn commit_all(cwd: &Path, message: &str) {
        git(cwd, &["add", "."]);
        git(cwd, &["-c", "commit.gpgsign=false", "commit", "-m", message]);
    }

    #[test]
    fn request_builders_assign_expected_capability_and_side_effect() {
        let adapter = ShellAdapter;

        let read_only = adapter.read_only_request("inspect repo state");
        assert_eq!(read_only.capability, CapabilityKind::RunCommand);
        assert_eq!(read_only.orientation, Some(InvocationOrientation::Context));
        assert_eq!(read_only.trust_boundary, Some(TrustBoundaryKind::LocalProcess));
        assert_eq!(read_only.lineage, Some(LineageClass::NonGenerative));
        assert_eq!(read_only.side_effect, SideEffectClass::ReadOnly);

        let mutating = adapter.mutating_request("rewrite file");
        assert_eq!(mutating.capability, CapabilityKind::ExecuteBoundedTransformation);
        assert_eq!(mutating.orientation, Some(InvocationOrientation::Generation));
        assert_eq!(mutating.trust_boundary, Some(TrustBoundaryKind::LocalProcess));
        assert_eq!(mutating.lineage, Some(LineageClass::NonGenerative));
        assert_eq!(mutating.side_effect, SideEffectClass::WorkspaceMutation);
    }

    #[test]
    fn git_diff_collects_changed_files_and_patch_between_refs() {
        let repo = init_repo();
        fs::write(repo.path().join("notes.md"), "one\n").expect("write first version");
        commit_all(repo.path(), "initial");

        fs::write(repo.path().join("notes.md"), "one\ntwo\n").expect("write second version");
        commit_all(repo.path(), "update");

        let adapter = ShellAdapter;
        let diff =
            adapter.git_diff("HEAD~1", "HEAD", repo.path()).expect("git diff should succeed");

        assert_eq!(diff.base_ref, "HEAD~1");
        assert_eq!(diff.head_ref, "HEAD");
        assert_eq!(diff.changed_files, vec!["notes.md"]);
        assert!(diff.changed_files_text.contains("notes.md"));
        assert!(diff.patch.contains("notes.md"));
        assert_eq!(diff.invocations.len(), 2);
        assert!(diff.invocations.iter().all(|invocation| invocation.allowed));
    }

    #[test]
    fn git_diff_worktree_reports_uncommitted_changes() {
        let repo = init_repo();
        fs::write(repo.path().join("worktree.md"), "before\n").expect("write first version");
        commit_all(repo.path(), "initial");

        fs::write(repo.path().join("worktree.md"), "before\nafter\n")
            .expect("write worktree change");

        let adapter = ShellAdapter;
        let diff =
            adapter.git_diff_worktree("HEAD", repo.path()).expect("worktree diff should succeed");

        assert_eq!(diff.base_ref, "HEAD");
        assert_eq!(diff.head_ref, "WORKTREE");
        assert_eq!(diff.changed_files, vec!["worktree.md"]);
        assert!(diff.patch.contains("worktree.md"));
        assert!(adapter.has_uncommitted_changes(repo.path()).expect("status should succeed"));
    }

    #[test]
    fn run_checked_returns_process_error_for_non_zero_exit_status() {
        let adapter = ShellAdapter;
        let request = adapter.validation_request("force shell adapter failure path");

        let error = adapter
            .run_checked(&request, "sh", &["-c", "printf boom >&2; exit 7"], None, false)
            .expect_err("non-zero exit status should fail");

        match error {
            AdapterError::Process(io_error) => {
                let message = io_error.to_string();
                assert!(message.contains("failed with code 7"));
                assert!(message.contains("boom"));
            }
            other => panic!("expected process error, got {other:?}"),
        }

        assert_eq!(request.side_effect, SideEffectClass::ReadOnly);
    }
}
