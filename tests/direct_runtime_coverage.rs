use std::fs;
use std::process::Command as ProcessCommand;

use canon_adapters::shell::ShellAdapter;
use canon_adapters::{
    AdapterError, CapabilityKind, InvocationOrientation, SideEffectClass, TrustBoundaryKind,
};
use canon_engine::EngineService;
use canon_engine::artifacts::contract::{
    contract_for_mode, validate_artifact, validate_release_bundle,
};
use canon_engine::domain::approval::ApprovalDecision;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{ClassificationProvenance, SystemContext};
use canon_engine::orchestrator::service::{AiTool, InspectEntry, InspectTarget, RunRequest};
use tempfile::TempDir;

fn request(
    mode: Mode,
    risk: RiskClass,
    zone: UsageZone,
    owner: &str,
    inputs: Vec<&str>,
) -> RunRequest {
    RunRequest {
        mode,
        risk,
        zone,
        system_context: match mode {
            Mode::Change
            | Mode::Backlog
            | Mode::SystemShaping
            | Mode::Architecture
            | Mode::Implementation
            | Mode::Refactor
            | Mode::Migration
            | Mode::Incident => Some(SystemContext::Existing),
            _ => None,
        },
        classification: ClassificationProvenance::explicit(),
        owner: owner.to_string(),
        inputs: inputs.into_iter().map(ToString::to_string).collect(),
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

fn git(workspace: &TempDir, args: &[&str]) {
    let output = ProcessCommand::new("git")
        .args(args)
        .current_dir(workspace.path())
        .output()
        .expect("git command");
    assert!(
        output.status.success(),
        "git {:?} failed: stdout=`{}` stderr=`{}`",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_change_repo(workspace: &TempDir) {
    git(workspace, &["init", "-b", "main"]);
    git(workspace, &["config", "user.name", "Canon Test"]);
    git(workspace, &["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("src/auth")).expect("src dir");
    fs::create_dir_all(workspace.path().join("tests")).expect("tests dir");
    fs::write(
        workspace.path().join("src/auth/session.rs"),
        "pub fn revoke_session(id: &str) -> String {\n    format!(\"revoked:{id}\")\n}\n",
    )
    .expect("source file");
    fs::write(
        workspace.path().join("tests/session.md"),
        "# Session Checks\n\n- revocation formatting remains stable\n",
    )
    .expect("test file");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "seed change repo"]);
}

fn complete_implementation_brief() -> &'static str {
    r#"# Implementation Brief

Feature Slice: Auth session revocation repository wiring inside the existing login subsystem.
Primary Upstream Mode: change

## Task Mapping
1. Add bounded auth session repository helpers.
2. Thread the new helper through the revocation service without expanding the public API.
3. Record implementation notes for operator review and rollback.

## Bounded Changes
- Auth session repository helper wiring.
- Revocation service internal composition.

## Mutation Bounds
src/auth/session.rs and src/auth/repository.rs only.

## Allowed Paths
- src/auth/session.rs
- src/auth/repository.rs

## Executed Changes
- Add the bounded repository helper and thread it through the revocation service without widening the public API.

## Task Linkage
- Step 1 adds the helper.
- Step 2 rewires the service behind the existing external contract.
- Step 3 records the resulting packet and rollback posture.

## Completion Evidence
- The emitted implementation packet and focused tests confirm the bounded slice is ready for operator review.

## Remaining Risks
- Repository wiring could still drift into adjacent auth modules if the bounded paths expand during review.

## Safety-Net Evidence
Contract coverage protects revocation formatting and audit ordering before mutation.

## Independent Checks
- cargo test --test session_contract
- cargo test --test auth_audit_ordering

## Rollback Triggers
Revocation output drifts, audit ordering becomes unstable, or repository wiring expands beyond the declared auth-session slice.

## Rollback Steps
1. Revert the bounded auth-session patch.
2. Redeploy the previous build.
3. Restore the last known-good audit ordering snapshot.
"#
}

fn complete_refactor_brief() -> &'static str {
    r#"# Refactor Brief

Feature Slice: Auth session boundary and repository composition inside the existing login subsystem.
Primary Upstream Mode: implementation

## Preserved Behavior
Session revocation formatting and audit ordering remain externally unchanged.

## Approved Exceptions
None.

## Refactor Scope
Auth session boundary and repository composition only.

## Allowed Paths
- src/auth/session.rs
- src/auth/repository.rs

## Structural Rationale
Isolate persistence concerns and internal composition without changing externally meaningful behavior.

## Untouched Surface
Public auth API, tests/session.md, deployment wiring, and analytics consumers stay unchanged.

## Safety-Net Evidence
Contract coverage protects revocation formatting and audit ordering before structural cleanup.

## Regression Findings
No regression findings are accepted in this bounded packet.

## Contract Drift
No public contract drift is allowed.

## Reviewer Notes
Reviewer confirmation is required before any drift or feature semantics are accepted.

## Feature Audit
No new feature behavior is introduced in this refactor packet.

## Decision
Preserve behavior and stop immediately if the surface expands or the packet starts to add feature semantics.
"#
}

fn complete_backlog_brief() -> &'static str {
    "# Backlog Brief\n\n## Delivery Intent\nPrepare a bounded delivery backlog for auth session hardening.\n\n## Desired Granularity\nepic-plus-slice\n\n## Planning Horizon\nnext two releases\n\n## Source References\n- docs/changes/auth-session.md\n- docs/architecture/auth-boundary.md\n\n## Priorities\n- Ship the rollback-safe slice first.\n- Keep dependency blockers explicit.\n\n## Constraints\n- Keep the output above task level.\n\n## Out of Scope\n- Login UI redesign\n"
}

fn complete_incident_brief() -> &'static str {
    "# Incident Brief\n\nIncident Scope: payments-api and checkout flow only.\nTrigger And Current State: elevated 5xx responses after the last deploy.\nOperational Constraints: no autonomous remediation and no schema changes.\nKnown Facts:\n- errors started after the deploy\n- rollback remains available\nWorking Hypotheses:\n- retry amplification is exhausting the service\nEvidence Gaps:\n- database saturation is not yet confirmed\nImpacted Surfaces:\n- payments-api\n- checkout flow\nPropagation Paths:\n- checkout request path\nConfidence And Unknowns:\n- medium confidence until saturation evidence is collected\nImmediate Actions:\n- disable async retries\nOrdered Sequence:\n- capture blast radius\n- disable retries\n- reassess error rate\nStop Conditions:\n- error rate stabilizes below the alert threshold\nDecision Points:\n- decide whether rollback is still required\nApproved Actions:\n- disable retries within the bounded surface\nDeferred Actions:\n- schema-level changes remain out of scope\nVerification Checks:\n- confirm 5xx rate drops\nRelease Readiness:\n- keep recommendation-only posture until the owner accepts the packet\nFollow-Up Work:\n- add a saturation dashboard and post-incident review item\n"
}

