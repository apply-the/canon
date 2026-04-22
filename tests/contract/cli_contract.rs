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
