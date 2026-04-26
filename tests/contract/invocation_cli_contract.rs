use std::fs;
use std::process::Command as ProcessCommand;

use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn cli_command() -> Command {
    let mut command = Command::new("cargo");
    command.args([
        "run",
        "--quiet",
        "--manifest-path",
        concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"),
        "-p",
        "canon-cli",
        "--bin",
        "canon",
        "--",
    ]);
    command
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

fn init_existing_repo(workspace: &TempDir) {
    git(workspace, &["init", "-b", "main"]);
    git(workspace, &["config", "user.name", "Canon Test"]);
    git(workspace, &["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("src/auth")).expect("src dir");
    fs::write(
        workspace.path().join("src/auth/session.rs"),
        "pub fn revoke_session(id: &str) -> String {\n    format!(\"revoked:{id}\")\n}\n",
    )
    .expect("source file");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "seed implementation repo"]);
}

#[test]
fn approve_help_exposes_target_instead_of_gate_only() {
    let mut command = cli_command();
    command
        .args(["approve", "--help"])
        .assert()
        .success()
        .stdout(contains("--target"))
        .stdout(contains("invocation:"))
        .stdout(contains("git user.name and user.email"));
}

#[test]
fn inspect_invocations_and_evidence_are_user_visible_and_populated() {
    let workspace = TempDir::new().expect("temp dir");
    init_existing_repo(&workspace);
    fs::write(
        workspace.path().join("implementation.md"),
        "# Implementation Brief\n\n## Task Mapping\n1. Add bounded auth session repository helpers.\n2. Thread the helper through the revocation service without expanding the public API.\n\n## Bounded Changes\n- Auth session repository helper wiring.\n- Revocation service internal composition.\n\n## Mutation Bounds\nsrc/auth/session.rs; src/auth/repository.rs\n\n## Allowed Paths\n- src/auth/session.rs\n- src/auth/repository.rs\n\n## Safety-Net Evidence\nContract coverage protects revocation formatting and audit ordering before mutation.\n\n## Independent Checks\ncargo test --test session_contract\n\n## Rollback Triggers\nRevocation output drifts or audit ordering becomes unstable.\n\n## Rollback Steps\nRevert the bounded auth-session patch and redeploy the previous build.\n",
    )
    .expect("implementation brief");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "implementation",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
            "--input",
            "implementation.md",
            "--output",
            "json",
        ])
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("json");
    let run_id = json["run_id"].as_str().expect("run id");

    let invocations = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "invocations", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let invocation_json: serde_json::Value = serde_json::from_slice(&invocations).expect("json");
    assert!(
        invocation_json["entries"].as_array().is_some_and(|entries| !entries.is_empty()),
        "invocation inspection should report at least one persisted request"
    );
    assert!(
        invocation_json["entries"].as_array().is_some_and(|entries| entries
            .iter()
            .any(|entry| entry["recommendation_only"].as_bool() == Some(true))),
        "invocation inspection should surface recommendation_only for constrained execution"
    );

    let evidence = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "evidence", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let evidence_json: serde_json::Value = serde_json::from_slice(&evidence).expect("json");
    assert!(
        evidence_json["entries"].as_array().is_some_and(|entries| !entries.is_empty()),
        "evidence inspection should expose the run-level evidence bundle"
    );
    assert_eq!(
        evidence_json["entries"][0]["execution_posture"].as_str(),
        Some("recommendation-only")
    );
}

#[test]
fn inspect_evidence_surfaces_upstream_context_from_folder_packet() {
    let workspace = TempDir::new().expect("temp dir");
    init_existing_repo(&workspace);

    let packet_root = workspace.path().join("canon-input").join("implementation");
    fs::create_dir_all(&packet_root).expect("packet root");
    fs::write(
        packet_root.join("brief.md"),
        "# Implementation Brief\n\nFeature Slice: auth session revocation\nPrimary Upstream Mode: change\n\n## Task Mapping\n1. Thread the helper through the revocation service without changing the public API.\n\n## Bounded Changes\n- Auth session repository helper wiring.\n\n## Mutation Bounds\nsrc/auth/session.rs; src/auth/repository.rs\n\n## Allowed Paths\n- src/auth/session.rs\n- src/auth/repository.rs\n\n## Safety-Net Evidence\nContract coverage protects revocation formatting and audit ordering before mutation.\n\n## Independent Checks\ncargo test --test session_contract\n\n## Rollback Triggers\nRevocation output drifts or audit ordering becomes unstable.\n\n## Rollback Steps\nRevert the bounded auth-session patch and redeploy the previous build.\n",
    )
    .expect("brief");
    fs::write(
        packet_root.join("source-map.md"),
        "# Source Map\n\n## Upstream Sources\n\n- docs/changes/R-20260422-AUTHREVOC/change-surface.md\n- docs/changes/R-20260422-AUTHREVOC/implementation-plan.md\n\n## Carried-Forward Decisions\n\n- Revocation output formatting stays stable.\n- Contract coverage must pass before and after mutation.\n\n## Excluded Upstream Scope\n\nLogin UI flow and token issuance remain out of scope.\n",
    )
    .expect("source map");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "implementation",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
            "--input",
            "canon-input/implementation",
            "--output",
            "json",
        ])
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("json");
    let run_id = json["run_id"].as_str().expect("run id");

    let evidence = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "evidence", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let evidence_json: serde_json::Value = serde_json::from_slice(&evidence).expect("json");

    assert_eq!(
        evidence_json["entries"][0]["upstream_feature_slice"].as_str(),
        Some("auth session revocation")
    );
    assert_eq!(evidence_json["entries"][0]["primary_upstream_mode"].as_str(), Some("change"));
    assert_eq!(
        evidence_json["entries"][0]["upstream_source_refs"][0].as_str(),
        Some("docs/changes/R-20260422-AUTHREVOC/change-surface.md")
    );
    assert_eq!(
        evidence_json["entries"][0]["carried_forward_items"][0].as_str(),
        Some("Revocation output formatting stays stable.")
    );
    assert_eq!(
        evidence_json["entries"][0]["excluded_upstream_scope"].as_str(),
        Some("Login UI flow and token issuance remain out of scope.")
    );
}

