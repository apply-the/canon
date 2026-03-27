use std::path::Path;
use std::process::Command;

use time::OffsetDateTime;

use crate::dispatcher;
use crate::{
    AdapterError, AdapterInvocation, AdapterKind, AdapterRequest, CapabilityKind, SideEffectClass,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellOutput {
    pub status_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub invocation: AdapterInvocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitDiff {
    pub base_ref: String,
    pub head_ref: String,
    pub changed_files: Vec<String>,
    pub patch: String,
    pub invocations: Vec<AdapterInvocation>,
}

#[derive(Debug, Default)]
pub struct ShellAdapter;

impl ShellAdapter {
    pub fn read_only_request(&self, purpose: &str) -> AdapterRequest {
        AdapterRequest {
            adapter: AdapterKind::Shell,
            capability: CapabilityKind::ExecReadOnlyCommand,
            purpose: purpose.to_string(),
            side_effect: SideEffectClass::ReadOnly,
        }
    }

    pub fn mutating_request(&self, purpose: &str) -> AdapterRequest {
        AdapterRequest {
            adapter: AdapterKind::Shell,
            capability: CapabilityKind::ExecMutatingCommand,
            purpose: purpose.to_string(),
            side_effect: SideEffectClass::WorkspaceMutation,
        }
    }

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

    pub fn git_diff(
        &self,
        base_ref: &str,
        head_ref: &str,
        cwd: &Path,
    ) -> Result<GitDiff, AdapterError> {
        let list_request = self.read_only_request("collect changed surfaces for pr-review");
        let names_output = self.run_checked(
            &list_request,
            "git",
            &["diff", "--name-only", base_ref, head_ref],
            Some(cwd),
            false,
        )?;

        let diff_request = self.read_only_request("collect diff patch for pr-review");
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
            patch: patch_output.stdout,
            invocations: vec![names_output.invocation, patch_output.invocation],
        })
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
