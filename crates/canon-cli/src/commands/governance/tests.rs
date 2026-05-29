use std::fs;

use canon_engine::domain::artifact::{
    ArtifactContract, ArtifactFormat, ArtifactRecord, ArtifactRequirement,
};
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{ClassificationProvenance, RunContext};
use canon_engine::persistence::layout::ProjectLayout;
use canon_engine::persistence::manifests::{LinkManifest, RunManifest, RunStateManifest};
use canon_engine::persistence::store::{PersistedArtifact, PersistedRunBundle, WorkspaceStore};
use canon_engine::{EngineError, EngineService};
use serde_json::json;
use tempfile::TempDir;
use time::OffsetDateTime;

use super::error::map_engine_error;
use super::handlers::{
    handle_refresh, handle_start, missing_domain_fields, validate_workspace_binding,
};
use super::parsers::{collect_input_references, normalize_workspace_relative_ref};
use super::paths::{artifact_contains_missing_authored_body, path_to_slash_string};
use super::protocol::{command_response, read_request_from};
use super::status::{
    approval_state_value, default_headline, default_message, normalized_status,
    packet_missing_sections, packet_readiness_value, response_reason_code,
};
use super::{
    ApprovalState, GovernanceBoundedContext, GovernanceInputDocument, GovernanceOperation,
    GovernanceReasonCode, GovernanceRequest, GovernanceResponse, GovernanceStatus, PacketReadiness,
};
use canon_engine::domain::run::RunState;

fn completed_requirements_manifest(run_id: &str) -> RunManifest {
    RunManifest {
        run_id: run_id.to_string(),
        uuid: None,
        short_id: None,
        slug: None,
        title: None,
        mode: Mode::Requirements,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: None,
        classification: ClassificationProvenance::explicit(),
        owner: "product-lead".to_string(),
        lineage: None,
        created_at: OffsetDateTime::UNIX_EPOCH,
    }
}

fn completed_requirements_context(
    repo_root: &std::path::Path,
    manifest: &RunManifest,
) -> RunContext {
    RunContext {
        repo_root: repo_root.display().to_string(),
        owner: Some(manifest.owner.clone()),
        inputs: vec!["requirements.md".to_string()],
        excluded_paths: Vec::new(),
        input_fingerprints: Vec::new(),
        system_context: manifest.system_context,
        upstream_context: None,
        implementation_execution: None,
        refactor_execution: None,
        backlog_planning: None,
        clarification_refinement: None,
        inline_inputs: Vec::new(),
        captured_at: manifest.created_at,
    }
}

fn requirements_artifact_contract() -> ArtifactContract {
    ArtifactContract {
        version: 1,
        artifact_requirements: vec![ArtifactRequirement {
            file_name: "01-problem-statement.md".to_string(),
            format: ArtifactFormat::Markdown,
            required_sections: vec!["Summary".to_string()],
            gates: Vec::new(),
            required: true,
        }],
        required_verification_layers: Vec::new(),
    }
}

fn requirements_artifact(run_id: &str) -> PersistedArtifact {
    PersistedArtifact {
        record: ArtifactRecord {
            file_name: "01-problem-statement.md".to_string(),
            relative_path: format!("artifacts/{run_id}/requirements/01-problem-statement.md"),
            format: ArtifactFormat::Markdown,
            provenance: None,
        },
        contents: "# Problem Statement\n\n## Summary\n\nReusable governed packet.\n".to_string(),
    }
}

fn persist_completed_requirements_packet(workspace: &std::path::Path, run_id: &str) {
    let manifest = completed_requirements_manifest(run_id);
    let store = WorkspaceStore::new(workspace);
    let bundle = PersistedRunBundle {
        run: manifest.clone(),
        context: completed_requirements_context(workspace, &manifest),
        state: RunStateManifest { state: RunState::Completed, updated_at: manifest.created_at },
        artifact_contract: requirements_artifact_contract(),
        artifacts: vec![requirements_artifact(run_id)],
        links: LinkManifest {
            artifacts: Vec::new(),
            decisions: Vec::new(),
            traces: Vec::new(),
            invocations: Vec::new(),
            evidence: None,
        },
        gates: Vec::new(),
        approvals: Vec::new(),
        verification_records: Vec::new(),
        evidence: None,
        invocations: Vec::new(),
    };

    store.persist_run_bundle(&bundle).expect("persist completed requirements packet");
}

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
fn command_response_starts_targeted_requirements_packets_in_pending_selection() {
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

    assert_eq!(response["status"], json!("pending_selection"));
    assert_eq!(response["approval_state"], json!("not_needed"));
    assert_eq!(response["packet_readiness"], json!("incomplete"));
    assert!(response["packet_ref"].as_str().is_some_and(|value| !value.is_empty()));
    assert!(response["expected_document_refs"].as_array().is_some_and(|refs| !refs.is_empty()));
}