fn complete_migration_brief() -> &'static str {
    "# Migration Brief\n\nCurrent State: auth-v1 serves login and token refresh traffic.\nTarget State: auth-v2 serves the same bounded traffic surface.\nTransition Boundaries: login and token refresh only.\nGuaranteed Compatibility:\n- existing tokens continue to validate\nTemporary Incompatibilities:\n- admin reporting stays on v1 during the rollout\nCoexistence Rules:\n- dual-write session metadata during cutover\nOrdered Steps:\n- enable shadow reads\n- start dual-write\n- cut traffic to auth-v2\nParallelizable Work:\n- docs and dashboards can update in parallel\nCutover Criteria:\n- error rate and token validation remain stable\nRollback Triggers:\n- token validation failures or elevated login errors\nFallback Paths:\n- route bounded traffic back to auth-v1\nRe-Entry Criteria:\n- compatibility regressions are resolved and revalidated\nVerification Checks:\n- login and token validation pass against auth-v2\nResidual Risks:\n- admin reporting remains temporarily inconsistent\nRelease Readiness:\n- keep recommendation-only posture until the owner accepts the packet\nMigration Decisions:\n- retain dual-write during the bounded cutover\nDeferred Decisions:\n- move admin reporting after the bounded migration completes\nApproval Notes:\n- explicit migration-lead sign-off is required before broader rollout\n"
}

fn init_review_repo(workspace: &TempDir) {
    git(workspace, &["init", "-b", "main"]);
    git(workspace, &["config", "user.name", "Canon Test"]);
    git(workspace, &["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("src")).expect("src dir");
    fs::create_dir_all(workspace.path().join("tests")).expect("tests dir");
    fs::write(
        workspace.path().join("src/reviewer.rs"),
        "pub fn format_review(label: &str) -> String {\n    format!(\"review:{label}\")\n}\n",
    )
    .expect("base source");
    fs::write(
        workspace.path().join("tests/reviewer.md"),
        "# Review Checks\n\nExisting tests cover the formatting helper.\n",
    )
    .expect("base tests");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "base review helper"]);
    git(workspace, &["checkout", "-b", "feature/pr-review"]);
}

fn add_completed_review_diff(workspace: &TempDir) {
    fs::write(
        workspace.path().join("src/reviewer.rs"),
        "pub fn format_review(label: &str) -> String {\n    format!(\"review:{}\", label.trim())\n}\n",
    )
    .expect("updated source");
    fs::write(
        workspace.path().join("tests/reviewer.md"),
        "# Review Checks\n\n- formatting trims trailing whitespace before labeling\n",
    )
    .expect("updated tests");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "trim review labels"]);
}

fn artifact_names(entries: &[InspectEntry]) -> Vec<String> {
    entries
        .iter()
        .map(|entry| match entry {
            InspectEntry::Name(name) => name.clone(),
            other => panic!("expected name entry, got {other:?}"),
        })
        .collect()
}

fn find_run_context_toml(workspace: &TempDir, run_id: &str) -> std::path::PathBuf {
    let runs_root = workspace.path().join(".canon").join("runs");
    for year_entry in fs::read_dir(&runs_root).expect("runs root") {
        let year_entry = year_entry.expect("year entry");
        if !year_entry.path().is_dir() {
            continue;
        }
        for month_entry in fs::read_dir(year_entry.path()).expect("month dir") {
            let month_entry = month_entry.expect("month entry");
            if !month_entry.path().is_dir() {
                continue;
            }
            for run_entry in fs::read_dir(month_entry.path()).expect("run dir") {
                let run_entry = run_entry.expect("run entry");
                let name = run_entry.file_name();
                let Some(name) = name.to_str() else {
                    continue;
                };
                if name == run_id || name.starts_with(&format!("{run_id}--")) {
                    return run_entry.path().join("context.toml");
                }
            }
        }
    }
    panic!("context.toml not found for run {run_id}");
}

