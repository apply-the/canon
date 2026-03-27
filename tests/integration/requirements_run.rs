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
fn run_requirements_persists_a_run_contract_and_artifact_bundle() {
    let workspace = TempDir::new().expect("temp dir");
    let idea_path = workspace.path().join("idea.md");
    fs::write(
        &idea_path,
        "# Idea\n\nBound AI-assisted engineering work with explicit governance.\n",
    )
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
            idea_path.file_name().expect("file name").to_str().expect("utf8"),
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).expect("utf8 stdout");
    let json: serde_json::Value = serde_json::from_str(&text).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");

    let run_root = workspace.path().join(".canon").join("runs").join(run_id);
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("requirements");

    assert!(run_root.join("run.toml").exists(), "run manifest should exist");
    assert!(run_root.join("context.toml").exists(), "context file should exist");
    assert!(run_root.join("artifact-contract.toml").exists(), "artifact contract should exist");
    assert!(run_root.join("state.toml").exists(), "state file should exist");
    assert!(run_root.join("links.toml").exists(), "links file should exist");
    assert!(run_root.join("gates").is_dir(), "gates directory should exist");
    assert!(
        run_root.join("gates").join("exploration.toml").exists(),
        "exploration gate should be persisted"
    );
    assert!(run_root.join("gates").join("risk.toml").exists(), "risk gate should be persisted");
    assert!(
        run_root.join("gates").join("release-readiness.toml").exists(),
        "release readiness gate should be persisted"
    );
    assert!(run_root.join("verification").is_dir(), "verification directory should exist");
    assert!(
        run_root.join("verification").join("verification-00.toml").exists(),
        "self-critique verification record should exist"
    );
    assert!(
        run_root.join("verification").join("verification-01.toml").exists(),
        "adversarial verification record should exist"
    );

    for artifact in [
        "problem-statement.md",
        "constraints.md",
        "options.md",
        "tradeoffs.md",
        "scope-cuts.md",
        "decision-checklist.md",
    ] {
        assert!(
            artifact_root.join(artifact).exists(),
            "{artifact} should exist in the requirements bundle"
        );
    }

    let status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_text = String::from_utf8(status_output).expect("utf8 stdout");
    let status_json: serde_json::Value = serde_json::from_str(&status_text).expect("json output");
    assert_eq!(status_json["state"], "Completed");
}