#[test]
fn command_response_refreshes_existing_governed_runs() {
    let workspace = TempDir::new().expect("temp dir");
    let service = EngineService::new(workspace.path());
    let run_ref = "R-20260529-governed0001";
    persist_completed_requirements_packet(workspace.path(), run_ref);

    let refresh_response = command_response(
        &service,
        workspace.path(),
        crate::app::GovernanceCommand::Refresh { json: true },
        Some(governance_refresh_request(
            &workspace,
            run_ref,
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
fn command_response_starts_targeted_architecture_packets_in_pending_selection() {
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

    assert_eq!(response["status"], json!("pending_selection"));
    assert_eq!(response["approval_state"], json!("not_needed"));
    assert_eq!(response["packet_readiness"], json!("incomplete"));
}

#[test]
fn command_response_starts_incomplete_targeted_requirements_packets_in_pending_selection() {
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

    assert_eq!(response["status"], json!("pending_selection"));
    assert_eq!(response["packet_readiness"], json!("incomplete"));
    assert!(response["missing_sections"].as_array().is_some_and(|sections| !sections.is_empty()));
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
    fs::write(layout.run_dir(&run_ref).join("artifact-contract.toml"), "artifact_requirements = [")
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
    let service = EngineService::new(workspace.path());
    let run_ref = "R-20260529-missingcontract01";
    persist_completed_requirements_packet(workspace.path(), run_ref);

    let layout = ProjectLayout::new(workspace.path());
    fs::remove_file(layout.run_dir(run_ref).join("artifact-contract.toml"))
        .expect("remove artifact contract");

    let refresh_response = command_response(
        &service,
        workspace.path(),
        crate::app::GovernanceCommand::Refresh { json: true },
        Some(governance_refresh_request(
            &workspace,
            run_ref,
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
        bounded_context: GovernanceBoundedContext { stage_brief_ref: Some("brief.md".to_string()) },
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
    assert_eq!(
        approval_state_value(RunState::AwaitingApproval, false, false),
        ApprovalState::Requested
    );
    assert_eq!(approval_state_value(RunState::Completed, true, false), ApprovalState::Granted);
    assert_eq!(approval_state_value(RunState::Completed, false, true), ApprovalState::Rejected);
}

#[test]
fn response_defaults_cover_pending_blocked_and_running_states() {
    assert_eq!(
        packet_readiness_value(&Vec::new(), &Vec::new(), &Vec::new(), &Vec::new()),
        Some(PacketReadiness::Pending)
    );
    assert_eq!(
        response_reason_code(GovernanceStatus::Blocked, Some(PacketReadiness::Incomplete)),
        Some(GovernanceReasonCode::IncompletePacket)
    );
    assert_eq!(
        default_headline(GovernanceStatus::Blocked, Some(PacketReadiness::Rejected)).as_deref(),
        Some("Governed packet was rejected for downstream reuse")
    );
    assert_eq!(
        default_message(GovernanceStatus::Running, "R-1", None),
        "run `R-1` is still running"
    );
}

#[test]
fn map_engine_error_preserves_machine_readable_reason_codes() {
    let validation = map_engine_error(
        EngineError::Validation("missing evidence".to_string()),
        Some("R-1".to_string()),
    );
    assert_eq!(validation.reason_code, Some(GovernanceReasonCode::DomainValidationFailed));
    assert_eq!(validation.run_ref.as_deref(), Some("R-1"));

    let unsupported = map_engine_error(EngineError::UnsupportedMode("legacy".to_string()), None);
    assert_eq!(unsupported.reason_code, Some(GovernanceReasonCode::UnsupportedMode));

    let io_error =
        map_engine_error(EngineError::Io(std::io::Error::other("disk unavailable")), None);
    assert_eq!(io_error.reason_code, Some(GovernanceReasonCode::WorkspaceUnavailable));
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
fn handle_start_maps_engine_validation_failures() {
    let workspace = TempDir::new().expect("temp dir");
    let service = EngineService::new(workspace.path());

    let response = handle_start(
        &service,
        workspace.path(),
        governance_start_request(
            &workspace,
            "requirements",
            "bounded-impact",
            "yellow",
            "owner",
            None,
        ),
    );

    assert_eq!(response.status, GovernanceStatus::Blocked);
    assert_eq!(response.reason_code, Some(GovernanceReasonCode::DomainValidationFailed));
}

#[test]
fn handle_refresh_returns_validation_failures_directly() {
    let workspace = TempDir::new().expect("temp dir");

    let response = handle_refresh(workspace.path(), GovernanceRequest::default());

    assert_eq!(response.status, GovernanceStatus::Blocked);
    assert_eq!(response.reason_code, Some(GovernanceReasonCode::MissingRequiredField));
}

#[test]
fn validate_workspace_binding_reports_unavailable_repo_root_and_requested_workspace() {
    let workspace = TempDir::new().expect("temp dir");
    let missing_root = workspace.path().join("missing-root");

    let missing_repo =
        validate_workspace_binding(&missing_root, workspace.path().to_string_lossy().as_ref())
            .expect_err("missing repo root should fail");
    assert_eq!(missing_repo.reason_code, Some(GovernanceReasonCode::WorkspaceUnavailable));
    assert!(missing_repo.message.starts_with("workspace is not accessible:"));

    let missing_requested = validate_workspace_binding(workspace.path(), "missing-dir")
        .expect_err("missing requested workspace should fail");
    assert_eq!(
        *missing_requested,
        GovernanceResponse::failed(
            GovernanceReasonCode::WorkspaceUnavailable,
            "workspace `missing-dir` is not accessible",
            None,
        )
    );
}

#[test]
fn normalize_workspace_relative_ref_reports_unavailable_workspace_root() {
    let workspace = TempDir::new().expect("temp dir");
    let missing_root = workspace.path().join("missing-root");

    let response = normalize_workspace_relative_ref(&missing_root, "brief.md")
        .expect_err("missing workspace root should fail");

    assert_eq!(response.reason_code, Some(GovernanceReasonCode::WorkspaceUnavailable));
    assert!(response.message.starts_with("workspace is not accessible:"));
}

#[test]
fn response_defaults_cover_additional_status_variants() {
    assert_eq!(normalized_status(RunState::Gated, None), GovernanceStatus::PendingSelection);
    assert_eq!(normalized_status(RunState::Executing, None), GovernanceStatus::Running);
    assert_eq!(normalized_status(RunState::Failed, None), GovernanceStatus::Failed);

    assert_eq!(
        response_reason_code(GovernanceStatus::Blocked, None),
        Some(GovernanceReasonCode::BlockedByGovernance)
    );
    assert_eq!(
        default_headline(GovernanceStatus::Blocked, None).as_deref(),
        Some("Governance execution is blocked")
    );
    assert_eq!(
        default_headline(GovernanceStatus::Completed, None).as_deref(),
        Some("Governance execution completed")
    );
    assert_eq!(
        default_headline(GovernanceStatus::Running, None).as_deref(),
        Some("Governance execution is still running")
    );
    assert_eq!(
        default_headline(GovernanceStatus::PendingSelection, None).as_deref(),
        Some("Governance execution is still selecting the next action")
    );
    assert_eq!(
        default_headline(GovernanceStatus::Failed, None).as_deref(),
        Some("Governance execution failed")
    );

    assert_eq!(
        default_message(GovernanceStatus::Blocked, "R-1", Some(PacketReadiness::Rejected)),
        "run `R-1` is blocked because the governed packet is not reusable"
    );
    assert_eq!(
        default_message(GovernanceStatus::Completed, "R-1", None),
        "run `R-1` completed without a reusable packet projection"
    );
    assert_eq!(
        default_message(GovernanceStatus::PendingSelection, "R-1", None),
        "run `R-1` is still selecting the next governed step"
    );
    assert_eq!(default_message(GovernanceStatus::Failed, "R-1", None), "run `R-1` failed");
}

#[test]
fn helper_functions_cover_pending_packet_and_path_normalization_edges() {
    assert_eq!(
        packet_readiness_value(&["expected.md".to_string()], &Vec::new(), &Vec::new(), &Vec::new(),),
        Some(PacketReadiness::Pending)
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

    assert_eq!(response.reason_code, Some(GovernanceReasonCode::RuntimeError));
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
    assert_eq!(
        normalized_status(RunState::Completed, Some(PacketReadiness::Reusable)),
        GovernanceStatus::GovernedReady
    );
    assert_eq!(
        normalized_status(RunState::Completed, Some(PacketReadiness::Incomplete)),
        GovernanceStatus::Blocked
    );
    assert_eq!(
        normalized_status(RunState::Completed, Some(PacketReadiness::Rejected)),
        GovernanceStatus::Blocked
    );
    assert_eq!(
        normalized_status(RunState::Completed, Some(PacketReadiness::Pending)),
        GovernanceStatus::Completed
    );
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
        Some(PacketReadiness::Incomplete)
    );
    assert_eq!(
        packet_readiness_value(&expected, &present, &Vec::new(), &rejected),
        Some(PacketReadiness::Rejected)
    );
}

#[test]
fn normalize_workspace_relative_ref_rejects_escape_paths() {
    let workspace = TempDir::new().expect("temp dir");
    let outside = TempDir::new().expect("outside dir");
    let outside_file = outside.path().join("brief.md");
    fs::write(&outside_file, "# Brief\n").expect("outside file");

    let response =
        normalize_workspace_relative_ref(workspace.path(), outside_file.to_string_lossy().as_ref())
            .expect_err("outside document should be blocked");

    assert_eq!(
        *response,
        GovernanceResponse::blocked(
            GovernanceReasonCode::PathOutsideWorkspace,
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
