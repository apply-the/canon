use std::fs;

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

#[test]
fn approve_help_exposes_target_instead_of_gate_only() {
    let mut command = cli_command();
    command
        .args(["approve", "--help"])
        .assert()
        .success()
        .stdout(contains("--target"))
        .stdout(contains("invocation:"));
}

#[test]
fn inspect_invocations_and_evidence_are_user_visible_and_populated() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(workspace.path().join("idea.md"), "# Idea\n\nInspect governed execution.\n")
        .expect("idea file");

    let output = cli_command()
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
            "--input",
            "idea.md",
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
    assert!(
        invocation_json["entries"].as_array().is_some_and(|entries| !entries.is_empty()),
        "invocation inspection should report at least one persisted request"
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
}
