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
fn implementation_run_persists_recommendation_only_mutation_traces() {
    let workspace = TempDir::new().expect("temp dir");
    init_implementation_repo(&workspace);
    fs::write(
        workspace.path().join("implementation.md"),
        "# Implementation Brief\n\nTask Mapping: 1. Add bounded auth session repository helpers.\n2. Thread the helper through the revocation service without expanding the public API.\nMutation Bounds: src/auth/session.rs; src/auth/repository.rs\nAllowed Paths:\n- src/auth/session.rs\n- src/auth/repository.rs\nSafety-Net Evidence: contract coverage protects revocation formatting and audit ordering before mutation.\nIndependent Checks: cargo test --test session_contract\nRollback Triggers: revocation output drifts or audit ordering becomes unstable.\nRollback Steps: revert the bounded auth-session patch and redeploy the previous build.\n",
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
        "# Refactor Brief\n\nPreserved Behavior: session revocation formatting and audit ordering remain externally unchanged.\nApproved Exceptions: none.\nRefactor Scope: auth session boundary and repository composition only.\nAllowed Paths:\n- src/auth/session.rs\n- src/auth/repository.rs\nStructural Rationale: isolate persistence concerns without changing externally meaningful behavior.\nUntouched Surface: public auth API, tests/session.md, and deployment wiring stay unchanged.\nSafety-Net Evidence: contract coverage protects revocation formatting and audit ordering before structural cleanup.\nRegression Findings: no regression findings are accepted in the bounded packet.\nContract Drift: no public contract drift is allowed.\nReviewer Notes: review packet confirms behavior preservation remains explicit.\nFeature Audit: no new feature behavior is introduced in this refactor packet.\nDecision: preserve behavior and stop if the surface expands.\n",
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