#[test]
fn engine_service_initializes_runtime_and_materializes_skills() {
    let workspace = TempDir::new().expect("temp dir");
    let service = EngineService::new(workspace.path());

    let init = service.init(None).expect("init summary");
    assert!(init.methods_materialized > 0);
    assert!(init.policies_materialized > 0);
    assert!(workspace.path().join(".canon").exists());

    let modes = service.inspect(InspectTarget::Modes).expect("inspect modes");
    let mode_names = artifact_names(&modes.entries);
    assert!(mode_names.contains(&"discovery".to_string()));
    assert!(mode_names.contains(&"architecture".to_string()));

    let methods = service.inspect(InspectTarget::Methods).expect("inspect methods");
    assert!(!methods.entries.is_empty());

    let policies = service.inspect(InspectTarget::Policies).expect("inspect policies");
    assert!(!policies.entries.is_empty());

    let installed = service.skills_install(AiTool::Codex).expect("skills install");
    assert!(installed.skills_materialized > 0);

    let listed = service.skills_list();
    assert!(listed.iter().any(|entry| entry.name == "canon-discovery"));
    assert!(listed.iter().any(|entry| entry.name == "canon-inspect-clarity"));
    assert!(listed.iter().any(|entry| entry.name == "canon-incident"));
    assert!(listed.iter().any(|entry| entry.name == "canon-migration"));

    let updated = service.skills_update(AiTool::Codex).expect("skills update");
    assert!(updated.skills_materialized > 0 || updated.skills_skipped > 0);
}

#[test]
fn requirements_direct_run_approves_invocation_and_resumes_to_completion() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("idea.md"),
        "# Requirements Brief\n\n## Problem\n\nSystemic requirements framing still needs governed approval.\n\n## Outcome\n\nThe packet remains reviewable after approval and resume.\n\n## Constraints\n\n- Preserve explicit approvals\n- Keep artifacts durable\n\n## Non-Negotiables\n\n- Human ownership remains explicit\n\n## Options\n\n1. Govern the packet before broader rollout.\n\n## Recommended Path\n\nGovern the packet before broader rollout.\n\n## Tradeoffs\n\n- Governance adds review steps.\n\n## Consequences\n\n- Review remains auditable.\n\n## Out of Scope\n\n- No unreviewed rollout.\n\n## Deferred Work\n\n- Broader rollout stays deferred.\n\n## Decision Checklist\n\n- [x] Risk is explicit\n\n## Open Questions\n\n- Which downstream mode consumes this packet next?\n",
    )
    .expect("idea file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::Requirements,
            RiskClass::SystemicImpact,
            UsageZone::Yellow,
            "product-lead",
            vec!["idea.md"],
        ))
        .expect("requirements run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert!(summary.invocations_total >= 3);
    assert_eq!(summary.blocking_classification.as_deref(), Some("approval-gated"));

    let invocations = service
        .inspect(InspectTarget::Invocations { run_id: summary.run_id.clone() })
        .expect("inspect invocations");
    let pending_request_id = invocations
        .entries
        .iter()
        .find_map(|entry| match entry {
            InspectEntry::Invocation(summary) if summary.policy_decision == "NeedsApproval" => {
                Some(summary.request_id.clone())
            }
            _ => None,
        })
        .expect("pending invocation approval");

    let approval = service
        .approve(
            &summary.run_id,
            &format!("invocation:{pending_request_id}"),
            "principal-engineer",
            ApprovalDecision::Approve,
            "Systemic framing may proceed with explicit human ownership.",
        )
        .expect("approval summary");
    assert_eq!(approval.state, "AwaitingApproval");

    let resumed = service.resume(&summary.run_id).expect("resume requirements run");
    assert_eq!(resumed.state, "Completed");
    assert_eq!(resumed.invocations_denied, 1);

    let artifacts = service
        .inspect(InspectTarget::Artifacts { run_id: summary.run_id.clone() })
        .expect("inspect artifacts");
    let artifact_paths = artifact_names(&artifacts.entries);
    assert!(artifact_paths.iter().any(|path| path.ends_with("problem-statement.md")));
    assert!(artifact_paths.iter().any(|path| path.ends_with("decision-checklist.md")));

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Completed");
    assert_eq!(status.pending_invocation_approvals, 0);
}

#[test]
fn discovery_direct_run_persists_completed_artifacts_and_evidence() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("discovery.md"),
        "# Discovery Brief\n\n## Problem Domain\n\nReconcile Canon mode coverage with real governed runtime depth.\n\n## Repo Surface\n\n- crates/canon-engine/src/orchestrator/service/\n- tests/direct_runtime_coverage.rs\n\n## Immediate Tensions\n\n- Discovery packets need stable authored contracts.\n\n## Downstream Handoff\n\nTranslate discovery into requirements mode with repo-specific scope cuts.\n\n## Unknowns\n\n- Which downstream mode should consume repo-grounded discovery first?\n\n## Assumptions\n\n- The existing runtime shape and evidence model remain stable.\n\n## Validation Targets\n\n- Confirm authored headings survive into emitted artifacts.\n\n## Confidence Levels\n\n- Medium until end-to-end runs confirm the contract.\n\n## In-Scope Context\n\n- Governed analysis modes only.\n\n## Out-of-Scope Context\n\n- No architecture or review-mode changes in this packet.\n\n## Translation Trigger\n\nTranslate discovery into requirements mode with repo-specific scope cuts.\n\n## Options\n\n1. Tighten discovery authoring contracts first.\n\n## Constraints\n\n- Preserve the existing runtime shape and evidence model.\n\n## Recommended Direction\n\nTighten discovery authoring contracts first.\n\n## Next-Phase Shape\n\nTranslate discovery into requirements mode with repo-specific scope cuts.\n\n## Pressure Points\n\n- Repo-local skills and runtime outputs can drift without a shared authored contract.\n\n## Blocking Decisions\n\n- Decide whether discovery specialization remains a first slice only.\n\n## Open Questions\n\n- Which downstream mode should consume repo-grounded discovery first?\n\n## Recommended Owner\n\n- researcher\n",
    )
    .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::Discovery,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "researcher",
            vec!["discovery.md"],
        ))
        .expect("discovery run");

    assert_eq!(summary.state, "Completed");
    assert_eq!(summary.invocations_total, 4);
    assert_eq!(summary.approval_targets, Vec::<String>::new());

    let artifacts = service
        .inspect(InspectTarget::Artifacts { run_id: summary.run_id.clone() })
        .expect("inspect artifacts");
    let artifact_paths = artifact_names(&artifacts.entries);
    assert_eq!(artifact_paths.len(), 5);
    assert!(artifact_paths.iter().any(|path| path.ends_with("problem-map.md")));

    let evidence = service
        .inspect(InspectTarget::Evidence { run_id: summary.run_id.clone() })
        .expect("inspect evidence");
    let evidence_entry = match evidence.entries.first().expect("evidence entry") {
        InspectEntry::Evidence(entry) => entry,
        other => panic!("expected evidence entry, got {other:?}"),
    };
    assert!(!evidence_entry.generation_paths.is_empty());
    assert!(!evidence_entry.validation_paths.is_empty());

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Completed");
    assert!(status.validation_independence_satisfied);
}

