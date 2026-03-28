use std::path::{Path, PathBuf};

use crate::domain::mode::Mode;

#[derive(Debug, Clone)]
pub struct ProjectLayout {
    pub repo_root: PathBuf,
    pub canon_root: PathBuf,
}

impl ProjectLayout {
    pub fn new(repo_root: impl AsRef<Path>) -> Self {
        let repo_root = repo_root.as_ref().to_path_buf();
        Self { canon_root: repo_root.join(".canon"), repo_root }
    }

    pub fn sessions_dir(&self) -> PathBuf {
        self.canon_root.join("sessions")
    }

    pub fn artifacts_dir(&self) -> PathBuf {
        self.canon_root.join("artifacts")
    }

    pub fn decisions_dir(&self) -> PathBuf {
        self.canon_root.join("decisions")
    }

    pub fn traces_dir(&self) -> PathBuf {
        self.canon_root.join("traces")
    }

    pub fn methods_dir(&self) -> PathBuf {
        self.canon_root.join("methods")
    }

    pub fn policies_dir(&self) -> PathBuf {
        self.canon_root.join("policies")
    }

    pub fn runs_dir(&self) -> PathBuf {
        self.canon_root.join("runs")
    }

    pub fn run_dir(&self, run_id: &str) -> PathBuf {
        self.runs_dir().join(run_id)
    }

    pub fn run_gates_dir(&self, run_id: &str) -> PathBuf {
        self.run_dir(run_id).join("gates")
    }

    pub fn run_approvals_dir(&self, run_id: &str) -> PathBuf {
        self.run_dir(run_id).join("approvals")
    }

    pub fn run_verification_dir(&self, run_id: &str) -> PathBuf {
        self.run_dir(run_id).join("verification")
    }

    pub fn run_invocations_dir(&self, run_id: &str) -> PathBuf {
        self.run_dir(run_id).join("invocations")
    }

    pub fn run_invocation_dir(&self, run_id: &str, request_id: &str) -> PathBuf {
        self.run_invocations_dir(run_id).join(request_id)
    }

    pub fn run_evidence_path(&self, run_id: &str) -> PathBuf {
        self.run_dir(run_id).join("evidence.toml")
    }

    pub fn run_artifact_dir(&self, run_id: &str, mode: Mode) -> PathBuf {
        self.artifacts_dir().join(run_id).join(mode.as_str())
    }
}
