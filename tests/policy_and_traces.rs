use std::fs;

use canon_engine::EngineService;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::orchestrator::service::RunRequest;
use canon_engine::persistence::store::WorkspaceStore;
use tempfile::TempDir;

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
            owner: "product-lead".to_string(),
            inputs: vec!["idea.md".to_string()],
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
        workspace.path().join(".canon").join("runs").join(&summary.run_id).join("links.toml"),
    )
    .expect("links");
    assert!(
        links.contains(&format!("traces/{}.jsonl", summary.run_id)),
        "run links should reference the trace stream"
    );
}