#[test]
fn system_shaping_direct_run_covers_completed_and_blocked_paths() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("system-shaping.md"),
        "# System Shaping Brief\n\nIntent: shape a new governed Canon workflow surface for incomplete analysis modes.\nConstraint: keep the runtime adapters, policy gates, and evidence model intact.\n\n## System Shape\nKeep the review surface grounded in authored packet sections rather than synthesized prose.\n\n## Boundary Decisions\n- Keep authored packet sections explicit per emitted artifact.\n- Keep gates, approvals, and publish destinations unchanged.\n\n## Domain Responsibilities\n- Identify candidate bounded contexts.\n- Surface ubiquitous language and weak terminology.\n- Preserve domain invariants for downstream modes.\n\n## Candidate Bounded Contexts\n- Runtime Governance: owns run lifecycle, approvals, and evidence lineage.\n- Artifact Authoring: owns packet structure and authored-body rendering.\n\n## Core And Supporting Domain Hypotheses\n- Runtime Governance is core because it protects Canon's operating model.\n- Artifact Authoring is supporting because it exists to make reviews durable.\n\n## Ubiquitous Language\n- Run: one governed Canon execution with durable evidence.\n- Packet: the artifact bundle produced by a mode.\n\n## Domain Invariants\n- Approval semantics remain unchanged.\n- Publish destinations remain unchanged.\n\n## Boundary Risks And Open Questions\n- The split between authoring and governance may still leak through shared helpers.\n\n## Structural Options\n- Option 1 keeps authored-body preservation local to the current renderer helpers.\n- Option 2 introduces a new system-shaping-specific mapping layer before rendering.\n\n## Selected Boundaries\n- Runtime Governance remains separate from Artifact Authoring so packet fidelity does not blur approval semantics.\n\n## Rationale\n- Explicit authored sections make the packet reviewable without widening Canon's execution model.\n\n## Capabilities\n- Bounded system-shape definition.\n- Context and invariant capture.\n- Reviewable sequencing and risk surfacing.\n\n## Dependencies\n- Existing policy gates.\n- Existing evidence persistence.\n- Existing renderer helpers that already support authored-body preservation.\n\n## Gaps\n- Near-match heading handling still needs explicit tests.\n- Some user-facing docs still lag the runtime contract.\n\n## Delivery Phases\n1. Extend authored-body preservation to the remaining system-shaping artifacts.\n2. Synchronize skills, templates, and worked examples with the runtime contract.\n3. Close remaining validation and non-regression gaps.\n\n## Sequencing Rationale\n- Runtime fidelity must land before documentation and release guidance so later surfaces describe real behavior.\n\n## Risk per Phase\n- Phase 1: renderer changes could silently regress packet fidelity.\n- Phase 2: docs could drift from the runtime contract.\n- Phase 3: release notes could overstate rollout completeness.\n\n## Hotspots\n- Shared helpers that mix authored text with generated summaries.\n- Mode-specific artifact families that still rely on legacy headings.\n\n## Mitigation Status\n- Shared authored-section rendering is already available and can be reused.\n- Existing contract coverage can contain section-level regressions once expanded.\n\n## Unresolved Risks\n- Legacy worked examples could keep teaching inline labels unless updated.\n- Non-target modes still need explicit non-regression validation.\n",
    )
    .expect("brief file");

    let service = EngineService::new(workspace.path());
    let completed = service
        .run(request(
            Mode::SystemShaping,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "architect",
            vec!["system-shaping.md"],
        ))
        .expect("system-shaping run");

    assert_eq!(completed.state, "Completed");
    assert_eq!(completed.invocations_total, 3);

    let completed_status = service.status(&completed.run_id).expect("completed status");
    assert_eq!(completed_status.state, "Completed");
    assert!(!completed_status.validation_independence_satisfied);

    let blocked_workspace = TempDir::new().expect("temp dir");
    fs::write(
        blocked_workspace.path().join("system-shaping.md"),
        "# System Shaping Brief\n\nNeed a future-looking shape for analysis mode support.\n",
    )
    .expect("underspecified brief");

    let blocked_service = EngineService::new(blocked_workspace.path());
    let blocked = blocked_service
        .run(request(
            Mode::SystemShaping,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "architect",
            vec!["system-shaping.md"],
        ))
        .expect("blocked system-shaping run");

    assert_eq!(blocked.state, "Blocked");
    assert_eq!(blocked.blocking_classification.as_deref(), Some("artifact-blocked"));
    assert!(blocked.blocked_gates.iter().any(|gate| {
        gate.gate == "architecture"
            && gate.blockers.iter().any(|blocker| blocker.contains("lacks sufficient evidence"))
    }));
}

