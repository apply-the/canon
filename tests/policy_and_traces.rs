use std::fs;

use canon_engine::EngineService;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::ClassificationProvenance;
use canon_engine::orchestrator::service::RunRequest;
use canon_engine::persistence::store::WorkspaceStore;
use tempfile::TempDir;

fn init_implementation_repo(workspace: &TempDir) {
    let git = |args: &[&str]| {
        let output = std::process::Command::new("git")
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
    };

    git(&["init", "-b", "main"]);
    git(&["config", "user.name", "Canon Test"]);
    git(&["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("src/auth")).expect("src dir");
    fs::write(
        workspace.path().join("src/auth/session.rs"),
        "pub fn revoke_session(id: &str) -> String {\n    format!(\"revoked:{id}\")\n}\n",
    )
    .expect("source file");

    git(&["add", "."]);
    git(&["commit", "-m", "seed implementation repo"]);
}

fn incomplete_migration_brief() -> &'static str {
    "# Migration Brief\n\nCurrent State: auth-v1 serves login and token refresh traffic.\nTarget State: auth-v2 serves the same bounded traffic surface.\nTransition Boundaries: login and token refresh only.\nGuaranteed Compatibility:\n- existing tokens continue to validate\nTemporary Incompatibilities:\n- admin reporting stays on v1 during the rollout\nCoexistence Rules:\n- dual-write session metadata during cutover\nOrdered Steps:\n- enable shadow reads\n- start dual-write\n- cut traffic to auth-v2\nParallelizable Work:\n- docs and dashboards can update in parallel\nCutover Criteria:\n- error rate and token validation remain stable\nVerification Checks:\n- login and token validation pass against auth-v2\nResidual Risks:\n- admin reporting remains temporarily inconsistent\nRelease Readiness:\n- fallback credibility is not yet established\nMigration Decisions:\n- retain dual-write during the bounded cutover\nDeferred Decisions:\n- move admin reporting after the bounded migration completes\nApproval Notes:\n- explicit migration-lead sign-off is required before broader rollout\n"
}

#[test]
fn load_policy_set_merges_known_local_overrides() {
    let workspace = TempDir::new().expect("temp dir");
    let store = WorkspaceStore::new(workspace.path());
    store.init_runtime_state(None).expect("runtime state");

    let override_root = workspace.path().join("policy-overrides");
    fs::create_dir_all(&override_root).expect("override root");
    fs::write(
        override_root.join("risk.toml"),
        r#"version = 1

[[class]]
name = "BoundedImpact"
requires_owner = true
mutable_execution = false
verification_layers = ["PeerReview"]
"#,
    )
    .expect("risk override");
    fs::write(
        override_root.join("zones.toml"),
        r#"version = 1

[[zone]]
name = "Yellow"
mutable_execution = false
"#,
    )
    .expect("zone override");
    fs::write(
        override_root.join("adapters.toml"),
        r#"version = 2

[[adapter]]
kind = "Filesystem"
capabilities = ["ReadRepository", "ReadArtifact", "EmitArtifact"]

[[adapter]]
kind = "Shell"
capabilities = ["RunCommand", "ValidateWithTool", "ExecuteBoundedTransformation"]

[[adapter]]
kind = "CopilotCli"
capabilities = ["GenerateContent", "CritiqueContent", "ProposeWorkspaceEdit"]

[[adapter]]
kind = "McpStdio"
capabilities = ["InvokeStructuredTool"]

[rules]
block_mutation_for_red_or_systemic = false
runtime_disabled_adapters = []
"#,
    )
    .expect("adapters override");

    let policy = store.load_policy_set(Some(override_root.as_path())).expect("merged policy set");

    assert_eq!(
        policy.verification_layers_for(RiskClass::BoundedImpact),
        vec![canon_engine::domain::verification::VerificationLayer::PeerReview]
    );
    assert!(!policy.allow_mutation(RiskClass::BoundedImpact, UsageZone::Yellow));
    assert!(!policy.block_mutation_for_red_or_systemic);
}

#[test]
fn load_policy_set_fails_closed_on_unknown_override_fields() {
    let workspace = TempDir::new().expect("temp dir");
    let store = WorkspaceStore::new(workspace.path());
    store.init_runtime_state(None).expect("runtime state");

    let override_root = workspace.path().join("bad-policy-overrides");
    fs::create_dir_all(&override_root).expect("override root");
    fs::write(
        override_root.join("gates.toml"),
        r#"version = 1
mandatory_gates = ["Exploration"]
unexpected = true
"#,
    )
    .expect("invalid gate override");

    let error = store
        .load_policy_set(Some(override_root.as_path()))
        .expect_err("unknown fields should fail");
    assert!(error.to_string().contains("unknown field"), "unexpected error: {error}");
}

#[test]
fn requirements_run_persists_a_trace_stream_and_links_it_from_the_run() {
    let workspace = TempDir::new().expect("temp dir");
    let idea_path = workspace.path().join("idea.md");
    fs::write(&idea_path, "# Idea\n\nTrace filesystem activity for governed runs.\n")
        .expect("idea file");

    let service = EngineService::new(workspace.path());
    service.init(None).expect("init");
    let summary = service
        .run(RunRequest {
            mode: Mode::Requirements,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: None,
            classification: ClassificationProvenance::explicit(),
            owner: "product-lead".to_string(),
            inputs: vec!["idea.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        })
        .expect("requirements run");

    let trace_path =
        workspace.path().join(".canon").join("traces").join(format!("{}.jsonl", summary.run_id));
    assert!(trace_path.exists(), "trace stream should exist");

    let trace_contents = fs::read_to_string(&trace_path).expect("trace contents");
    assert!(
        trace_contents.contains("\"adapter\":\"Filesystem\""),
        "trace stream should record filesystem adapter invocations"
    );
    assert!(
        trace_contents.contains("\"capability\":\"EmitArtifact\""),
        "trace stream should record write capability usage"
    );

    let links = fs::read_to_string(
        canon_engine::persistence::layout::ProjectLayout::new(workspace.path())
            .run_dir(&summary.run_id)
            .join("links.toml"),
    )
    .expect("links");
    assert!(
        links.contains(&format!("traces/{}.jsonl", summary.run_id)),
        "run links should reference the trace stream"
    );
}

#[test]
fn downgraded_backlog_run_persists_trace_stream_and_risk_only_evidence_refs() {
    let workspace = TempDir::new().expect("temp dir");
    init_implementation_repo(&workspace);

    let packet_root = workspace.path().join("canon-input").join("backlog");
    fs::create_dir_all(&packet_root).expect("packet root");
    fs::write(
        packet_root.join("brief.md"),
        "# Backlog Brief\n\n## Delivery Intent\nPrepare a bounded delivery backlog for auth session hardening.\n\n## Desired Granularity\nepic-plus-slice\n\n## Planning Horizon\nnext two releases\n\n## Source References\n- docs/changes/auth-session.md\n- docs/architecture/auth-boundary.md\n\n## Constraints\n- Keep the output above task level.\n",
    )
    .expect("brief file");

    let service = EngineService::new(workspace.path());
    service.init(None).expect("init");
    let summary = service
        .run(RunRequest {
            mode: Mode::Backlog,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(canon_engine::domain::run::SystemContext::Existing),
            classification: ClassificationProvenance::explicit(),
            owner: "planner".to_string(),
            inputs: vec!["canon-input/backlog".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        })
        .expect("backlog run");

    assert_eq!(summary.state, "Completed");
    assert_eq!(summary.artifact_count, 2);

    let trace_path =
        workspace.path().join(".canon").join("traces").join(format!("{}.jsonl", summary.run_id));
    assert!(trace_path.exists(), "trace stream should exist");

    let trace_contents = fs::read_to_string(&trace_path).expect("trace contents");
    assert!(
        trace_contents.contains("\"capability\":\"EmitArtifact\""),
        "trace stream should record artifact writes for downgraded backlog runs"
    );

    let evidence = WorkspaceStore::new(workspace.path())
        .load_evidence_bundle(&summary.run_id)
        .expect("evidence bundle load")
        .expect("evidence bundle");
    assert_eq!(evidence.artifact_refs.len(), 2);
    assert!(
        evidence.artifact_refs.iter().all(
            |path| path.ends_with("backlog-overview.md") || path.ends_with("planning-risks.md")
        ),
        "downgraded backlog evidence should only reference the risk-only packet: {:?}",
        evidence.artifact_refs
    );
}

#[test]
fn implementation_run_persists_recommendation_only_mutation_traces() {
    let workspace = TempDir::new().expect("temp dir");
    init_implementation_repo(&workspace);
    fs::write(
        workspace.path().join("implementation.md"),
        "# Implementation Brief\n\nFeature Slice: Auth session revocation repository wiring inside the existing login subsystem.\nPrimary Upstream Mode: change\n\n## Task Mapping\n1. Add bounded auth session repository helpers.\n2. Thread the helper through the revocation service without expanding the public API.\n3. Record implementation notes for operator review and rollback.\n\n## Bounded Changes\n- Auth session repository helper wiring.\n- Revocation service internal composition.\n\n## Mutation Bounds\nsrc/auth/session.rs and src/auth/repository.rs only.\n\n## Allowed Paths\n- src/auth/session.rs\n- src/auth/repository.rs\n\n## Executed Changes\n- Add the bounded repository helper and thread it through the revocation service without widening the public API.\n\n## Task Linkage\n- Step 1 adds the helper.\n- Step 2 rewires the service behind the existing external contract.\n- Step 3 records the resulting packet and rollback posture.\n\n## Completion Evidence\n- The emitted implementation packet and focused tests confirm the bounded slice is ready for operator review.\n\n## Remaining Risks\n- Repository wiring could still drift into adjacent auth modules if the bounded paths expand during review.\n\n## Safety-Net Evidence\nContract coverage protects revocation formatting and audit ordering before mutation.\n\n## Independent Checks\n- cargo test --test session_contract\n- cargo test --test auth_audit_ordering\n\n## Rollback Triggers\nRevocation output drifts, audit ordering becomes unstable, or repository wiring expands beyond the declared auth-session slice.\n\n## Rollback Steps\n1. Revert the bounded auth-session patch.\n2. Redeploy the previous build.\n3. Restore the last known-good audit ordering snapshot.\n",
    )
    .expect("brief file");

    let service = EngineService::new(workspace.path());
    service.init(None).expect("init");
    let summary = service
        .run(RunRequest {
            mode: Mode::Implementation,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(canon_engine::domain::run::SystemContext::Existing),
            classification: ClassificationProvenance::explicit(),
            owner: "maintainer".to_string(),
            inputs: vec!["implementation.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        })
        .expect("implementation run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert_eq!(
        summary.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );
    assert!(summary.approval_targets.iter().any(|target| target == "gate:execution"));

    let trace_path =
        workspace.path().join(".canon").join("traces").join(format!("{}.jsonl", summary.run_id));
    assert!(trace_path.exists(), "trace stream should exist");

    let trace_contents = fs::read_to_string(&trace_path).expect("trace contents");
    assert!(
        trace_contents.contains("\"capability\":\"ExecuteBoundedTransformation\""),
        "trace stream should record bounded transformation invocations"
    );
    assert!(
        trace_contents.contains("\"outcome\":\"RecommendationOnly\""),
        "trace stream should persist recommendation-only mutation outcomes"
    );
}

#[test]
fn red_zone_refactor_run_persists_recommendation_only_mutation_traces() {
    let workspace = TempDir::new().expect("temp dir");
    init_implementation_repo(&workspace);
    fs::write(
        workspace.path().join("refactor.md"),
        "# Refactor Brief\n\nFeature Slice: Auth session boundary and repository composition inside the existing login subsystem.\nPrimary Upstream Mode: implementation\n\n## Preserved Behavior\nSession revocation formatting and audit ordering remain externally unchanged.\n\n## Approved Exceptions\nNone.\n\n## Refactor Scope\nAuth session boundary and repository composition only.\n\n## Allowed Paths\n- src/auth/session.rs\n- src/auth/repository.rs\n\n## Structural Rationale\nIsolate persistence concerns and internal composition without changing externally meaningful behavior.\n\n## Untouched Surface\nPublic auth API, tests/session.md, deployment wiring, and analytics consumers stay unchanged.\n\n## Safety-Net Evidence\nContract coverage protects revocation formatting and audit ordering before structural cleanup.\n\n## Regression Findings\nNo regression findings are accepted in this bounded packet.\n\n## Contract Drift\nNo public contract drift is allowed.\n\n## Reviewer Notes\nReviewer confirmation is required before any drift or feature semantics are accepted.\n\n## Feature Audit\nNo new feature behavior is introduced in this refactor packet.\n\n## Decision\nPreserve behavior and stop immediately if the surface expands or the packet starts to add feature semantics.\n",
    )
    .expect("brief file");

    let service = EngineService::new(workspace.path());
    service.init(None).expect("init");
    let summary = service
        .run(RunRequest {
            mode: Mode::Refactor,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Red,
            system_context: Some(canon_engine::domain::run::SystemContext::Existing),
            classification: ClassificationProvenance::explicit(),
            owner: "maintainer".to_string(),
            inputs: vec!["refactor.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        })
        .expect("refactor run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert_eq!(
        summary.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );
    assert!(
        summary.approval_targets.iter().any(|target| target == "gate:execution"),
        "approval_targets should contain gate:execution, got: {:?}",
        summary.approval_targets
    );

    let trace_path =
        workspace.path().join(".canon").join("traces").join(format!("{}.jsonl", summary.run_id));
    assert!(trace_path.exists(), "trace stream should exist");

    let trace_contents = fs::read_to_string(&trace_path).expect("trace contents");
    assert!(
        trace_contents.contains("\"capability\":\"ExecuteBoundedTransformation\""),
        "trace stream should record bounded transformation invocations"
    );
    assert!(
        trace_contents.contains("\"outcome\":\"RecommendationOnly\""),
        "trace stream should persist recommendation-only mutation outcomes"
    );
}

#[test]
fn blocked_migration_run_persists_traces_and_operational_status_surfaces() {
    let workspace = TempDir::new().expect("temp dir");
    init_implementation_repo(&workspace);
    fs::write(workspace.path().join("migration.md"), incomplete_migration_brief())
        .expect("brief file");

    let service = EngineService::new(workspace.path());
    service.init(None).expect("init");
    let summary = service
        .run(RunRequest {
            mode: Mode::Migration,
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            system_context: Some(canon_engine::domain::run::SystemContext::Existing),
            classification: ClassificationProvenance::explicit(),
            owner: "migration-lead".to_string(),
            inputs: vec!["migration.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        })
        .expect("migration run");

    assert_eq!(summary.state, "Blocked");
    assert_eq!(
        summary.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );

    let trace_path =
        workspace.path().join(".canon").join("traces").join(format!("{}.jsonl", summary.run_id));
    assert!(trace_path.exists(), "trace stream should exist");

    let trace_contents = fs::read_to_string(&trace_path).expect("trace contents");
    assert!(
        trace_contents.contains("\"capability\":\"EmitArtifact\""),
        "trace stream should record artifact writes for blocked migration runs"
    );

    let evidence = service
        .inspect(canon_engine::orchestrator::service::InspectTarget::Evidence {
            run_id: summary.run_id.clone(),
        })
        .expect("inspect evidence");
    let evidence_entry = match evidence.entries.first().expect("evidence entry") {
        canon_engine::orchestrator::service::InspectEntry::Evidence(entry) => entry,
        other => panic!("expected evidence entry, got {other:?}"),
    };
    assert_eq!(evidence_entry.execution_posture.as_deref(), Some("recommendation-only"));
    assert_eq!(evidence_entry.artifact_provenance_links.len(), 6);

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Blocked");
    assert_eq!(
        status.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );
    assert!(status.blocked_gates.iter().any(|gate| gate.gate == "migration-safety"));
}
