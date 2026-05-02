use std::collections::HashSet;
use std::io::ErrorKind;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

use canon_engine::artifacts::markdown::MISSING_AUTHORED_BODY_MARKER;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
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

const ADAPTER_SCHEMA_VERSION: &str = "v1";
const OPERATIONS: [&str; 3] = ["start", "refresh", "capabilities"];
const STATUS_VALUES: [&str; 7] = [
    "pending_selection",
    "running",
    "governed_ready",
    "awaiting_approval",
    "blocked",
    "completed",
    "failed",
];
const APPROVAL_STATE_VALUES: [&str; 5] =
    ["not_needed", "requested", "granted", "rejected", "expired"];
const PACKET_READINESS_VALUES: [&str; 4] = ["pending", "incomplete", "reusable", "rejected"];
type GovernanceFailure = Box<GovernanceResponse>;

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
    status_values: [&'static str; 7],
    approval_state_values: [&'static str; 5],
    packet_readiness_values: [&'static str; 4],
    compatibility_notes: [&'static str; 2],
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
struct GovernanceResponse {
    adapter_schema_version: &'static str,
    status: String,
    approval_state: String,
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
    packet_readiness: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    missing_fields: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    missing_sections: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason_code: Option<String>,
}

impl GovernanceResponse {
    fn blocked(reason_code: &str, message: impl Into<String>, missing_fields: Vec<String>) -> Self {
        Self {
            adapter_schema_version: ADAPTER_SCHEMA_VERSION,
            status: "blocked".to_string(),
            approval_state: "not_needed".to_string(),
            message: message.into(),
            run_ref: None,
            packet_ref: None,
            expected_document_refs: Vec::new(),
            document_refs: Vec::new(),
            packet_readiness: None,
            missing_fields,
            missing_sections: Vec::new(),
            headline: Some("Governance request is blocked".to_string()),
            reason_code: Some(reason_code.to_string()),
        }
    }

    fn failed(reason_code: &str, message: impl Into<String>, run_ref: Option<String>) -> Self {
        Self {
            adapter_schema_version: ADAPTER_SCHEMA_VERSION,
            status: "failed".to_string(),
            approval_state: "not_needed".to_string(),
            message: message.into(),
            run_ref,
            packet_ref: None,
            expected_document_refs: Vec::new(),
            document_refs: Vec::new(),
            packet_readiness: None,
            missing_fields: Vec::new(),
            missing_sections: Vec::new(),
            headline: Some("Governance request failed".to_string()),
            reason_code: Some(reason_code.to_string()),
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
    approval_state: String,
    packet_ref: Option<String>,
    expected_document_refs: Vec<String>,
    document_refs: Vec<String>,
    packet_readiness: Option<String>,
    missing_sections: Vec<String>,
}

fn command_response(
    service: &EngineService,
    repo_root: &Path,
    command: GovernanceCommand,
    request: Option<GovernanceRequest>,
) -> CliResult<serde_json::Value> {
    match command {
        GovernanceCommand::Capabilities { json } => {
            require_json(json)?;
            Ok(serde_json::to_value(capabilities_response())?)
        }
        GovernanceCommand::Start { json } => {
            require_json(json)?;
            let request = request.ok_or_else(|| {
                CliError::InvalidInput("governance start requires a JSON request body".to_string())
            })?;
            enforce_supported_schema_version(request.adapter_schema_version.as_deref())?;
            Ok(serde_json::to_value(handle_start(service, repo_root, request))?)
        }
        GovernanceCommand::Refresh { json } => {
            require_json(json)?;
            let request = request.ok_or_else(|| {
                CliError::InvalidInput(
                    "governance refresh requires a JSON request body".to_string(),
                )
            })?;
            enforce_supported_schema_version(request.adapter_schema_version.as_deref())?;
            Ok(serde_json::to_value(handle_refresh(repo_root, request))?)
        }
    }
}

fn capabilities_response() -> GovernanceCapabilitiesResponse {
    GovernanceCapabilitiesResponse {
        canon_version: env!("CARGO_PKG_VERSION"),
        supported_schema_versions: [ADAPTER_SCHEMA_VERSION],
        operations: OPERATIONS,
        supported_modes: Mode::all().iter().map(|mode| mode.as_str()).collect(),
        status_values: STATUS_VALUES,
        approval_state_values: APPROVAL_STATE_VALUES,
        packet_readiness_values: PACKET_READINESS_VALUES,
        compatibility_notes: [
            "Requests that omit adapter_schema_version are interpreted as v1.",
            "Unknown additive request fields are ignored within supported schema versions.",
        ],
    }
}

fn require_json(json: bool) -> CliResult<()> {
    if json {
        Ok(())
    } else {
        Err(CliError::InvalidInput(
            "governance commands require --json for the machine-facing contract".to_string(),
        ))
    }
}

fn read_request() -> CliResult<GovernanceRequest> {
    read_request_from(std::io::stdin())
}

fn read_request_from(mut reader: impl Read) -> CliResult<GovernanceRequest> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    Ok(serde_json::from_str(&buffer)?)
}

fn enforce_supported_schema_version(version: Option<&str>) -> CliResult<()> {
    match version {
        None | Some(ADAPTER_SCHEMA_VERSION) => Ok(()),
        Some(other) => {
            Err(CliError::InvalidInput(format!("unsupported adapter schema version: {other}")))
        }
    }
}

fn handle_start(
    service: &EngineService,
    repo_root: &Path,
    request: GovernanceRequest,
) -> GovernanceResponse {
    let validated = match validate_request(repo_root, &request, GovernanceOperation::Start) {
        Ok(validated) => validated,
        Err(response) => return *response,
    };

    let run_request = RunRequest {
        mode: validated.mode,
        risk: validated.risk,
        zone: validated.zone,
        system_context: Some(validated.system_context),
        classification: explicit_classification(),
        owner: validated.owner,
        inputs: validated.inputs,
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    };

    match service.run(run_request) {
        Ok(summary) => project_run_response(
            repo_root,
            &summary.run_id,
            summary.mode_result.map(|result| result.headline),
        ),
        Err(error) => map_engine_error(error, None),
    }
}

fn handle_refresh(repo_root: &Path, request: GovernanceRequest) -> GovernanceResponse {
    let run_ref = request.run_ref.clone();
    if let Err(response) = validate_request(repo_root, &request, GovernanceOperation::Refresh) {
        return *response;
    }

    project_run_response(
        repo_root,
        non_empty(run_ref.as_deref()).expect("validated refresh requests always include run_ref"),
        None,
    )
}

fn validate_request(
    repo_root: &Path,
    request: &GovernanceRequest,
    operation: GovernanceOperation,
) -> Result<ValidatedStartRequest, GovernanceFailure> {
    let missing = missing_domain_fields(request, &operation);
    if !missing.is_empty() {
        return Err(Box::new(GovernanceResponse::blocked(
            "missing_required_field",
            "request is missing required fields for domain execution",
            missing,
        )));
    }

    let expected_kind = match operation {
        GovernanceOperation::Start => "start",
        GovernanceOperation::Refresh => "refresh",
    };
    let request_kind = non_empty(request.request_kind.as_deref()).unwrap_or_default();
    if request_kind != expected_kind {
        return Err(Box::new(GovernanceResponse::blocked(
            "request_kind_mismatch",
            format!(
                "request_kind `{request_kind}` does not match the invoked `{expected_kind}` operation"
            ),
            vec!["request_kind".to_string()],
        )));
    }

    validate_workspace_binding(repo_root, request.workspace_ref.as_deref().unwrap_or_default())?;

    let mode = parse_mode(request.mode.as_deref().unwrap_or_default())?;
    let system_context =
        parse_system_context(request.system_context.as_deref().unwrap_or_default())?;
    let risk = parse_risk(request.risk.as_deref().unwrap_or_default())?;
    let zone = parse_zone(request.zone.as_deref().unwrap_or_default())?;
    let owner = request.owner.as_deref().unwrap_or_default().trim().to_string();
    let inputs = collect_input_references(repo_root, request)?;

    Ok(ValidatedStartRequest { mode, system_context, risk, zone, owner, inputs })
}

fn missing_domain_fields(
    request: &GovernanceRequest,
    operation: &GovernanceOperation,
) -> Vec<String> {
    let mut missing = Vec::new();
    let required = [
        ("request_kind", request.request_kind.as_deref()),
        ("governance_attempt_id", request.governance_attempt_id.as_deref()),
        ("stage_key", request.stage_key.as_deref()),
        ("goal", request.goal.as_deref()),
        ("workspace_ref", request.workspace_ref.as_deref()),
        ("mode", request.mode.as_deref()),
        ("system_context", request.system_context.as_deref()),
        ("risk", request.risk.as_deref()),
        ("zone", request.zone.as_deref()),
        ("owner", request.owner.as_deref()),
    ];

    for (field, value) in required {
        if non_empty(value).is_none() {
            missing.push(field.to_string());
        }
    }

    if matches!(operation, GovernanceOperation::Refresh)
        && non_empty(request.run_ref.as_deref()).is_none()
    {
        missing.push("run_ref".to_string());
    }

    missing
}

fn validate_workspace_binding(
    repo_root: &Path,
    workspace_ref: &str,
) -> Result<(), GovernanceFailure> {
    let canonical_repo = canonical_repo_root(repo_root).map_err(|error| {
        Box::new(GovernanceResponse::failed(
            "workspace_unavailable",
            format!("workspace is not accessible: {error}"),
            None,
        ))
    })?;
    let requested = resolve_request_path(repo_root, workspace_ref);
    let canonical_requested = requested.canonicalize().map_err(|_| {
        Box::new(GovernanceResponse::failed(
            "workspace_unavailable",
            format!("workspace `{workspace_ref}` is not accessible"),
            None,
        ))
    })?;

    if canonical_requested != canonical_repo {
        return Err(Box::new(GovernanceResponse::blocked(
            "workspace_mismatch",
            "workspace_ref must bind to the current Canon workspace",
            vec!["workspace_ref".to_string()],
        )));
    }

    Ok(())
}

fn parse_mode(value: &str) -> Result<Mode, GovernanceFailure> {
    value.parse::<Mode>().map_err(|_| {
        Box::new(GovernanceResponse::blocked(
            "unsupported_mode",
            format!("mode `{value}` is not supported by Canon governance"),
            vec!["mode".to_string()],
        ))
    })
}

fn parse_system_context(value: &str) -> Result<SystemContext, GovernanceFailure> {
    value.parse::<SystemContext>().map_err(|_| {
        Box::new(GovernanceResponse::blocked(
            "unsupported_system_context",
            format!("system_context `{value}` is not supported by Canon governance"),
            vec!["system_context".to_string()],
        ))
    })
}

fn parse_risk(value: &str) -> Result<RiskClass, GovernanceFailure> {
    value.parse::<RiskClass>().map_err(|_| {
        Box::new(GovernanceResponse::blocked(
            "unsupported_risk",
            format!("risk `{value}` is not supported by Canon governance"),
            vec!["risk".to_string()],
        ))
    })
}

fn parse_zone(value: &str) -> Result<UsageZone, GovernanceFailure> {
    value.parse::<UsageZone>().map_err(|_| {
        Box::new(GovernanceResponse::blocked(
            "unsupported_zone",
            format!("zone `{value}` is not supported by Canon governance"),
            vec!["zone".to_string()],
        ))
    })
}

fn collect_input_references(
    repo_root: &Path,
    request: &GovernanceRequest,
) -> Result<Vec<String>, GovernanceFailure> {
    let mut inputs = Vec::new();

    if let Some(stage_brief_ref) = non_empty(request.bounded_context.stage_brief_ref.as_deref()) {
        push_unique_input(repo_root, stage_brief_ref, &mut inputs)?;
    }

    for document in &request.input_documents {
        if let Some(reference) = document.reference() {
            push_unique_input(repo_root, reference, &mut inputs)?;
        }
    }

    Ok(inputs)
}

fn push_unique_input(
    repo_root: &Path,
    reference: &str,
    inputs: &mut Vec<String>,
) -> Result<(), GovernanceFailure> {
    let normalized = normalize_workspace_relative_ref(repo_root, reference)?;
    if !inputs.contains(&normalized) {
        inputs.push(normalized);
    }
    Ok(())
}

fn normalize_workspace_relative_ref(
    repo_root: &Path,
    reference: &str,
) -> Result<String, GovernanceFailure> {
    let canonical_repo = canonical_repo_root(repo_root).map_err(|error| {
        Box::new(GovernanceResponse::failed(
            "workspace_unavailable",
            format!("workspace is not accessible: {error}"),
            None,
        ))
    })?;
    let candidate = resolve_request_path(repo_root, reference);
    let canonical_candidate = candidate.canonicalize().map_err(|_| {
        Box::new(GovernanceResponse::blocked(
            "input_document_missing",
            format!("document `{reference}` is not accessible inside the workspace"),
            vec!["input_documents".to_string()],
        ))
    })?;

    if !canonical_candidate.starts_with(&canonical_repo) {
        return Err(Box::new(GovernanceResponse::blocked(
            "path_outside_workspace",
            format!("document `{reference}` escapes the declared workspace boundary"),
            vec!["input_documents".to_string()],
        )));
    }

    let relative = canonical_candidate.strip_prefix(&canonical_repo).map_err(|_| {
        Box::new(GovernanceResponse::blocked(
            "path_outside_workspace",
            format!("document `{reference}` escapes the declared workspace boundary"),
            vec!["input_documents".to_string()],
        ))
    })?;

    Ok(path_to_slash_string(relative))
}

fn project_run_response(
    repo_root: &Path,
    run_ref: &str,
    headline_hint: Option<String>,
) -> GovernanceResponse {
    let projection = match load_run_projection(repo_root, run_ref) {
        Ok(projection) => projection,
        Err(response) => return *response,
    };

    let status = normalized_status(projection.run_state, projection.packet_readiness.as_deref());
    let reason_code = response_reason_code(status, projection.packet_readiness.as_deref());
    let headline =
        headline_hint.or_else(|| default_headline(status, projection.packet_readiness.as_deref()));
    let message =
        default_message(status, &projection.run_ref, projection.packet_readiness.as_deref());

    GovernanceResponse {
        adapter_schema_version: ADAPTER_SCHEMA_VERSION,
        status: status.to_string(),
        approval_state: projection.approval_state,
        message,
        run_ref: Some(projection.run_ref),
        packet_ref: projection.packet_ref,
        expected_document_refs: projection.expected_document_refs,
        document_refs: projection.document_refs,
        packet_readiness: projection.packet_readiness,
        missing_fields: Vec::new(),
        missing_sections: projection.missing_sections,
        headline,
        reason_code,
    }
}

fn load_run_projection(
    repo_root: &Path,
    run_ref: &str,
) -> Result<RunProjection, GovernanceFailure> {
    let store = WorkspaceStore::new(repo_root);
    let manifest = store.load_run_manifest(run_ref).map_err(|_| {
        Box::new(GovernanceResponse::failed(
            "run_not_found",
            format!("run `{run_ref}` was not found in this workspace"),
            Some(run_ref.to_string()),
        ))
    })?;
    let state = store.load_run_state(run_ref).map_err(|error| {
        Box::new(GovernanceResponse::failed(
            "runtime_error",
            format!("run `{run_ref}` state could not be loaded: {error}"),
            Some(run_ref.to_string()),
        ))
    })?;
    let approvals = store.load_approval_records(run_ref).map_err(|error| {
        Box::new(GovernanceResponse::failed(
            "runtime_error",
            format!("run `{run_ref}` approvals could not be loaded: {error}"),
            Some(run_ref.to_string()),
        ))
    })?;

    let (expected_document_refs, artifact_contract_missing) =
        match store.load_artifact_contract(run_ref) {
            Ok(contract) => (
                contract
                    .artifact_requirements
                    .into_iter()
                    .map(|requirement| {
                        format!(
                            ".canon/artifacts/{run_ref}/{}/{}",
                            manifest.mode.as_str(),
                            requirement.file_name
                        )
                    })
                    .collect::<Vec<_>>(),
                false,
            ),
            Err(error) if error.kind() == ErrorKind::NotFound => (Vec::new(), true),
            Err(error) => {
                return Err(Box::new(GovernanceResponse::failed(
                    "artifact_contract_unreadable",
                    format!("run `{run_ref}` artifact contract could not be loaded: {error}"),
                    Some(run_ref.to_string()),
                )));
            }
        };

    let document_refs = load_document_refs(&store, run_ref, manifest.mode, &expected_document_refs)
        .map_err(|error| {
            Box::new(GovernanceResponse::failed(
                "runtime_error",
                format!("run `{run_ref}` artifacts could not be listed: {error}"),
                Some(run_ref.to_string()),
            ))
        })?;

    if artifact_contract_missing && !document_refs.is_empty() {
        return Err(Box::new(GovernanceResponse::failed(
            "artifact_contract_missing",
            format!("run `{run_ref}` artifacts are present but artifact contract is missing"),
            Some(run_ref.to_string()),
        )));
    }

    let missing_refs = missing_document_refs(&expected_document_refs, &document_refs);
    let rejected_refs = rejected_document_refs(repo_root, &document_refs);
    let packet_readiness = packet_readiness_value(
        &expected_document_refs,
        &document_refs,
        &missing_refs,
        &rejected_refs,
    );
    let missing_sections = packet_missing_sections(&missing_refs, &rejected_refs);
    let approval_state = approval_state_value(
        state.state,
        approvals.iter().any(|record| record.is_approved()),
        approvals.iter().any(|record| !record.is_approved()),
    );
    let packet_ref = if expected_document_refs.is_empty() && document_refs.is_empty() {
        None
    } else {
        Some(format!(".canon/artifacts/{run_ref}/{}", manifest.mode.as_str()))
    };

    Ok(RunProjection {
        run_ref: run_ref.to_string(),
        run_state: state.state,
        approval_state,
        packet_ref,
        expected_document_refs,
        document_refs,
        packet_readiness,
        missing_sections,
    })
}

fn load_document_refs(
    store: &WorkspaceStore,
    run_ref: &str,
    mode: Mode,
    expected_document_refs: &[String],
) -> Result<Vec<String>, std::io::Error> {
    let mode_segment = format!("/{}/", mode.as_str());
    let available = store
        .list_artifact_files(run_ref)?
        .into_iter()
        .filter(|path| path.contains(&mode_segment))
        .collect::<HashSet<_>>();

    if expected_document_refs.is_empty() {
        let mut refs = available.into_iter().collect::<Vec<_>>();
        refs.sort();
        return Ok(refs);
    }

    Ok(expected_document_refs
        .iter()
        .filter(|path| available.contains(path.as_str()))
        .cloned()
        .collect())
}

fn missing_document_refs(
    expected_document_refs: &[String],
    document_refs: &[String],
) -> Vec<String> {
    let present = document_refs.iter().map(String::as_str).collect::<HashSet<_>>();
    expected_document_refs
        .iter()
        .filter(|reference| !present.contains(reference.as_str()))
        .cloned()
        .collect()
}

fn rejected_document_refs(repo_root: &Path, document_refs: &[String]) -> Vec<String> {
    document_refs
        .iter()
        .filter(|reference| artifact_contains_missing_authored_body(repo_root, reference))
        .cloned()
        .collect()
}

fn packet_readiness_value(
    expected_document_refs: &[String],
    document_refs: &[String],
    missing_refs: &[String],
    rejected_refs: &[String],
) -> Option<String> {
    if expected_document_refs.is_empty() && document_refs.is_empty() {
        return Some("pending".to_string());
    }
    if !rejected_refs.is_empty() {
        return Some("rejected".to_string());
    }
    if !missing_refs.is_empty() {
        return Some("incomplete".to_string());
    }
    if !document_refs.is_empty() {
        return Some("reusable".to_string());
    }

    Some("pending".to_string())
}

fn packet_missing_sections(missing_refs: &[String], rejected_refs: &[String]) -> Vec<String> {
    let mut sections =
        missing_refs.iter().map(|reference| packet_leaf(reference)).collect::<Vec<_>>();

    for rejected in rejected_refs {
        let leaf = packet_leaf(rejected);
        if !sections.contains(&leaf) {
            sections.push(leaf);
        }
    }

    sections
}

fn approval_state_value(run_state: RunState, any_approved: bool, any_rejected: bool) -> String {
    if matches!(run_state, RunState::AwaitingApproval) {
        return "requested".to_string();
    }
    if any_rejected {
        return "rejected".to_string();
    }
    if any_approved {
        return "granted".to_string();
    }
    "not_needed".to_string()
}

fn normalized_status(run_state: RunState, packet_readiness: Option<&str>) -> &'static str {
    match run_state {
        RunState::Draft
        | RunState::ContextCaptured
        | RunState::Classified
        | RunState::Contracted
        | RunState::Gated => "pending_selection",
        RunState::Executing | RunState::Verifying => "running",
        RunState::AwaitingApproval => "awaiting_approval",
        RunState::Blocked => "blocked",
        RunState::Completed => match packet_readiness {
            Some("reusable") => "governed_ready",
            Some("incomplete") | Some("rejected") => "blocked",
            _ => "completed",
        },
        RunState::Failed | RunState::Aborted | RunState::Superseded => "failed",
    }
}

fn response_reason_code(status: &str, packet_readiness: Option<&str>) -> Option<String> {
    match status {
        "awaiting_approval" => Some("approval_required".to_string()),
        "blocked" => match packet_readiness {
            Some("incomplete") => Some("incomplete_packet".to_string()),
            Some("rejected") => Some("rejected_packet".to_string()),
            _ => Some("blocked_by_governance".to_string()),
        },
        "failed" => Some("run_failed".to_string()),
        _ => None,
    }
}

fn default_headline(status: &str, packet_readiness: Option<&str>) -> Option<String> {
    let headline = match status {
        "governed_ready" => "Governed packet is reusable",
        "awaiting_approval" => "Governed packet is waiting on approval",
        "blocked" => match packet_readiness {
            Some("incomplete") => "Governed packet is incomplete",
            Some("rejected") => "Governed packet was rejected for downstream reuse",
            _ => "Governance execution is blocked",
        },
        "completed" => "Governance execution completed",
        "running" => "Governance execution is still running",
        "pending_selection" => "Governance execution is still selecting the next action",
        "failed" => "Governance execution failed",
        _ => return None,
    };

    Some(headline.to_string())
}

fn default_message(status: &str, run_ref: &str, packet_readiness: Option<&str>) -> String {
    match status {
        "governed_ready" => {
            format!("run `{run_ref}` produced a reusable governed packet")
        }
        "awaiting_approval" => {
            format!("run `{run_ref}` is awaiting approval before downstream reuse")
        }
        "blocked" => match packet_readiness {
            Some("incomplete") => {
                format!("run `{run_ref}` is blocked because the governed packet is incomplete")
            }
            Some("rejected") => {
                format!("run `{run_ref}` is blocked because the governed packet is not reusable")
            }
            _ => format!("run `{run_ref}` is blocked by Canon governance"),
        },
        "completed" => format!("run `{run_ref}` completed without a reusable packet projection"),
        "running" => format!("run `{run_ref}` is still running"),
        "pending_selection" => format!("run `{run_ref}` is still selecting the next governed step"),
        "failed" => format!("run `{run_ref}` failed"),
        _ => format!("run `{run_ref}` returned an unknown governance status"),
    }
}

fn explicit_classification() -> ClassificationProvenance {
    ClassificationProvenance {
        risk: ClassificationFieldProvenance::new(
            ClassificationSource::Explicit,
            "Risk class was supplied by the governance adapter request.".to_string(),
            Vec::new(),
        ),
        zone: ClassificationFieldProvenance::new(
            ClassificationSource::Explicit,
            "Usage zone was supplied by the governance adapter request.".to_string(),
            Vec::new(),
        ),
    }
}

fn map_engine_error(error: EngineError, run_ref: Option<String>) -> GovernanceResponse {
    match error {
        EngineError::Validation(message) => {
            GovernanceResponse::blocked("domain_validation_failed", message, Vec::new())
                .with_run_ref(run_ref)
        }
        EngineError::UnsupportedMode(mode) => GovernanceResponse::blocked(
            "unsupported_mode",
            format!("mode `{mode}` is not supported by Canon governance"),
            vec!["mode".to_string()],
        )
        .with_run_ref(run_ref),
        EngineError::Io(error) => GovernanceResponse::failed(
            "workspace_unavailable",
            format!("workspace or runtime state is not accessible: {error}"),
            run_ref,
        ),
        EngineError::UnsupportedInspectTarget(target) => GovernanceResponse::failed(
            "runtime_error",
            format!("unexpected engine target surfaced from governance execution: {target}"),
            run_ref,
        ),
    }
}

impl GovernanceResponse {
    fn with_run_ref(mut self, run_ref: Option<String>) -> Self {
        self.run_ref = run_ref;
        self
    }
}

fn resolve_request_path(repo_root: &Path, value: &str) -> PathBuf {
    let path = Path::new(value);
    if path.is_absolute() { path.to_path_buf() } else { repo_root.join(path) }
}

fn canonical_repo_root(repo_root: &Path) -> Result<PathBuf, std::io::Error> {
    repo_root.canonicalize()
}

fn path_to_slash_string(path: &Path) -> String {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(segment) => Some(segment.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn artifact_contains_missing_authored_body(repo_root: &Path, document_ref: &str) -> bool {
    std::fs::read_to_string(repo_root.join(document_ref))
        .map(|contents| contents.contains(MISSING_AUTHORED_BODY_MARKER))
        .unwrap_or(false)
}

fn packet_leaf(reference: &str) -> String {
    Path::new(reference)
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| reference.to_string())
}

fn non_empty(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use canon_engine::persistence::layout::ProjectLayout;
    use canon_engine::{EngineError, EngineService};
    use serde_json::json;
    use tempfile::TempDir;

    use super::{
        GovernanceBoundedContext, GovernanceInputDocument, GovernanceOperation, GovernanceRequest,
        GovernanceResponse, approval_state_value, artifact_contains_missing_authored_body,
        collect_input_references, command_response, default_headline, default_message,
        map_engine_error, missing_domain_fields, normalize_workspace_relative_ref,
        normalized_status, packet_missing_sections, packet_readiness_value, path_to_slash_string,
        read_request_from, response_reason_code,
    };
    use canon_engine::domain::run::RunState;

    fn complete_requirements_brief() -> &'static str {
        r#"# Requirements Brief

## Problem

Bound AI-assisted engineering work with explicit governance.

## Outcome

Operators can review a complete requirements packet before downstream planning.

## Constraints

- Keep execution local-first
- Preserve explicit approval checkpoints

## Non-Negotiables

- Persist evidence under `.canon/`
- Keep named ownership explicit

## Options

1. Deliver the bounded packet first.
2. Defer broader rollout.

## Recommended Path

Deliver the bounded packet first.

## Tradeoffs

- Structure before speed

## Consequences

- Reviewers can inspect durable artifacts.

## Out of Scope

- No hosted control plane in this slice

## Deferred Work

- Hosted coordination remains later work.

## Decision Checklist

- [x] Scope is explicit
- [x] Ownership is explicit

## Open Questions

- Which downstream mode should consume the packet first?
"#
    }

    fn incomplete_requirements_brief() -> &'static str {
        "# Requirements Brief\n\n## Problem\n\nBound the firmware-flashing workflow to a USB-only CLI surface.\n"
    }

    fn complete_architecture_brief() -> &'static str {
        r#"# Architecture Brief

Decision focus: map boundaries and tradeoffs for governed analysis-mode expansion.
Constraint: preserve Canon persistence, evidence, and approval behavior.

## Decision
Use a dedicated context map to make architecture boundaries reviewable.

## Options
- Keep domain boundaries implicit in existing prose.
- Add a dedicated `context-map.md` artifact.

## Constraints
- Preserve run identity and approval behavior.
- Keep non-target modes unchanged.

## Candidate Boundaries
- Runtime Governance
- Artifact Authoring

## Invariants
- Evidence remains linked to the run.
- Risk review stays explicit.

## Evaluation Criteria
- Ownership clarity
- Seam visibility.

## Decision Drivers
- Reviewers need the chosen direction and rationale without consulting chat history.
- The packet must remain critique-first when authored context is weak.

## Options Considered
- Keep the current generic decision summary.
- Preserve authored decision and option-analysis sections directly in the existing artifacts.

## Pros
- The emitted packet records the chosen option and rejected alternatives explicitly.
- Reviewers can reuse the packet outside the originating conversation.

## Cons
- The authored brief must carry more explicit decision content.

## Recommendation
Preserve authored decision and option-analysis sections directly in the existing architecture decision artifacts.

## Why Not The Others
- The generic summary shape hides rejected alternatives.
- A new artifact family would widen scope beyond this slice.

## Risks
- Context crossings may be hidden inside summary prose.

## Bounded Contexts
- Runtime Governance: owns approvals, run state, and evidence linkage.
- Artifact Authoring: owns packet structure and authored-body fidelity.

## Context Relationships
- Artifact Authoring consumes gate and lineage outcomes from Runtime Governance.

## Integration Seams
- `mode_shaping` hands rendered artifacts to gate evaluation and summarization.

## Anti-Corruption Candidates
- Renderer helpers should remain isolated from governance-specific state semantics.

## Ownership Boundaries
- Governance code owns gate evaluation.
- Rendering code owns authored markdown fidelity.

## Shared Invariants
- Every artifact remains bound to one run id.
- Approval-gated architecture runs cannot skip risk review.

## System Context
- System: `canon-engine` governs analysis packets and durable evidence.
- External actors:
  - architect-reviewer: reads architecture packets.
  - copilot-cli-adapter: generates and critiques packet content.

## Containers
- `canon-cli` (Rust CLI): entrypoint for run and inspect commands.
- `canon-engine` (Rust library): orchestrates generation, critique, gates, and rendering.
- `.canon/` (filesystem): persists run manifests, artifacts, and evidence.

## Components
- `mode_shaping`: runs architecture orchestration.
- `gatekeeper`: validates contract and policy gates.
- `markdown renderer`: emits reviewable architecture artifacts.
"#
    }

    fn governance_start_request(
        workspace: &TempDir,
        mode: &str,
        risk: &str,
        zone: &str,
        owner: &str,
        input_path: Option<&str>,
    ) -> GovernanceRequest {
        GovernanceRequest {
            request_kind: Some("start".to_string()),
            governance_attempt_id: Some("ga-start-001".to_string()),
            stage_key: Some("analysis".to_string()),
            goal: Some("Create a governed packet".to_string()),
            workspace_ref: Some(workspace.path().to_string_lossy().into_owned()),
            mode: Some(mode.to_string()),
            system_context: Some("existing".to_string()),
            risk: Some(risk.to_string()),
            zone: Some(zone.to_string()),
            owner: Some(owner.to_string()),
            input_documents: input_path
                .into_iter()
                .map(|path| GovernanceInputDocument { path: Some(path.to_string()) })
                .collect(),
            ..GovernanceRequest::default()
        }
    }

    fn governance_refresh_request(
        workspace: &TempDir,
        run_ref: &str,
        mode: &str,
        risk: &str,
        zone: &str,
        owner: &str,
    ) -> GovernanceRequest {
        GovernanceRequest {
            request_kind: Some("refresh".to_string()),
            governance_attempt_id: Some("ga-refresh-001".to_string()),
            stage_key: Some("verification".to_string()),
            goal: Some("Refresh the governed packet".to_string()),
            workspace_ref: Some(workspace.path().to_string_lossy().into_owned()),
            mode: Some(mode.to_string()),
            system_context: Some("existing".to_string()),
            risk: Some(risk.to_string()),
            zone: Some(zone.to_string()),
            owner: Some(owner.to_string()),
            run_ref: Some(run_ref.to_string()),
            ..GovernanceRequest::default()
        }
    }

    #[test]
    fn read_request_from_parses_json_and_ref_alias() {
        let request = read_request_from(
            r#"{
  "request_kind": "start",
  "workspace_ref": ".",
  "input_documents": [{ "ref": "brief.md" }]
}"#
            .as_bytes(),
        )
        .expect("request should parse");

        assert_eq!(request.request_kind.as_deref(), Some("start"));
        assert_eq!(request.input_documents[0].reference(), Some("brief.md"));
    }

    #[test]
    fn command_response_reports_capabilities_contract() {
        let service = EngineService::new(".");

        let response = command_response(
            &service,
            std::path::Path::new("."),
            crate::app::GovernanceCommand::Capabilities { json: true },
            None,
        )
        .expect("capabilities should succeed");

        assert_eq!(response["supported_schema_versions"], json!(["v1"]));
        assert_eq!(response["operations"], json!(["start", "refresh", "capabilities"]));
        assert_eq!(
            response["packet_readiness_values"],
            json!(["pending", "incomplete", "reusable", "rejected"])
        );
    }

    #[test]
    fn command_response_requires_json_flag() {
        let service = EngineService::new(".");

        let error = command_response(
            &service,
            std::path::Path::new("."),
            crate::app::GovernanceCommand::Capabilities { json: false },
            None,
        )
        .expect_err("missing --json should fail");

        assert!(error.to_string().contains("require --json"));
    }

    #[test]
    fn command_response_rejects_unsupported_schema_versions() {
        let workspace = TempDir::new().expect("temp dir");
        let service = EngineService::new(workspace.path());

        let error = command_response(
            &service,
            workspace.path(),
            crate::app::GovernanceCommand::Start { json: true },
            Some(GovernanceRequest {
                adapter_schema_version: Some("v2".to_string()),
                ..GovernanceRequest::default()
            }),
        )
        .expect_err("unsupported version should fail");

        assert!(error.to_string().contains("unsupported adapter schema version: v2"));
    }

    #[test]
    fn command_response_returns_governed_ready_for_complete_requirements_packets() {
        let workspace = TempDir::new().expect("temp dir");
        fs::write(workspace.path().join("requirements.md"), complete_requirements_brief())
            .expect("requirements brief");
        let service = EngineService::new(workspace.path());

        let response = command_response(
            &service,
            workspace.path(),
            crate::app::GovernanceCommand::Start { json: true },
            Some(governance_start_request(
                &workspace,
                "requirements",
                "bounded-impact",
                "yellow",
                "product-lead",
                Some("requirements.md"),
            )),
        )
        .expect("start should succeed");

        assert_eq!(response["status"], json!("governed_ready"));
        assert_eq!(response["approval_state"], json!("not_needed"));
        assert_eq!(response["packet_readiness"], json!("reusable"));
        assert!(response["packet_ref"].as_str().is_some_and(|value| !value.is_empty()));
        assert!(response["document_refs"].as_array().is_some_and(|refs| !refs.is_empty()));
    }

    #[test]
    fn command_response_refreshes_existing_governed_runs() {
        let workspace = TempDir::new().expect("temp dir");
        fs::write(workspace.path().join("requirements.md"), complete_requirements_brief())
            .expect("requirements brief");
        let service = EngineService::new(workspace.path());

        let start_response = command_response(
            &service,
            workspace.path(),
            crate::app::GovernanceCommand::Start { json: true },
            Some(governance_start_request(
                &workspace,
                "requirements",
                "bounded-impact",
                "yellow",
                "product-lead",
                Some("requirements.md"),
            )),
        )
        .expect("start should succeed");
        let run_ref = start_response["run_ref"].as_str().expect("run ref").to_string();

        let refresh_response = command_response(
            &service,
            workspace.path(),
            crate::app::GovernanceCommand::Refresh { json: true },
            Some(governance_refresh_request(
                &workspace,
                &run_ref,
                "requirements",
                "bounded-impact",
                "yellow",
                "product-lead",
            )),
        )
        .expect("refresh should succeed");

        assert_eq!(refresh_response["run_ref"], json!(run_ref));
        assert_eq!(refresh_response["status"], json!("governed_ready"));
        assert_eq!(refresh_response["packet_readiness"], json!("reusable"));
    }

    #[test]
    fn command_response_surfaces_approval_gated_architecture_runs() {
        let workspace = TempDir::new().expect("temp dir");
        fs::write(workspace.path().join("architecture.md"), complete_architecture_brief())
            .expect("architecture brief");
        let service = EngineService::new(workspace.path());

        let response = command_response(
            &service,
            workspace.path(),
            crate::app::GovernanceCommand::Start { json: true },
            Some(governance_start_request(
                &workspace,
                "architecture",
                "systemic-impact",
                "yellow",
                "staff-architect",
                Some("architecture.md"),
            )),
        )
        .expect("architecture start should succeed");

        assert_eq!(response["status"], json!("awaiting_approval"));
        assert_eq!(response["approval_state"], json!("requested"));
        assert_eq!(response["reason_code"], json!("approval_required"));
    }

    #[test]
    fn command_response_blocks_rejected_requirements_packets() {
        let workspace = TempDir::new().expect("temp dir");
        fs::write(workspace.path().join("requirements.md"), incomplete_requirements_brief())
            .expect("requirements brief");
        let service = EngineService::new(workspace.path());

        let response = command_response(
            &service,
            workspace.path(),
            crate::app::GovernanceCommand::Start { json: true },
            Some(governance_start_request(
                &workspace,
                "requirements",
                "bounded-impact",
                "yellow",
                "product-lead",
                Some("requirements.md"),
            )),
        )
        .expect("blocked start should still return a response");

        assert_eq!(response["status"], json!("blocked"));
        assert_eq!(response["packet_readiness"], json!("rejected"));
        assert_eq!(response["reason_code"], json!("rejected_packet"));
        assert!(response["missing_sections"].as_array().is_some_and(|sections| {
            sections.iter().any(|section| section == "problem-statement.md")
        }));
    }

    #[test]
    fn command_response_returns_failed_for_unknown_run_refs() {
        let workspace = TempDir::new().expect("temp dir");
        let service = EngineService::new(workspace.path());

        let response = command_response(
            &service,
            workspace.path(),
            crate::app::GovernanceCommand::Refresh { json: true },
            Some(governance_refresh_request(
                &workspace,
                "R-20260502-deadbeef",
                "verification",
                "bounded-impact",
                "yellow",
                "staff-engineer",
            )),
        )
        .expect("failed refresh should still return a response");

        assert_eq!(response["status"], json!("failed"));
        assert_eq!(response["reason_code"], json!("run_not_found"));
    }

    #[test]
    fn command_response_fails_when_artifact_contract_is_unreadable() {
        let workspace = TempDir::new().expect("temp dir");
        fs::write(workspace.path().join("requirements.md"), complete_requirements_brief())
            .expect("requirements brief");
        let service = EngineService::new(workspace.path());

        let start_response = command_response(
            &service,
            workspace.path(),
            crate::app::GovernanceCommand::Start { json: true },
            Some(governance_start_request(
                &workspace,
                "requirements",
                "bounded-impact",
                "yellow",
                "product-lead",
                Some("requirements.md"),
            )),
        )
        .expect("start should succeed");
        let run_ref = start_response["run_ref"].as_str().expect("run ref").to_string();

        let layout = ProjectLayout::new(workspace.path());
        fs::write(
            layout.run_dir(&run_ref).join("artifact-contract.toml"),
            "artifact_requirements = [",
        )
        .expect("corrupt artifact contract");

        let refresh_response = command_response(
            &service,
            workspace.path(),
            crate::app::GovernanceCommand::Refresh { json: true },
            Some(governance_refresh_request(
                &workspace,
                &run_ref,
                "requirements",
                "bounded-impact",
                "yellow",
                "product-lead",
            )),
        )
        .expect("refresh should return a machine-facing response");

        assert_eq!(refresh_response["status"], json!("failed"));
        assert_eq!(refresh_response["reason_code"], json!("artifact_contract_unreadable"));
        assert!(
            refresh_response["message"]
                .as_str()
                .is_some_and(|message| message.contains("artifact contract"))
        );
    }

    #[test]
    fn command_response_fails_when_artifact_contract_is_missing_after_artifacts_exist() {
        let workspace = TempDir::new().expect("temp dir");
        fs::write(workspace.path().join("requirements.md"), complete_requirements_brief())
            .expect("requirements brief");
        let service = EngineService::new(workspace.path());

        let start_response = command_response(
            &service,
            workspace.path(),
            crate::app::GovernanceCommand::Start { json: true },
            Some(governance_start_request(
                &workspace,
                "requirements",
                "bounded-impact",
                "yellow",
                "product-lead",
                Some("requirements.md"),
            )),
        )
        .expect("start should succeed");
        let run_ref = start_response["run_ref"].as_str().expect("run ref").to_string();

        let layout = ProjectLayout::new(workspace.path());
        fs::remove_file(layout.run_dir(&run_ref).join("artifact-contract.toml"))
            .expect("remove artifact contract");

        let refresh_response = command_response(
            &service,
            workspace.path(),
            crate::app::GovernanceCommand::Refresh { json: true },
            Some(governance_refresh_request(
                &workspace,
                &run_ref,
                "requirements",
                "bounded-impact",
                "yellow",
                "product-lead",
            )),
        )
        .expect("refresh should return a machine-facing response");

        assert_eq!(refresh_response["status"], json!("failed"));
        assert_eq!(refresh_response["reason_code"], json!("artifact_contract_missing"));
    }

    #[test]
    fn collect_input_references_deduplicates_workspace_relative_inputs() {
        let workspace = TempDir::new().expect("temp dir");
        fs::write(workspace.path().join("brief.md"), "# Brief\n").expect("brief file");

        let request = GovernanceRequest {
            bounded_context: GovernanceBoundedContext {
                stage_brief_ref: Some("brief.md".to_string()),
            },
            input_documents: vec![
                GovernanceInputDocument { path: Some("brief.md".to_string()) },
                GovernanceInputDocument { path: Some("./brief.md".to_string()) },
            ],
            ..GovernanceRequest::default()
        };

        assert_eq!(
            collect_input_references(workspace.path(), &request).expect("inputs should collect"),
            vec!["brief.md"]
        );
    }

    #[test]
    fn approval_state_value_covers_requested_granted_and_rejected_outcomes() {
        assert_eq!(approval_state_value(RunState::AwaitingApproval, false, false), "requested");
        assert_eq!(approval_state_value(RunState::Completed, true, false), "granted");
        assert_eq!(approval_state_value(RunState::Completed, false, true), "rejected");
    }

    #[test]
    fn response_defaults_cover_pending_blocked_and_running_states() {
        assert_eq!(
            packet_readiness_value(&Vec::new(), &Vec::new(), &Vec::new(), &Vec::new()),
            Some("pending".to_string())
        );
        assert_eq!(
            response_reason_code("blocked", Some("incomplete")).as_deref(),
            Some("incomplete_packet")
        );
        assert_eq!(
            default_headline("blocked", Some("rejected")).as_deref(),
            Some("Governed packet was rejected for downstream reuse")
        );
        assert_eq!(default_message("running", "R-1", None), "run `R-1` is still running");
    }

    #[test]
    fn map_engine_error_preserves_machine_readable_reason_codes() {
        let validation = map_engine_error(
            EngineError::Validation("missing evidence".to_string()),
            Some("R-1".to_string()),
        );
        assert_eq!(validation.reason_code.as_deref(), Some("domain_validation_failed"));
        assert_eq!(validation.run_ref.as_deref(), Some("R-1"));

        let unsupported =
            map_engine_error(EngineError::UnsupportedMode("legacy".to_string()), None);
        assert_eq!(unsupported.reason_code.as_deref(), Some("unsupported_mode"));

        let io_error =
            map_engine_error(EngineError::Io(std::io::Error::other("disk unavailable")), None);
        assert_eq!(io_error.reason_code.as_deref(), Some("workspace_unavailable"));
    }

    #[test]
    fn command_response_requires_request_bodies_for_stateful_operations() {
        let service = EngineService::new(".");

        let start_error = command_response(
            &service,
            std::path::Path::new("."),
            crate::app::GovernanceCommand::Start { json: true },
            None,
        )
        .expect_err("start should require a request body");
        assert!(start_error.to_string().contains("requires a JSON request body"));

        let refresh_error = command_response(
            &service,
            std::path::Path::new("."),
            crate::app::GovernanceCommand::Refresh { json: true },
            None,
        )
        .expect_err("refresh should require a request body");
        assert!(refresh_error.to_string().contains("requires a JSON request body"));
    }

    #[test]
    fn command_response_blocks_missing_fields_and_request_kind_mismatches() {
        let workspace = TempDir::new().expect("temp dir");
        let service = EngineService::new(workspace.path());

        let missing = command_response(
            &service,
            workspace.path(),
            crate::app::GovernanceCommand::Start { json: true },
            Some(GovernanceRequest::default()),
        )
        .expect("missing fields should return a blocked response");
        assert_eq!(missing["status"], json!("blocked"));
        assert_eq!(missing["reason_code"], json!("missing_required_field"));
        assert_eq!(
            missing["missing_fields"],
            json!([
                "request_kind",
                "governance_attempt_id",
                "stage_key",
                "goal",
                "workspace_ref",
                "mode",
                "system_context",
                "risk",
                "zone",
                "owner"
            ])
        );
        assert!(missing.get("missing_sections").is_none());

        let mismatch = command_response(
            &service,
            workspace.path(),
            crate::app::GovernanceCommand::Start { json: true },
            Some(GovernanceRequest {
                request_kind: Some("refresh".to_string()),
                governance_attempt_id: Some("ga-1".to_string()),
                stage_key: Some("analysis".to_string()),
                goal: Some("Create a governed packet".to_string()),
                workspace_ref: Some(workspace.path().to_string_lossy().into_owned()),
                mode: Some("requirements".to_string()),
                system_context: Some("existing".to_string()),
                risk: Some("bounded-impact".to_string()),
                zone: Some("yellow".to_string()),
                owner: Some("owner".to_string()),
                ..GovernanceRequest::default()
            }),
        )
        .expect("mismatch should return a blocked response");
        assert_eq!(mismatch["status"], json!("blocked"));
        assert_eq!(mismatch["reason_code"], json!("request_kind_mismatch"));
    }

    #[test]
    fn missing_domain_fields_for_refresh_include_run_ref() {
        let request = GovernanceRequest {
            request_kind: Some("refresh".to_string()),
            governance_attempt_id: Some("ga-1".to_string()),
            stage_key: Some("analysis".to_string()),
            goal: Some("Refresh the governed packet".to_string()),
            workspace_ref: Some(".".to_string()),
            mode: Some("verification".to_string()),
            system_context: Some("existing".to_string()),
            risk: Some("bounded-impact".to_string()),
            zone: Some("yellow".to_string()),
            owner: Some("owner".to_string()),
            ..GovernanceRequest::default()
        };

        assert_eq!(missing_domain_fields(&request, &GovernanceOperation::Refresh), vec!["run_ref"]);
    }

    #[test]
    fn command_response_rejects_workspace_mismatches_and_unavailable_inputs() {
        let workspace = TempDir::new().expect("temp dir");
        let other = TempDir::new().expect("other workspace");
        let service = EngineService::new(workspace.path());

        let mismatch = command_response(
            &service,
            workspace.path(),
            crate::app::GovernanceCommand::Start { json: true },
            Some(GovernanceRequest {
                request_kind: Some("start".to_string()),
                governance_attempt_id: Some("ga-1".to_string()),
                stage_key: Some("analysis".to_string()),
                goal: Some("Create a governed packet".to_string()),
                workspace_ref: Some(other.path().to_string_lossy().into_owned()),
                mode: Some("requirements".to_string()),
                system_context: Some("existing".to_string()),
                risk: Some("bounded-impact".to_string()),
                zone: Some("yellow".to_string()),
                owner: Some("owner".to_string()),
                ..GovernanceRequest::default()
            }),
        )
        .expect("workspace mismatch should return a blocked response");
        assert_eq!(mismatch["reason_code"], json!("workspace_mismatch"));

        let missing_document = command_response(
            &service,
            workspace.path(),
            crate::app::GovernanceCommand::Start { json: true },
            Some(governance_start_request(
                &workspace,
                "requirements",
                "bounded-impact",
                "yellow",
                "owner",
                Some("missing.md"),
            )),
        )
        .expect("missing document should return a blocked response");
        assert_eq!(missing_document["reason_code"], json!("input_document_missing"));
    }

    #[test]
    fn command_response_rejects_unsupported_domain_values() {
        let workspace = TempDir::new().expect("temp dir");
        let service = EngineService::new(workspace.path());

        for (mode, system_context, risk, zone, expected_reason) in [
            ("legacy", "existing", "bounded-impact", "yellow", "unsupported_mode"),
            ("requirements", "unknown", "bounded-impact", "yellow", "unsupported_system_context"),
            ("requirements", "existing", "unknown", "yellow", "unsupported_risk"),
            ("requirements", "existing", "bounded-impact", "unknown", "unsupported_zone"),
        ] {
            let response = command_response(
                &service,
                workspace.path(),
                crate::app::GovernanceCommand::Start { json: true },
                Some(GovernanceRequest {
                    request_kind: Some("start".to_string()),
                    governance_attempt_id: Some("ga-1".to_string()),
                    stage_key: Some("analysis".to_string()),
                    goal: Some("Create a governed packet".to_string()),
                    workspace_ref: Some(workspace.path().to_string_lossy().into_owned()),
                    mode: Some(mode.to_string()),
                    system_context: Some(system_context.to_string()),
                    risk: Some(risk.to_string()),
                    zone: Some(zone.to_string()),
                    owner: Some("owner".to_string()),
                    ..GovernanceRequest::default()
                }),
            )
            .expect("unsupported domain values should block");

            assert_eq!(response["status"], json!("blocked"));
            assert_eq!(response["reason_code"], json!(expected_reason));
        }
    }

    #[test]
    fn response_defaults_cover_additional_status_variants() {
        assert_eq!(normalized_status(RunState::Gated, None), "pending_selection");
        assert_eq!(normalized_status(RunState::Executing, None), "running");
        assert_eq!(normalized_status(RunState::Failed, None), "failed");

        assert_eq!(response_reason_code("blocked", None).as_deref(), Some("blocked_by_governance"));
        assert_eq!(
            default_headline("blocked", None).as_deref(),
            Some("Governance execution is blocked")
        );
        assert_eq!(
            default_headline("completed", None).as_deref(),
            Some("Governance execution completed")
        );
        assert_eq!(
            default_headline("running", None).as_deref(),
            Some("Governance execution is still running")
        );
        assert_eq!(
            default_headline("pending_selection", None).as_deref(),
            Some("Governance execution is still selecting the next action")
        );
        assert_eq!(
            default_headline("failed", None).as_deref(),
            Some("Governance execution failed")
        );
        assert_eq!(default_headline("unknown", None), None);

        assert_eq!(
            default_message("blocked", "R-1", Some("rejected")),
            "run `R-1` is blocked because the governed packet is not reusable"
        );
        assert_eq!(
            default_message("completed", "R-1", None),
            "run `R-1` completed without a reusable packet projection"
        );
        assert_eq!(
            default_message("pending_selection", "R-1", None),
            "run `R-1` is still selecting the next governed step"
        );
        assert_eq!(default_message("failed", "R-1", None), "run `R-1` failed");
        assert_eq!(
            default_message("unknown", "R-1", None),
            "run `R-1` returned an unknown governance status"
        );
    }

    #[test]
    fn helper_functions_cover_pending_packet_and_path_normalization_edges() {
        assert_eq!(
            packet_readiness_value(
                &["expected.md".to_string()],
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
            ),
            Some("pending".to_string())
        );

        assert_eq!(
            path_to_slash_string(std::path::Path::new("./nested/../packet.md")),
            "nested/packet.md"
        );
    }

    #[test]
    fn map_engine_error_covers_unexpected_engine_targets() {
        let response = map_engine_error(
            EngineError::UnsupportedInspectTarget("surprise".to_string()),
            Some("R-2".to_string()),
        );

        assert_eq!(response.reason_code.as_deref(), Some("runtime_error"));
        assert_eq!(response.run_ref.as_deref(), Some("R-2"));
    }

    #[test]
    fn missing_domain_fields_preserves_contract_order() {
        let request = GovernanceRequest {
            request_kind: Some("start".to_string()),
            governance_attempt_id: Some("ga-1".to_string()),
            stage_key: Some("analysis".to_string()),
            ..GovernanceRequest::default()
        };

        assert_eq!(
            missing_domain_fields(&request, &GovernanceOperation::Start),
            vec!["goal", "workspace_ref", "mode", "system_context", "risk", "zone", "owner"]
        );
    }

    #[test]
    fn completed_runs_only_become_governed_ready_with_reusable_packets() {
        assert_eq!(normalized_status(RunState::Completed, Some("reusable")), "governed_ready");
        assert_eq!(normalized_status(RunState::Completed, Some("incomplete")), "blocked");
        assert_eq!(normalized_status(RunState::Completed, Some("rejected")), "blocked");
        assert_eq!(normalized_status(RunState::Completed, Some("pending")), "completed");
    }

    #[test]
    fn packet_readiness_marks_missing_and_rejected_packets() {
        let expected = vec![
            ".canon/artifacts/run-1/requirements/problem-statement.md".to_string(),
            ".canon/artifacts/run-1/requirements/decision-checklist.md".to_string(),
        ];
        let present = vec![".canon/artifacts/run-1/requirements/problem-statement.md".to_string()];
        let missing = vec![".canon/artifacts/run-1/requirements/decision-checklist.md".to_string()];
        let rejected = vec![".canon/artifacts/run-1/requirements/problem-statement.md".to_string()];

        assert_eq!(
            packet_readiness_value(&expected, &present, &missing, &Vec::new()),
            Some("incomplete".to_string())
        );
        assert_eq!(
            packet_readiness_value(&expected, &present, &Vec::new(), &rejected),
            Some("rejected".to_string())
        );
    }

    #[test]
    fn normalize_workspace_relative_ref_rejects_escape_paths() {
        let workspace = TempDir::new().expect("temp dir");
        let outside = TempDir::new().expect("outside dir");
        let outside_file = outside.path().join("brief.md");
        fs::write(&outside_file, "# Brief\n").expect("outside file");

        let response = normalize_workspace_relative_ref(
            workspace.path(),
            outside_file.to_string_lossy().as_ref(),
        )
        .expect_err("outside document should be blocked");

        assert_eq!(
            *response,
            GovernanceResponse::blocked(
                "path_outside_workspace",
                format!(
                    "document `{}` escapes the declared workspace boundary",
                    outside_file.to_string_lossy()
                ),
                vec!["input_documents".to_string()]
            )
        );
    }

    #[test]
    fn artifact_marker_detection_uses_missing_authored_body_marker() {
        let workspace = TempDir::new().expect("temp dir");
        let document_ref = ".canon/artifacts/run-1/requirements/problem-statement.md";
        let document_path = workspace.path().join(document_ref);
        fs::create_dir_all(document_path.parent().expect("parent")).expect("artifact dir");
        fs::write(
            &document_path,
            "# Problem Statement\n\n## Missing Authored Body\n\nAuthor a real section.",
        )
        .expect("artifact file");

        assert!(artifact_contains_missing_authored_body(workspace.path(), document_ref));
    }

    #[test]
    fn packet_missing_sections_report_leaf_names_once() {
        let missing = vec![".canon/artifacts/run-1/requirements/decision-checklist.md".to_string()];
        let rejected = vec![
            ".canon/artifacts/run-1/requirements/problem-statement.md".to_string(),
            ".canon/artifacts/run-1/requirements/decision-checklist.md".to_string(),
        ];

        assert_eq!(
            packet_missing_sections(&missing, &rejected),
            vec!["decision-checklist.md", "problem-statement.md"]
        );
    }

    #[test]
    fn bounded_context_defaults_do_not_require_stage_brief() {
        let request = GovernanceRequest {
            request_kind: Some("refresh".to_string()),
            governance_attempt_id: Some("ga-2".to_string()),
            stage_key: Some("verification".to_string()),
            goal: Some("refresh".to_string()),
            workspace_ref: Some(".".to_string()),
            mode: Some("verification".to_string()),
            system_context: Some("existing".to_string()),
            risk: Some("bounded-impact".to_string()),
            zone: Some("yellow".to_string()),
            owner: Some("owner".to_string()),
            run_ref: Some("R-20260502-deadbeef".to_string()),
            bounded_context: GovernanceBoundedContext::default(),
            ..GovernanceRequest::default()
        };

        assert!(missing_domain_fields(&request, &GovernanceOperation::Refresh).is_empty());
    }
}