#[test]
fn architecture_direct_run_requires_gate_approval_and_completes_after_status_refresh() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("architecture.md"),
        "# Architecture Brief\n\nDecision focus: identify boundary ownership and tradeoffs for analysis-mode expansion.\nConstraint: preserve Canon runtime contracts, approvals, and evidence persistence.\n\n## Decision\nMake bounded contexts and context relationships first-class in architecture packets.\n\n## Constraints\n- Preserve approval semantics and publish destinations.\n- Keep non-target modes unchanged.\n\n## Candidate Boundaries\n- Runtime Governance\n- Artifact Authoring\n\n## Invariants\n- Run identity remains unchanged.\n- Evidence lineage remains reviewable.\n\n## Evaluation Criteria\n- Boundary clarity\n- Coupling visibility\n\n## Decision Drivers\n- Reviewers need the selected option and rationale without consulting chat history.\n- The packet must remain critique-first when authored context is weak.\n\n## Options Considered\n- Keep generic architecture summaries and accept the loss of rejected alternatives.\n- Preserve authored decision and option-analysis sections directly in the existing artifacts.\n\n## Pros\n- Reviewers can inspect the chosen and rejected options directly.\n- The packet remains reusable outside the originating conversation.\n\n## Cons\n- Authors must provide richer decision content up front.\n\n## Recommendation\nPreserve authored decision and option-analysis sections in the existing architecture decision artifacts.\n\n## Why Not The Others\n- The generic summary path hides rejected alternatives.\n- A brand new artifact family would widen scope and churn.\n\n## Risks\n- Shared helpers may hide ownership boundaries.\n- Context crossings may look cleaner than they are.\n\n## Bounded Contexts\n- Runtime Governance: owns approvals, gate state, and evidence linkage.\n- Artifact Authoring: owns packet composition and authored-body fidelity.\n\n## Context Relationships\n- Artifact Authoring depends on Runtime Governance for gate outcomes and persisted artifact identity.\n\n## Integration Seams\n- Orchestrator service boundaries separate artifact generation from gate evaluation.\n\n## Anti-Corruption Candidates\n- A narrow renderer-facing contract should shield authored packet structure from orchestration internals.\n\n## Ownership Boundaries\n- Runtime Governance is owned by the execution and policy layer.\n- Artifact Authoring is owned by the markdown rendering and authored-body extraction layer.\n\n## Shared Invariants\n- Published artifacts remain traceable to one run id.\n- Approval-gated work cannot silently skip risk review.\n\n## System Context\n- System: `canon-engine` governs AI-assisted analysis packets for bounded engineering work.\n- External actors:\n  - architect-reviewer: inspects architecture packets and risk posture.\n  - copilot-cli-adapter: generates and critiques bounded packet content.\n\n## Containers\n- `canon-cli` (Rust CLI): starts runs and exposes inspect/approve/status flows.\n- `canon-engine` (Rust library): owns orchestration, gating, and artifact rendering.\n- `.canon/` (local filesystem runtime store): persists manifests, artifacts, and evidence.\n\n## Components\n- `mode_shaping`: drives `system-shaping` and `architecture` execution paths.\n- `gatekeeper`: evaluates gate readiness from artifact contracts and evidence.\n- `markdown renderer`: materializes reviewable markdown artifacts from authored inputs and AI outputs.\n",
    )
    .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::Architecture,
            RiskClass::SystemicImpact,
            UsageZone::Yellow,
            "principal-architect",
            vec!["architecture.md"],
        ))
        .expect("architecture run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert!(summary.approval_targets.contains(&"gate:risk".to_string()));
    assert_eq!(summary.invocations_pending_approval, 0);

    let approval = service
        .approve(
            &summary.run_id,
            "gate:risk",
            "principal-engineer",
            ApprovalDecision::Approve,
            "Systemic architecture analysis may proceed with explicit ownership.",
        )
        .expect("gate approval");
    assert_eq!(approval.state, "Completed");

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Completed");

    let evidence = service
        .inspect(InspectTarget::Evidence { run_id: summary.run_id.clone() })
        .expect("inspect evidence");
    assert_eq!(evidence.entries.len(), 1);
}

#[test]
fn incident_direct_run_produces_artifacts_and_completes_after_risk_approval() {
    let workspace = TempDir::new().expect("temp dir");
    init_change_repo(&workspace);
    fs::write(workspace.path().join("incident.md"), complete_incident_brief()).expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::Incident,
            RiskClass::SystemicImpact,
            UsageZone::Red,
            "incident-commander",
            vec!["incident.md"],
        ))
        .expect("incident run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert_eq!(summary.artifact_count, 6);
    assert!(summary.approval_targets.iter().any(|target| target == "gate:risk"));
    assert_eq!(
        summary.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("incident-frame.md")));
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("containment-plan.md")));

    let approval = service
        .approve(
            &summary.run_id,
            "gate:risk",
            "incident-commander",
            ApprovalDecision::Approve,
            "bounded containment packet accepted for operator review",
        )
        .expect("gate approval");
    assert_eq!(approval.state, "Completed");

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Completed");
    assert_eq!(
        status.mode_result.as_ref().map(|result| result.primary_artifact_title.as_str()),
        Some("Incident Frame")
    );
}

