use std::collections::HashSet;
use std::fs;
use std::io::ErrorKind;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

use canon_engine::artifacts::markdown::MISSING_AUTHORED_BODY_MARKER;
use canon_engine::domain::artifact::{RUNTIME_PACKET_METADATA_FILE_NAME, RuntimePacketMetadata};
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::publish_profile::{
    AdaptiveGovernanceV1Envelope, AdaptiveGovernanceV1RuntimeInputs, AuthorityApprovalState,
    AuthorityGovernanceV1Envelope, AuthorityGovernanceV1RuntimeInputs, AuthorityPacketReadiness,
    SEMANTIC_ARTIFACT_CONTRACT_LINE_V1, SemanticArtifactDescriptor, SemanticEligibilityState,
    SemanticProvenanceBoundary,
};
use canon_engine::domain::run::{
    ClassificationFieldProvenance, ClassificationProvenance, ClassificationSource, RunState,
    SystemContext,
};
use canon_engine::persistence::store::WorkspaceStore;
use canon_engine::{EngineError, EngineService, RunRequest};
use serde::{Deserialize, Serialize};

use crate::app::{GovernanceCommand, OutputFormat};
use crate::error::{CliError, CliResult};
use crate::output;

mod error;
mod handlers;
mod parsers;
mod paths;
mod projection;
mod protocol;
mod status;

use paths::non_empty;
use protocol::{command_response, read_request};