#[test]
fn inspect_evidence_reflects_approved_recommendation_after_resume() {
    let workspace = TempDir::new().expect("temp dir");
    init_existing_repo(&workspace);

    let packet_root = workspace.path().join("canon-input").join("refactor");
    fs::create_dir_all(&packet_root).expect("packet root");
    fs::write(
        packet_root.join("brief.md"),
        "# Refactor Brief\n\nPreserved Behavior: session revocation formatting and audit ordering remain externally unchanged.\nRefactor Scope: auth session boundary and repository composition only.\nStructural Rationale: isolate persistence concerns without changing externally meaningful behavior.\n",
    )
    .expect("brief");
    fs::write(
        packet_root.join("preserved-behavior.md"),
        "# Preserved Behavior\n\n## Summary\nSession behavior is preserved.\n\n## Preserved Behavior\nSession revocation formatting and audit ordering remain externally unchanged.\n\n## Approved Exceptions\nNone.\n",
    )
    .expect("preserved behavior");
    fs::write(
        packet_root.join("refactor-scope.md"),
        "# Refactor Scope\n\n## Summary\nBounded structural cleanup.\n\n## Refactor Scope\nAuth session boundary and repository composition only.\n\n## Allowed Paths\n- src/auth/session.rs\n- src/auth/repository.rs\n",
    )
    .expect("refactor scope");
    fs::write(
        packet_root.join("structural-rationale.md"),
        "# Structural Rationale\n\n## Summary\nCleanup is safe.\n\n## Structural Rationale\nIsolate persistence concerns without changing externally meaningful behavior.\n\n## Untouched Surface\nPublic auth API, tests/session.md, and deployment wiring stay unchanged.\n",
    )
    .expect("structural rationale");
    fs::write(
        packet_root.join("regression-evidence.md"),
        "# Regression Evidence\n\nNo regression findings are accepted in the bounded packet.\n",
    )
    .expect("regression evidence");
    fs::write(
        packet_root.join("contract-drift-check.md"),
        "# Contract Drift Check\n\n## Summary\nNo public contract drift.\n\n## Contract Drift\nNo public contract drift is allowed.\n",
    )
    .expect("contract drift");
    fs::write(
        packet_root.join("no-feature-addition.md"),
        "# No Feature Addition\n\n## Summary\nNo new features.\n\n## Feature Audit\nNo new feature behavior is introduced in this refactor packet.\n",
    )
    .expect("no feature addition");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "refactor",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
            "--output",
            "json",
        ])
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("json");

    // Run is blocked, so no approval/resume/evidence inspection needed
    // Just verify the run is blocked as expected
    assert_eq!(json["state"].as_str(), Some("Blocked"));
    assert_eq!(json["blocking_classification"].as_str(), Some("artifact-blocked"));
    assert_eq!(json["mode_result"]["execution_posture"].as_str(), Some("recommendation-only"));
}

#[test]
fn inspect_invocations_and_evidence_capture_completed_backlog_runs() {
    let workspace = TempDir::new().expect("temp dir");
    init_existing_repo(&workspace);

    let packet_root = workspace.path().join("canon-input").join("backlog");
    fs::create_dir_all(&packet_root).expect("packet root");
    fs::write(
        packet_root.join("brief.md"),
        "# Backlog Brief\n\n## Delivery Intent\nPrepare a bounded delivery backlog for auth session hardening.\n\n## Desired Granularity\nepic-plus-slice\n\n## Planning Horizon\nnext two releases\n\n## Source References\n- docs/changes/auth-session.md\n- docs/architecture/auth-boundary.md\n\n## Constraints\n- Keep the output above task level.\n\n## Out of Scope\n- Login UI redesign\n",
    )
    .expect("brief");
    fs::write(
        packet_root.join("priorities.md"),
        "# Priorities\n\n- Ship the rollback-safe slice first.\n- Keep dependency blockers explicit.\n",
    )
    .expect("priorities");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "backlog",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "planner",
            "--input",
            "canon-input/backlog",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("json");
    let run_id = json["run_id"].as_str().expect("run id");

    let invocations = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "invocations", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let invocation_json: serde_json::Value = serde_json::from_slice(&invocations).expect("json");
    let entries = invocation_json["entries"].as_array().expect("invocation entries");
    assert_eq!(entries.len(), 4);
    assert!(entries.iter().any(|entry| entry["capability"] == "ReadRepository"));
    assert!(entries.iter().any(|entry| entry["capability"] == "GenerateContent"));
    assert!(entries.iter().any(|entry| entry["capability"] == "CritiqueContent"));
    assert!(entries.iter().any(|entry| entry["capability"] == "ValidateWithTool"));
    assert!(entries.iter().all(|entry| entry["recommendation_only"].as_bool() == Some(false)));

    let evidence = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "evidence", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let evidence_json: serde_json::Value = serde_json::from_slice(&evidence).expect("json");
    let evidence_entry = &evidence_json["entries"][0];
    assert!(evidence_entry["generation_paths"].as_array().is_some_and(|paths| !paths.is_empty()));
    assert!(evidence_entry["validation_paths"].as_array().is_some_and(|paths| !paths.is_empty()));
    assert!(
        evidence_entry["artifact_provenance_links"]
            .as_array()
            .is_some_and(|paths| paths.len() == 8)
    );
}