#[test]
fn incident_direct_run_blocks_on_missing_containment_evidence() {
    let workspace = TempDir::new().expect("temp dir");
    init_change_repo(&workspace);
    fs::write(
        workspace.path().join("incident.md"),
        "# Incident Brief\n\nIncident Scope: payments-api only.\nTrigger And Current State: elevated 5xx responses after the last deploy.\nOperational Constraints: no autonomous remediation and no schema changes.\nKnown Facts:\n- errors started after the deploy\nWorking Hypotheses:\n- retry amplification is exhausting the service\nEvidence Gaps:\n- blast radius is still uncertain\nDecision Points:\n- decide whether rollback is still required\nApproved Actions:\n- none yet\nDeferred Actions:\n- schema-level changes remain out of scope\nRelease Readiness:\n- do not advance until containment details are explicit\nFollow-Up Work:\n- add a saturation dashboard and post-incident review item\n",
    )
    .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::Incident,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "incident-commander",
            vec!["incident.md"],
        ))
        .expect("incident run");

    assert_eq!(summary.state, "Blocked");
    assert_eq!(summary.blocking_classification.as_deref(), Some("artifact-blocked"));
    assert_eq!(
        summary.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );
    assert!(summary.blocked_gates.iter().any(|gate| gate.gate == "incident-containment"));
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("blast-radius-map.md")));

    let evidence = service
        .inspect(InspectTarget::Evidence { run_id: summary.run_id.clone() })
        .expect("inspect evidence");
    let evidence_entry = match evidence.entries.first().expect("evidence entry") {
        InspectEntry::Evidence(entry) => entry,
        other => panic!("expected evidence entry, got {other:?}"),
    };
    assert_eq!(evidence_entry.execution_posture.as_deref(), Some("recommendation-only"));

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Blocked");
    assert!(status.blocked_gates.iter().any(|gate| gate.gate == "incident-containment"));
}

#[test]
fn migration_direct_run_produces_artifacts_and_completes_after_risk_approval() {
    let workspace = TempDir::new().expect("temp dir");
    init_change_repo(&workspace);
    fs::write(workspace.path().join("migration.md"), complete_migration_brief())
        .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::Migration,
            RiskClass::SystemicImpact,
            UsageZone::Yellow,
            "migration-lead",
            vec!["migration.md"],
        ))
        .expect("migration run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert_eq!(summary.artifact_count, 6);
    assert!(summary.approval_targets.iter().any(|target| target == "gate:risk"));
    assert_eq!(
        summary.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("source-target-map.md")));
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("fallback-plan.md")));

    let approval = service
        .approve(
            &summary.run_id,
            "gate:risk",
            "migration-lead",
            ApprovalDecision::Approve,
            "bounded compatibility packet accepted for rollout review",
        )
        .expect("gate approval");
    assert_eq!(approval.state, "Completed");

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Completed");
    assert_eq!(
        status.mode_result.as_ref().map(|result| result.primary_artifact_title.as_str()),
        Some("Source-Target Map")
    );
}

#[test]
fn change_direct_run_records_validation_paths_and_runtime_details() {
    let workspace = TempDir::new().expect("temp dir");
    init_change_repo(&workspace);
    fs::write(
        workspace.path().join("change.md"),
        "# Change Brief\n\n## System Slice\n\nauth session boundary and persistence layer.\n\n## Domain Slice\n\nSession lifecycle and cleanup semantics within the auth domain.\n\n## Excluded Areas\n\n- payment settlement\n- billing reports\n\n## Intended Change\n\nAdd bounded repository methods while preserving the public auth contract.\n\n## Legacy Invariants\n\n- session revocation remains eventually consistent\n- audit log ordering stays stable\n\n## Domain Invariants\n\n- a revoked session must never become active again through cleanup retries\n- audit trails must preserve causal order across repository updates\n\n## Forbidden Normalization\n\n- Do not collapse audit-ordering quirks that operators still rely on.\n\n## Change Surface\n\n- session repository\n- auth service\n- token cleanup job\n\n## Ownership\n\n- primary owner: maintainer\n\n## Cross-Context Risks\n\n- cleanup scheduling can leak into notification flows if repository boundaries widen\n\n## Implementation Plan\n\nAdd bounded repository methods and preserve the public auth contract.\n\n## Sequencing\n\n1. Add bounded repository methods.\n2. Switch callers behind the preserved contract.\n\n## Validation Strategy\n\n- contract tests\n- invariant checks\n\n## Independent Checks\n\n- rollback rehearsal by a separate operator\n\n## Decision Record\n\nPrefer additive change over normalization to preserve operator expectations.\n\n## Boundary Tradeoffs\n\n- keep cleanup logic inside the auth boundary even if that duplicates some scheduling code\n\n## Consequences\n\n- preserved surface remains explicit and reviewable\n\n## Unresolved Questions\n\n- should the cleanup job roll out in the same slice?\n\nOwner: maintainer\nRisk Level: bounded-impact\nZone: yellow\n",
    )
    .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::Change,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "maintainer",
            vec!["change.md"],
        ))
        .expect("change run");

    assert_eq!(summary.state, "Completed");
    assert!(summary.invocations_total >= 3);

    let invocations = service
        .inspect(InspectTarget::Invocations { run_id: summary.run_id.clone() })
        .expect("inspect invocations");
    assert!(invocations.entries.iter().any(|entry| match entry {
        InspectEntry::Invocation(summary) => summary.capability == "ValidateWithTool",
        _ => false,
    }));

    let evidence = service
        .inspect(InspectTarget::Evidence { run_id: summary.run_id.clone() })
        .expect("inspect evidence");
    let evidence_entry = match evidence.entries.first().expect("evidence entry") {
        InspectEntry::Evidence(entry) => entry,
        other => panic!("expected evidence entry, got {other:?}"),
    };
    assert!(!evidence_entry.validation_paths.is_empty());

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Completed");
    assert!(status.validation_independence_satisfied);
}

