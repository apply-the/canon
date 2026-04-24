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
        "# Implementation Brief\n\nTask Mapping: 1. Add bounded auth session repository helpers.\n2. Thread the helper through the revocation service without expanding the public API.\nMutation Bounds: src/auth/session.rs; src/auth/repository.rs\nAllowed Paths:\n- src/auth/session.rs\n- src/auth/repository.rs\nSafety-Net Evidence: contract coverage protects revocation formatting and audit ordering before mutation.\nIndependent Checks: cargo test --test session_contract\nRollback Triggers: revocation output drifts or audit ordering becomes unstable.\nRollback Steps: revert the bounded auth-session patch and redeploy the previous build.\n",
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
        .code(3)
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
        "# Implementation Brief\n\nFeature Slice: auth session revocation\nPrimary Upstream Mode: change\nTask Mapping: 1. Thread the helper through the revocation service without changing the public API.\nMutation Bounds: src/auth/session.rs; src/auth/repository.rs\nAllowed Paths:\n- src/auth/session.rs\n- src/auth/repository.rs\nSafety-Net Evidence: contract coverage protects revocation formatting and audit ordering before mutation.\nIndependent Checks:\n- cargo test --test session_contract\nRollback Triggers: revocation output drifts or audit ordering becomes unstable.\nRollback Steps: revert the bounded auth-session patch and redeploy the previous build.\n",
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
        .code(3)
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
    fs::write(
        workspace.path().join("refactor.md"),
        "# Refactor Brief\n\nPreserved Behavior: session revocation formatting and audit ordering remain externally unchanged.\nApproved Exceptions: none.\nRefactor Scope: auth session boundary and repository composition only.\nAllowed Paths:\n- src/auth/session.rs\n- src/auth/repository.rs\nStructural Rationale: isolate persistence concerns without changing externally meaningful behavior.\nUntouched Surface: public auth API, tests/session.md, and deployment wiring stay unchanged.\nSafety-Net Evidence: contract coverage protects revocation formatting and audit ordering before structural cleanup.\nRegression Findings: no regression findings are accepted in the bounded packet.\nContract Drift: no public contract drift is allowed.\nReviewer Notes: review packet confirms behavior preservation remains explicit.\nFeature Audit: no new feature behavior is introduced in this refactor packet.\nDecision: preserve behavior and stop if the surface expands.\n",
    )
    .expect("refactor brief");

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
            "--input",
            "refactor.md",
            "--output",
            "json",
        ])
        .assert()
        .code(3)
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("json");
    let run_id = json["run_id"].as_str().expect("run id");

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            run_id,
            "--target",
            "gate:execution",
            "--by",
            "maintainer",
            "--decision",
            "approve",
            "--rationale",
            "approved bounded execution",
        ])
        .assert()
        .success();

    cli_command()
        .current_dir(workspace.path())
        .args(["resume", "--run", run_id])
        .assert()
        .success();

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
        evidence_json["entries"][0]["execution_posture"].as_str(),
        Some("approved-recommendation")
    );
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
