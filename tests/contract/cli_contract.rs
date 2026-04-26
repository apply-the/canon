use std::fs;
use std::process::Command as ProcessCommand;

use assert_cmd::Command;
use predicates::str::contains;

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

fn git(workspace: &tempfile::TempDir, args: &[&str]) {
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

fn init_existing_repo(workspace: &tempfile::TempDir) {
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
fn help_lists_the_expected_top_level_commands() {
    let mut command = cli_command();
    command
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("init"))
        .stdout(contains("run"))
        .stdout(contains("resume"))
        .stdout(contains("status"))
        .stdout(contains("approve"))
        .stdout(contains("verify"))
        .stdout(contains("inspect"))
        .stdout(contains("publish"))
        .stdout(contains("skills"));
}

#[test]
fn inspect_risk_zone_returns_a_confirmation_payload() {
    let workspace = tempfile::TempDir::new().expect("temp dir");
    std::fs::write(
        workspace.path().join("discovery.md"),
        "# Discovery Brief\n\nProblem: production boundary drift.\nConstraints: preserve repo-local evidence.\n",
    )
    .expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "risk-zone",
            "--mode",
            "discovery",
            "--input",
            "discovery.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let entry = json["entries"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("classification entry");

    assert_eq!(entry["mode"].as_str(), Some("discovery"));
    assert!(entry["risk"].as_str().is_some());
    assert!(entry["zone"].as_str().is_some());
    assert_eq!(entry["requires_confirmation"].as_bool(), Some(true));
    assert!(entry["confidence"].as_str().is_some());
}

#[test]
fn run_rejects_missing_authored_input_for_requirements() {
    let workspace = tempfile::TempDir::new().expect("temp dir");

    cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "requirements",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "product-lead",
        ])
        .assert()
        .failure()
        .stderr(contains("requires at least one authored input via --input or --input-text"));
}

#[test]
fn run_implementation_auto_binds_canonical_input_before_runtime_support_check() {
    let workspace = tempfile::TempDir::new().expect("temp dir");
    init_existing_repo(&workspace);
    std::fs::create_dir_all(workspace.path().join("canon-input")).expect("canon-input dir");
    std::fs::write(
        workspace.path().join("canon-input").join("implementation.md"),
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
            "staff-engineer",
            "--output",
            "json",
        ])
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");

    assert_eq!(json["state"].as_str(), Some("Blocked"));
    assert_eq!(json["mode"].as_str(), Some("implementation"));
    assert_eq!(json["mode_result"]["execution_posture"].as_str(), Some("recommendation-only"));
    assert_eq!(json["blocking_classification"].as_str(), Some("artifact-blocked"));
}

#[test]
fn run_backlog_auto_binds_canonical_input_and_emits_a_planning_packet() {
    let workspace = tempfile::TempDir::new().expect("temp dir");
    init_existing_repo(&workspace);
    std::fs::create_dir_all(workspace.path().join("canon-input")).expect("canon-input dir");
    std::fs::write(
        workspace.path().join("canon-input").join("backlog.md"),
        "# Backlog Brief\n\n## Delivery Intent\nPrepare a bounded delivery backlog for auth session hardening.\n\n## Desired Granularity\nepic-plus-slice\n\n## Planning Horizon\nnext two releases\n\n## Source References\n- docs/changes/auth-session.md\n- docs/architecture/auth-boundary.md\n\n## Priorities\n- Ship the rollback-safe slice first.\n\n## Constraints\n- Keep the output above task level.\n\n## Out of Scope\n- Login UI redesign\n",
    )
    .expect("backlog brief");

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
            "staff-engineer",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");

    assert_eq!(json["state"].as_str(), Some("Completed"));
    assert_eq!(json["mode"].as_str(), Some("backlog"));
    assert!(
        json["mode_result"]["primary_artifact_path"]
            .as_str()
            .is_some_and(|path| path.ends_with("backlog-overview.md"))
    );
}

#[test]
fn inspect_risk_zone_supports_inline_authored_input() {
    let workspace = tempfile::TempDir::new().expect("temp dir");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "risk-zone",
            "--mode",
            "requirements",
            "--input-text",
            "# Requirements Brief\n\n## Problem\nBound runtime governance.\n\n## Constraints\n- Keep evidence local\n- Preserve approvals",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let entry = json["entries"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("classification entry");

    assert_eq!(entry["mode"].as_str(), Some("requirements"));
    assert!(entry["risk"].as_str().is_some());
    assert!(entry["zone"].as_str().is_some());
    assert_eq!(entry["requires_confirmation"].as_bool(), Some(true));
}

#[test]
fn inspect_risk_zone_supports_review_mode_inputs() {
    let workspace = tempfile::TempDir::new().expect("temp dir");
    std::fs::create_dir_all(workspace.path().join("canon-input")).expect("canon-input dir");
    std::fs::write(
        workspace.path().join("canon-input").join("review.md"),
        "# Review Brief\n\nReview Target: bounded service boundary.\nEvidence Basis: owned interfaces and current tests.\n",
    )
    .expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "risk-zone",
            "--mode",
            "review",
            "--input",
            "canon-input/review.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let entry = json["entries"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("classification entry");

    assert_eq!(entry["mode"].as_str(), Some("review"));
    assert!(entry["risk"].as_str().is_some());
    assert!(entry["zone"].as_str().is_some());
    assert_eq!(entry["requires_confirmation"].as_bool(), Some(true));
}

#[test]
fn inspect_risk_zone_rejects_noncanonical_review_inputs() {
    let workspace = tempfile::TempDir::new().expect("temp dir");
    std::fs::write(
        workspace.path().join("review.md"),
        "# Review Brief\n\nReview Target: bounded service boundary.\nEvidence Basis: owned interfaces and current tests.\n",
    )
    .expect("brief file");

    cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "risk-zone", "--mode", "review", "--input", "review.md"])
        .assert()
        .failure()
        .stderr(contains("review accepts only canon-input/review.md or canon-input/review/"));
}

#[test]
fn inspect_risk_zone_supports_verification_mode_inputs() {
    let workspace = tempfile::TempDir::new().expect("temp dir");
    std::fs::write(
        workspace.path().join("verification.md"),
        "# Verification Brief\n\nClaims Under Test: rollback remains bounded and auditable.\nEvidence Basis: repo checks and contract notes.\n",
    )
    .expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "risk-zone",
            "--mode",
            "verification",
            "--input",
            "verification.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let entry = json["entries"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("classification entry");

    assert_eq!(entry["mode"].as_str(), Some("verification"));
    assert!(entry["risk"].as_str().is_some());
    assert!(entry["zone"].as_str().is_some());
    assert_eq!(entry["requires_confirmation"].as_bool(), Some(true));
}