#[test]
fn backlog_direct_run_emits_full_packet_and_persists_planning_context() {
    let workspace = TempDir::new().expect("temp dir");
    init_change_repo(&workspace);
    fs::write(workspace.path().join("backlog.md"), complete_backlog_brief()).expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::Backlog,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "planner",
            vec!["backlog.md"],
        ))
        .expect("backlog run");

    assert_eq!(summary.state, "Completed");
    assert_eq!(summary.invocations_total, 4);
    assert_eq!(summary.artifact_count, 8);
    assert!(summary.approval_targets.is_empty());
    assert!(
        summary.mode_result.as_ref().is_some_and(|result| {
            result.primary_artifact_path.ends_with("backlog-overview.md")
        })
    );
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("backlog-overview.md")));
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("planning-risks.md")));

    let artifacts = service
        .inspect(InspectTarget::Artifacts { run_id: summary.run_id.clone() })
        .expect("inspect artifacts");
    let artifact_paths = artifact_names(&artifacts.entries);
    assert_eq!(artifact_paths.len(), 8);
    assert!(artifact_paths.iter().any(|path| path.ends_with("epic-tree.md")));
    assert!(artifact_paths.iter().any(|path| path.ends_with("delivery-slices.md")));

    let evidence = service
        .inspect(InspectTarget::Evidence { run_id: summary.run_id.clone() })
        .expect("inspect evidence");
    let evidence_entry = match evidence.entries.first().expect("evidence entry") {
        InspectEntry::Evidence(entry) => entry,
        other => panic!("expected evidence entry, got {other:?}"),
    };
    assert!(!evidence_entry.generation_paths.is_empty());
    assert!(!evidence_entry.validation_paths.is_empty());
    assert_eq!(evidence_entry.execution_posture, None);
    assert_eq!(evidence_entry.artifact_provenance_links.len(), 8);

    let context_toml = fs::read_to_string(find_run_context_toml(&workspace, &summary.run_id))
        .expect("context.toml");
    assert!(context_toml.contains("[backlog_planning]"));
    assert!(context_toml.contains(
        "delivery_intent = \"Prepare a bounded delivery backlog for auth session hardening.\""
    ));
    assert!(context_toml.contains("planning_horizon = \"next two releases\""));

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Completed");
    assert!(
        status.mode_result.as_ref().is_some_and(|result| {
            result.primary_artifact_path.ends_with("backlog-overview.md")
        })
    );
}

#[test]
fn implementation_direct_run_surfaces_recommendation_only_posture_and_bounded_artifacts() {
    let workspace = TempDir::new().expect("temp dir");
    init_change_repo(&workspace);
    fs::write(workspace.path().join("implementation.md"), complete_implementation_brief())
        .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::Implementation,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "maintainer",
            vec!["implementation.md"],
        ))
        .expect("implementation run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert!(summary.approval_targets.iter().any(|target| target == "gate:execution"));
    assert!(summary.invocations_total >= 4);
    assert_eq!(
        summary.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("task-mapping.md")));
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("mutation-bounds.md")));

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "AwaitingApproval");
    assert_eq!(
        status.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );
}

#[test]
fn refactor_direct_run_surfaces_recommendation_only_posture_and_preservation_artifacts() {
    let workspace = TempDir::new().expect("temp dir");
    init_change_repo(&workspace);
    fs::write(workspace.path().join("refactor.md"), complete_refactor_brief()).expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::Refactor,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "maintainer",
            vec!["refactor.md"],
        ))
        .expect("refactor run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert!(summary.approval_targets.iter().any(|target| target == "gate:execution"));
    assert!(summary.invocations_total >= 4);
    assert_eq!(
        summary.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("preserved-behavior.md")));
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("refactor-scope.md")));

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "AwaitingApproval");
    assert_eq!(
        status.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );
}

#[test]
fn pr_review_direct_run_handles_committed_and_worktree_diffs() {
    let workspace = TempDir::new().expect("temp dir");
    init_review_repo(&workspace);
    add_completed_review_diff(&workspace);

    let service = EngineService::new(workspace.path());
    let committed = service
        .run(request(
            Mode::PrReview,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "reviewer",
            vec!["refs/heads/main", "HEAD"],
        ))
        .expect("committed review run");

    assert_eq!(committed.state, "Completed");
    assert_eq!(committed.artifact_count, 8);
    assert!(committed.artifact_paths.iter().any(|path| path.ends_with("pr-analysis.md")));

    let committed_status = service.status(&committed.run_id).expect("committed status");
    assert_eq!(committed_status.state, "Completed");

    fs::write(
        workspace.path().join("src/reviewer.rs"),
        "pub fn format_review(label: &str) -> String {\n    format!(\"review:{}\", label.to_uppercase())\n}\n",
    )
    .expect("worktree change");

    let worktree = service
        .run(request(
            Mode::PrReview,
            RiskClass::LowImpact,
            UsageZone::Green,
            "reviewer",
            vec!["refs/heads/main", "WORKTREE"],
        ))
        .expect("worktree review run");

    assert_eq!(worktree.state, "Completed");
    assert!(worktree.artifact_paths.iter().any(|path| path.ends_with("review-summary.md")));
}

