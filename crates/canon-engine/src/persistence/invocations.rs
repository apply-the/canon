use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::domain::approval::ApprovalRecord;
use crate::domain::execution::{
    InvocationAttempt, InvocationPolicyDecision, InvocationRequest, PayloadReference,
};

/// A fully persisted invocation record, including the request, decision, attempts, and approvals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedInvocation {
    /// The invocation request.
    pub request: InvocationRequest,
    /// The policy decision applied to this invocation.
    pub decision: InvocationPolicyDecision,
    /// All execution attempts made for this invocation.
    pub attempts: Vec<InvocationAttempt>,
    /// Approvals recorded against this invocation request.
    pub approvals: Vec<ApprovalRecord>,
}

/// Returns the directory path for a given invocation under a run directory.
pub fn invocation_dir(run_dir: &Path, request_id: &str) -> PathBuf {
    run_dir.join("invocations").join(request_id)
}

/// Returns the path to the request file for a given invocation.
pub fn request_path(run_dir: &Path, request_id: &str) -> PathBuf {
    invocation_dir(run_dir, request_id).join("request.toml")
}

/// Returns the path to the decision file for a given invocation.
pub fn decision_path(run_dir: &Path, request_id: &str) -> PathBuf {
    invocation_dir(run_dir, request_id).join("decision.toml")
}

/// Returns the path to an attempt file for a given invocation and attempt number.
pub fn attempt_path(run_dir: &Path, request_id: &str, attempt_number: u32) -> PathBuf {
    invocation_dir(run_dir, request_id).join(format!("attempt-{attempt_number:02}.toml"))
}

/// Returns the directory path for payload files for a given invocation.
pub fn payload_dir(run_dir: &Path, request_id: &str) -> PathBuf {
    invocation_dir(run_dir, request_id).join("payload")
}

/// Returns the path to a specific payload file for a given invocation.
pub fn payload_path(run_dir: &Path, request_id: &str, file_name: &str) -> PathBuf {
    payload_dir(run_dir, request_id).join(file_name)
}

/// Returns a payload reference descriptor for the given run, invocation, and filename.
pub fn payload_reference(run_id: &str, request_id: &str, file_name: &str) -> PayloadReference {
    PayloadReference {
        path: format!("runs/{run_id}/invocations/{request_id}/payload/{file_name}"),
        digest: None,
    }
}

/// Lists all invocation request IDs under the given run directory.
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::list_invocation_ids;

    #[test]
    fn list_invocation_ids_returns_empty_when_invocations_dir_absent() {
        // Point at a directory that exists but has no `invocations/` child.
        let result = list_invocation_ids(Path::new(env!("CARGO_MANIFEST_DIR")));
        assert_eq!(result.expect("should succeed without error"), Vec::<String>::new());
    }
}
