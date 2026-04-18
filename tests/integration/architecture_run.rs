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

fn architecture_brief() -> &'static str {
    "# Architecture Brief\n\nDecision focus: map boundaries and tradeoffs for governed analysis-mode expansion.\nConstraint: preserve Canon persistence, evidence, and approval behavior.\n"
}

#[test]
fn run_architecture_persists_a_completed_run_and_artifact_bundle() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("architecture.md");
    fs::write(&brief_path, architecture_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "architecture",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "staff-architect",
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

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("architecture");

    assert_eq!(json["state"], "Completed");
    assert_eq!(json["invocations_total"], 3);
    assert_eq!(json["invocations_pending_approval"], 0);
    assert_eq!(
        json["mode_result"]["primary_artifact_title"].as_str(),
        Some("Architecture Decisions")
    );
    assert!(
        json["mode_result"]["primary_artifact_path"]
            .as_str()
            .is_some_and(|value| value.ends_with("/architecture/architecture-decisions.md"))
    );

    for artifact in [
        "architecture-decisions.md",
        "invariants.md",
        "tradeoff-matrix.md",
        "boundary-map.md",
        "readiness-assessment.md",
    ] {
        assert!(
            artifact_root.join(artifact).exists(),
            "{artifact} should exist in the architecture bundle"
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
    let status_json: serde_json::Value =
        serde_json::from_slice(&status_output).expect("status json");
    assert_eq!(status_json["state"], "Completed");
    assert_eq!(
        status_json["mode_result"]["primary_artifact_title"].as_str(),
        Some("Architecture Decisions")
    );
}

#[test]
fn architecture_run_enters_awaiting_approval_for_systemic_and_red_zone_cases() {
    for (risk, zone) in [("systemic-impact", "yellow"), ("bounded-impact", "red")] {
        let workspace = TempDir::new().expect("temp dir");
        let brief_path = workspace.path().join("architecture.md");
        fs::write(&brief_path, architecture_brief()).expect("brief file");

        let output = cli_command()
            .current_dir(workspace.path())
            .args([
                "run",
                "--mode",
                "architecture",
                "--risk",
                risk,
                "--zone",
                zone,
                "--owner",
                "staff-architect",
                "--input",
                brief_path.file_name().expect("file name").to_str().expect("utf8"),
                "--output",
                "json",
            ])
            .assert()
            .code(3)
            .get_output()
            .stdout
            .clone();

        let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
        assert_eq!(json["state"], "AwaitingApproval");
        assert_eq!(json["blocking_classification"], "approval-gated");
        assert!(
            json["approval_targets"]
                .as_array()
                .is_some_and(|targets| targets.iter().any(|target| target == "gate:risk")),
            "{risk}/{zone} architecture run should surface gate:risk approval"
        );
    }
}
