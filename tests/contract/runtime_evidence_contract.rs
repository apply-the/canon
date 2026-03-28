use std::fs;

use assert_cmd::Command;
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
fn requirements_run_persists_invocation_manifests_and_run_evidence_bundle() {
    let workspace = TempDir::new().expect("temp dir");
    let idea_path = workspace.path().join("idea.md");
    fs::write(&idea_path, "# Idea\n\nGovern external execution before artifacts.\n")
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
    let run_root = workspace.path().join(".canon").join("runs").join(run_id);

    assert!(run_root.join("evidence.toml").exists(), "run-level evidence should exist");

    let invocations_dir = run_root.join("invocations");
    assert!(invocations_dir.is_dir(), "invocations directory should exist");

    let first_request = fs::read_dir(&invocations_dir)
        .expect("invocation dir")
        .next()
        .expect("at least one invocation")
        .expect("dir entry")
        .path();

    assert!(first_request.join("request.toml").exists(), "request manifest should exist");
    assert!(first_request.join("decision.toml").exists(), "decision manifest should exist");
    assert!(first_request.join("attempt-01.toml").exists(), "attempt manifest should exist");
}
