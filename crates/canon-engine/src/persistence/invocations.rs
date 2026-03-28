use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::domain::approval::ApprovalRecord;
use crate::domain::execution::{
    InvocationAttempt, InvocationPolicyDecision, InvocationRequest, PayloadReference,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedInvocation {
    pub request: InvocationRequest,
    pub decision: InvocationPolicyDecision,
    pub attempts: Vec<InvocationAttempt>,
    pub approvals: Vec<ApprovalRecord>,
}

pub fn invocation_dir(run_dir: &Path, request_id: &str) -> PathBuf {
    run_dir.join("invocations").join(request_id)
}

pub fn request_path(run_dir: &Path, request_id: &str) -> PathBuf {
    invocation_dir(run_dir, request_id).join("request.toml")
}

pub fn decision_path(run_dir: &Path, request_id: &str) -> PathBuf {
    invocation_dir(run_dir, request_id).join("decision.toml")
}

pub fn attempt_path(run_dir: &Path, request_id: &str, attempt_number: u32) -> PathBuf {
    invocation_dir(run_dir, request_id).join(format!("attempt-{attempt_number:02}.toml"))
}

pub fn payload_dir(run_dir: &Path, request_id: &str) -> PathBuf {
    invocation_dir(run_dir, request_id).join("payload")
}

pub fn payload_path(run_dir: &Path, request_id: &str, file_name: &str) -> PathBuf {
    payload_dir(run_dir, request_id).join(file_name)
}

pub fn payload_reference(run_id: &str, request_id: &str, file_name: &str) -> PayloadReference {
    PayloadReference {
        path: format!("runs/{run_id}/invocations/{request_id}/payload/{file_name}"),
        digest: None,
    }
}

pub fn list_invocation_ids(run_dir: &Path) -> Result<Vec<String>, Error> {
    let root = run_dir.join("invocations");
    if !root.exists() {
        return Ok(Vec::new());
    }

    let mut entries = fs::read_dir(root)?
        .map(|entry| entry.map(|entry| entry.file_name().to_string_lossy().to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort();
    Ok(entries)
}