#[test]
fn shell_adapter_reports_worktree_diff_and_enforces_mutation_policy() {
    let workspace = TempDir::new().expect("temp dir");
    git(&workspace, &["init", "-b", "main"]);
    git(&workspace, &["config", "user.name", "Canon Test"]);
    git(&workspace, &["config", "user.email", "canon@example.com"]);
    fs::write(workspace.path().join("notes.txt"), "base\n").expect("notes file");
    git(&workspace, &["add", "."]);
    git(&workspace, &["commit", "-m", "base notes"]);
    fs::write(workspace.path().join("notes.txt"), "base\nupdated\n").expect("updated notes");

    let shell = ShellAdapter;
    let read_request = shell.read_only_request("inspect worktree status");
    assert_eq!(read_request.capability, CapabilityKind::RunCommand);
    assert_eq!(read_request.orientation, Some(InvocationOrientation::Context));
    assert_eq!(read_request.side_effect, SideEffectClass::ReadOnly);
    assert_eq!(read_request.trust_boundary, Some(TrustBoundaryKind::LocalProcess));

    let status = shell
        .run(&read_request, "git", &["status", "--porcelain"], Some(workspace.path()), false)
        .expect("git status");
    assert_eq!(status.status_code, 0);
    assert!(status.stdout.contains("notes.txt"));

    assert!(shell.has_uncommitted_changes(workspace.path()).expect("uncommitted changes"));

    let diff = shell.git_diff_worktree("refs/heads/main", workspace.path()).expect("worktree diff");
    assert_eq!(diff.head_ref, "WORKTREE");
    assert!(diff.changed_files.contains(&"notes.txt".to_string()));
    assert!(diff.patch.contains("updated"));

    let validation_request = shell.validation_request("validate repository visibility");
    assert_eq!(validation_request.capability, CapabilityKind::ValidateWithTool);

    let mutation_request = shell.mutating_request("touch a file");
    let error = shell
        .run(&mutation_request, "git", &["status"], Some(workspace.path()), false)
        .expect_err("mutating request should be blocked when mutation is disallowed");
    assert!(matches!(error, AdapterError::MutationBlocked));
}

#[test]
fn artifact_contract_helpers_cover_analysis_profiles_and_validation_failures() {
    for mode in [
        Mode::Discovery,
        Mode::SystemShaping,
        Mode::Architecture,
        Mode::Backlog,
        Mode::Implementation,
    ] {
        let contract = contract_for_mode(mode);
        assert!(!contract.artifact_requirements.is_empty());

        let complete_bundle = contract
            .artifact_requirements
            .iter()
            .map(|requirement| {
                let contents = requirement
                    .required_sections
                    .iter()
                    .map(|section| format!("## {section}\n\nRecorded content for {section}."))
                    .collect::<Vec<_>>()
                    .join("\n\n");
                (requirement.file_name.clone(), contents)
            })
            .collect::<Vec<_>>();

        assert!(validate_release_bundle(&contract, &complete_bundle).is_empty());

        let first_requirement = contract.artifact_requirements.first().expect("first requirement");
        let incomplete_contents = if first_requirement.required_sections.len() == 1 {
            "No required headings are present.".to_string()
        } else {
            format!("## {}\n\nOnly one section is present.", first_requirement.required_sections[0])
        };
        let blockers = validate_artifact(first_requirement, &incomplete_contents);
        assert!(!blockers.is_empty());

        let missing_bundle = complete_bundle.into_iter().skip(1).collect::<Vec<_>>();
        let bundle_blockers = validate_release_bundle(&contract, &missing_bundle);
        assert!(
            bundle_blockers.iter().any(|blocker| blocker.contains(&first_requirement.file_name))
        );
    }
}

#[test]
fn implementation_direct_run_completes_via_approve_and_resume() {
    let workspace = TempDir::new().expect("temp dir");
    init_change_repo(&workspace);
    fs::write(workspace.path().join("implementation.md"), complete_implementation_brief())
        .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::Implementation,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "maintainer",
            vec!["implementation.md"],
        ))
        .expect("implementation run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert!(summary.approval_targets.iter().any(|t| t == "gate:execution"));

    let approved = service
        .approve(
            &summary.run_id,
            "gate:execution",
            "maintainer",
            ApprovalDecision::Approve,
            "Bounded implementation approved after packet review.",
        )
        .expect("approve gate:execution");
    assert_eq!(approved.state, "AwaitingApproval");

    let post_approve_status = service.status(&summary.run_id).expect("status after approve");
    assert_eq!(post_approve_status.state, "AwaitingApproval");
    assert!(post_approve_status.approval_targets.is_empty());

    let resumed = service.resume(&summary.run_id).expect("resume implementation run");
    assert_eq!(resumed.state, "Completed");
    assert_eq!(
        resumed.mode_result.as_ref().and_then(|r| r.execution_posture.as_deref()),
        Some("approved-recommendation")
    );
}

#[test]
fn refactor_direct_run_completes_via_approve_and_resume() {
    let workspace = TempDir::new().expect("temp dir");
    init_change_repo(&workspace);
    fs::write(workspace.path().join("refactor.md"), complete_refactor_brief()).expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::Refactor,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "maintainer",
            vec!["refactor.md"],
        ))
        .expect("refactor run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert!(summary.approval_targets.iter().any(|t| t == "gate:execution"));

    let approved = service
        .approve(
            &summary.run_id,
            "gate:execution",
            "maintainer",
            ApprovalDecision::Approve,
            "Bounded refactor approved after packet review.",
        )
        .expect("approve gate:execution");
    assert_eq!(approved.state, "AwaitingApproval");

    let post_approve_status = service.status(&summary.run_id).expect("status after approve");
    assert_eq!(post_approve_status.state, "AwaitingApproval");
    assert!(post_approve_status.approval_targets.is_empty());

    let resumed = service.resume(&summary.run_id).expect("resume refactor run");
    assert_eq!(resumed.state, "Completed");
    assert_eq!(
        resumed.mode_result.as_ref().and_then(|r| r.execution_posture.as_deref()),
        Some("approved-recommendation")
    );
}
