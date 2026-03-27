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

fn blocked_brief() -> &'static str {
    "# Brownfield Brief\n\nSystem Slice: auth session boundary and persistence layer.\nImplementation Plan: keep the external auth API stable while tightening the persistence boundary.\n"
}

fn complete_brief() -> &'static str {
    "# Brownfield Brief\n\nSystem Slice: auth session boundary and persistence layer.\nLegacy Invariants: session revocation remains eventually consistent and audit log ordering stays stable.\nChange Surface: session repository, auth service, and token cleanup job.\nImplementation Plan: add bounded repository methods and preserve the public auth contract.\nValidation Strategy: contract tests, invariant checks, and rollback rehearsal.\nDecision Record: prefer additive change over normalization to preserve operator expectations.\n"
}

#[test]
fn run_brownfield_change_blocks_when_preservation_artifacts_are_incomplete() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("brownfield.md");
    fs::write(&brief_path, blocked_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "brownfield-change",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
            "--input",
            brief_path.file_name().expect("file name").to_str().expect("utf8"),
            "--output",
            "json",
        ])
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).expect("utf8 stdout");
    let json: serde_json::Value = serde_json::from_str(&text).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");

    let run_root = workspace.path().join(".canon").join("runs").join(run_id);
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("brownfield-change");

    assert_eq!(json["state"], "Blocked");
    assert!(run_root.join("run.toml").exists(), "run manifest should exist");
    assert!(run_root.join("artifact-contract.toml").exists(), "artifact contract should exist");
    assert!(
        run_root.join("gates").join("brownfield-preservation.toml").exists(),
        "brownfield preservation gate should be persisted"
    );
    assert!(
        artifact_root.join("legacy-invariants.md").exists(),
        "legacy invariants artifact should exist"
    );
    assert!(
        artifact_root.join("change-surface.md").exists(),
        "change surface artifact should exist"
    );

    let status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_json: serde_json::Value =
        serde_json::from_slice(&status_output).expect("status json output");
    assert_eq!(status_json["state"], "Blocked");
}

#[test]
fn run_brownfield_change_completes_when_context_is_fully_described() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("brownfield.md");
    fs::write(&brief_path, complete_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "brownfield-change",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
            "--input",
            brief_path.file_name().expect("file name").to_str().expect("utf8"),
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

    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("brownfield-change");

    assert_eq!(json["state"], "Completed");

    for artifact in [
        "system-slice.md",
        "legacy-invariants.md",
        "change-surface.md",
        "implementation-plan.md",
        "validation-strategy.md",
        "decision-record.md",
    ] {
        assert!(
            artifact_root.join(artifact).exists(),
            "{artifact} should exist in the brownfield bundle"
        );
    }
}
