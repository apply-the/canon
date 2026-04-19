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

fn ready_review_brief() -> &'static str {
    "# Review Brief\n\nReview Target: bounded service boundary package.\nEvidence Basis: owned interfaces, current tests, and rollback notes.\nOwner: reviewer\n"
}

fn gated_review_brief() -> &'static str {
    "# Review Brief\n\nReview Target: release boundary package with must-fix follow-up.\nEvidence Basis: missing evidence remains for rollback rehearsal and sign-off.\nOpen Concern: must-fix disposition is still required before acceptance.\n"
}

fn write_review_brief(workspace: &TempDir, contents: &str) {
    let input_dir = workspace.path().join("canon-input");
    fs::create_dir_all(&input_dir).expect("canon-input dir");
    fs::write(input_dir.join("review.md"), contents).expect("review brief");
}

#[test]
fn review_run_returns_completed_result_for_evidence_bounded_package() {
    let workspace = TempDir::new().expect("temp dir");
    write_review_brief(&workspace, ready_review_brief());

    let run_output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "review",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "reviewer",
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

    let run_json: serde_json::Value = serde_json::from_slice(&run_output).expect("run json");
    let run_id = run_json["run_id"].as_str().expect("run id");
    assert_eq!(run_json["state"], "Completed");
    assert!(run_json["blocking_classification"].is_null());
    assert!(
        run_json["approval_targets"].as_array().is_some_and(|targets| targets.is_empty()),
        "completed review runs should not advertise approval targets"
    );
    assert_eq!(run_json["artifact_count"], 5);
    assert!(
        run_json["artifact_paths"].as_array().is_some_and(|paths| paths.len() == 5),
        "review runs should expose the full review packet"
    );
    assert_eq!(run_json["mode_result"]["primary_artifact_title"], "Review Disposition");
    assert_eq!(
        run_json["mode_result"]["primary_artifact_path"],
        format!(".canon/artifacts/{run_id}/review/review-disposition.md")
    );
    assert!(
        run_json["mode_result"]["headline"]
            .as_str()
            .is_some_and(|headline| headline.contains("downstream inspection"))
    );
    assert!(run_json["recommended_next_action"].is_null());

    let status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_json: serde_json::Value = serde_json::from_slice(&status_output).expect("status");
    assert_eq!(status_json["state"], "Completed");
    assert_eq!(status_json["mode_result"]["primary_artifact_title"], "Review Disposition");
    assert!(status_json["recommended_next_action"].is_null());
}

#[test]
fn review_run_requires_explicit_disposition_for_evidence_gaps() {
    let workspace = TempDir::new().expect("temp dir");
    write_review_brief(&workspace, gated_review_brief());

    let run_output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "review",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "reviewer",
            "--input",
            "canon-input/review.md",
            "--output",
            "json",
        ])
        .assert()
        .code(3)
        .stdout(contains("\"state\": \"AwaitingApproval\""))
        .get_output()
        .stdout
        .clone();

    let run_json: serde_json::Value = serde_json::from_slice(&run_output).expect("run json");
    let run_id = run_json["run_id"].as_str().expect("run id");
    assert_eq!(run_json["state"], "AwaitingApproval");
    assert_eq!(run_json["blocking_classification"], "approval-gated");
    assert_eq!(run_json["mode_result"]["primary_artifact_title"], "Review Disposition");
    assert_eq!(run_json["approval_targets"][0], "gate:review-disposition");
    assert_eq!(run_json["recommended_next_action"]["action"], "inspect-artifacts");
    assert!(
        run_json["artifact_paths"].as_array().is_some_and(|paths| paths.len() == 5),
        "approval-gated review runs should still expose the readable review packet"
    );

    let status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_json: serde_json::Value = serde_json::from_slice(&status_output).expect("status");
    assert_eq!(status_json["state"], "AwaitingApproval");
    assert_eq!(status_json["approval_targets"][0], "gate:review-disposition");
    assert_eq!(status_json["recommended_next_action"]["action"], "inspect-artifacts");

    cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "markdown"])
        .assert()
        .success()
        .stdout(contains("## Result"))
        .stdout(contains("review-disposition.md"))
        .stdout(contains("explicit disposition"));
}

#[test]
fn review_run_rejects_noncanonical_input_paths() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(workspace.path().join("review.md"), ready_review_brief()).expect("brief file");

    cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "review",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "reviewer",
            "--input",
            "review.md",
        ])
        .assert()
        .failure()
        .stderr(contains("review accepts only canon-input/review.md or canon-input/review/"));
}