const ADAPTER_SCHEMA_VERSION: &str = "v1";
const OPERATIONS: [&str; 3] = ["start", "refresh", "capabilities"];
const STATUS_VALUES: [GovernanceStatus; 7] = [
    GovernanceStatus::PendingSelection,
    GovernanceStatus::Running,
    GovernanceStatus::GovernedReady,
    GovernanceStatus::AwaitingApproval,
    GovernanceStatus::Blocked,
    GovernanceStatus::Completed,
    GovernanceStatus::Failed,
];
const APPROVAL_STATE_VALUES: [ApprovalState; 5] = [
    ApprovalState::NotNeeded,
    ApprovalState::Requested,
    ApprovalState::Granted,
    ApprovalState::Rejected,
    ApprovalState::Expired,
];
const PACKET_READINESS_VALUES: [PacketReadiness; 4] = [
    PacketReadiness::Pending,
    PacketReadiness::Incomplete,
    PacketReadiness::Reusable,
    PacketReadiness::Rejected,
];
type GovernanceFailure = Box<GovernanceResponse>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum GovernanceStatus {
    PendingSelection,
    Running,
    GovernedReady,
    AwaitingApproval,
    Blocked,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ApprovalState {
    NotNeeded,
    Requested,
    Granted,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum PacketReadiness {
    Pending,
    Incomplete,
    Reusable,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum GovernanceReasonCode {
    ApprovalRequired,
    ArtifactContractMissing,
    ArtifactContractUnreadable,
    BlockedByGovernance,
    DomainValidationFailed,
    IncompletePacket,
    InputDocumentMissing,
    MissingRequiredField,
    PathOutsideWorkspace,
    RejectedPacket,
    RequestKindMismatch,
    RunFailed,
    RunNotFound,
    RuntimeError,
    UnsupportedMode,
    UnsupportedRisk,
    UnsupportedSystemContext,
    UnsupportedZone,
    WorkspaceMismatch,
    WorkspaceUnavailable,
}

pub fn execute(
    service: &EngineService,
    repo_root: &Path,
    command: GovernanceCommand,
) -> CliResult<i32> {
    let request = if matches!(
        &command,
        GovernanceCommand::Start { .. } | GovernanceCommand::Refresh { .. }
    ) {
        Some(read_request()?)
    } else {
        None
    };

    let response = command_response(service, repo_root, command, request)?;
    output::print_value(&response, OutputFormat::Json)?;
    Ok(0)
}

#[derive(Debug, Deserialize, Default, Clone)]
struct GovernanceRequest {
    #[serde(default)]
    adapter_schema_version: Option<String>,
    #[serde(default)]
    request_kind: Option<String>,
    #[serde(default)]
    governance_attempt_id: Option<String>,
    #[serde(default)]
    stage_key: Option<String>,
    #[serde(default)]
    goal: Option<String>,
    #[serde(default)]
    workspace_ref: Option<String>,
    #[serde(default)]
    mode: Option<String>,
    #[serde(default)]
    system_context: Option<String>,
    #[serde(default)]
    risk: Option<String>,
    #[serde(default)]
    zone: Option<String>,
    #[serde(default)]
    owner: Option<String>,
    #[serde(default)]
    run_ref: Option<String>,
    #[serde(default)]
    bounded_context: GovernanceBoundedContext,
    #[serde(default)]
    input_documents: Vec<GovernanceInputDocument>,
}

#[derive(Debug, Deserialize, Default, Clone)]
struct GovernanceBoundedContext {
    #[serde(default)]
    stage_brief_ref: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
struct GovernanceInputDocument {
    #[serde(default, alias = "ref")]
    path: Option<String>,
}

impl GovernanceInputDocument {
    fn reference(&self) -> Option<&str> {
        non_empty(self.path.as_deref())
    }
}

#[derive(Debug, Serialize)]
struct GovernanceCapabilitiesResponse {
    canon_version: &'static str,
    supported_schema_versions: [&'static str; 1],
    operations: [&'static str; 3],
    supported_modes: Vec<&'static str>,
    status_values: [GovernanceStatus; 7],
    approval_state_values: [ApprovalState; 5],
    packet_readiness_values: [PacketReadiness; 4],
    compatibility_notes: [&'static str; 2],
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
struct GovernanceResponse {
    adapter_schema_version: &'static str,
    status: GovernanceStatus,
    approval_state: ApprovalState,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    packet_ref: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    expected_document_refs: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    document_refs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    packet_readiness: Option<PacketReadiness>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    missing_fields: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    missing_sections: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason_code: Option<GovernanceReasonCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authority_governance: Option<AuthorityGovernanceV1Envelope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    adaptive_governance: Option<AdaptiveGovernanceV1Envelope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    semantic_descriptor: Option<SemanticArtifactDescriptor>,
}

impl GovernanceResponse {
    fn blocked(
        reason_code: GovernanceReasonCode,
        message: impl Into<String>,
        missing_fields: Vec<String>,
    ) -> Self {
        Self {
            adapter_schema_version: ADAPTER_SCHEMA_VERSION,
            status: GovernanceStatus::Blocked,
            approval_state: ApprovalState::NotNeeded,
            message: message.into(),
            run_ref: None,
            packet_ref: None,
            expected_document_refs: Vec::new(),
            document_refs: Vec::new(),
            packet_readiness: None,
            missing_fields,
            missing_sections: Vec::new(),
            headline: Some("Governance request is blocked".to_string()),
            reason_code: Some(reason_code),
            authority_governance: None,
            adaptive_governance: None,
            semantic_descriptor: None,
        }
    }

    fn failed(
        reason_code: GovernanceReasonCode,
        message: impl Into<String>,
        run_ref: Option<String>,
    ) -> Self {
        Self {
            adapter_schema_version: ADAPTER_SCHEMA_VERSION,
            status: GovernanceStatus::Failed,
            approval_state: ApprovalState::NotNeeded,
            message: message.into(),
            run_ref,
            packet_ref: None,
            expected_document_refs: Vec::new(),
            document_refs: Vec::new(),
            packet_readiness: None,
            missing_fields: Vec::new(),
            missing_sections: Vec::new(),
            headline: Some("Governance request failed".to_string()),
            reason_code: Some(reason_code),
            authority_governance: None,
            adaptive_governance: None,
            semantic_descriptor: None,
        }
    }
}

#[derive(Debug)]
enum GovernanceOperation {
    Start,
    Refresh,
}

#[derive(Debug)]
struct ValidatedStartRequest {
    mode: Mode,
    system_context: SystemContext,
    risk: RiskClass,
    zone: UsageZone,
    owner: String,
    inputs: Vec<String>,
}

#[derive(Debug)]
struct RunProjection {
    run_ref: String,
    run_state: RunState,
    approval_state: ApprovalState,
    packet_ref: Option<String>,
    expected_document_refs: Vec<String>,
    document_refs: Vec<String>,
    packet_readiness: Option<PacketReadiness>,
    missing_sections: Vec<String>,
    authority_governance: Option<AuthorityGovernanceV1Envelope>,
    adaptive_governance: Option<AdaptiveGovernanceV1Envelope>,
    semantic_descriptor: Option<SemanticArtifactDescriptor>,
}

#[cfg(test)]
mod tests;
