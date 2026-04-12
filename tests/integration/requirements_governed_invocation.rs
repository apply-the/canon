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
fn bounded_requirements_run_reports_governed_invocation_counts_and_evidence() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("idea.md"),
        "# Idea\n\nUse governed execution instead of renderer-only output.\n",
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

    assert_eq!(json["invocations_total"], 4);
    assert_eq!(json["invocations_denied"], 1);
    assert_eq!(json["invocations_pending_approval"], 0);
    assert!(
        json.get("evidence_bundle").is_none(),
        "run JSON should not expose internal evidence bundle paths"
    );

    let invocations = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "invocations", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let invocation_json: serde_json::Value = serde_json::from_slice(&invocations).expect("json");
    let entries = invocation_json["entries"].as_array().expect("entries");
    assert_eq!(entries.len(), 4, "requirements should persist 4 governed requests");
    assert!(
        entries.iter().any(|entry| entry["policy_decision"] == "Deny"
            && entry["capability"] == "ProposeWorkspaceEdit"),
        "requirements mode should persist the denied workspace-edit request"
    );
}

#[test]
fn systemic_requirements_run_requires_invocation_approval_and_completes_on_resume() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("idea.md"),
        "# Idea\n\nSystemic requirements framing still needs governed approval.\n",
    )
    .expect("idea file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "requirements",
            "--risk",
            "systemic-impact",
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
        .code(3)
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("json");
    let run_id = json["run_id"].as_str().expect("run id");
    assert_eq!(json["state"], "AwaitingApproval");

    let invocations = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "invocations", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let invocation_json: serde_json::Value = serde_json::from_slice(&invocations).expect("json");
    let pending_request_id = invocation_json["entries"]
        .as_array()
        .and_then(|entries| {
            entries.iter().find_map(|entry| {
                if entry["policy_decision"] == "NeedsApproval" {
                    entry["request_id"].as_str().map(ToString::to_string)
                } else {
                    None
                }
            })
        })
        .expect("pending invocation");

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            run_id,
            "--target",
            &format!("invocation:{pending_request_id}"),
            "--by",
            "principal-engineer",
            "--decision",
            "approve",
            "--rationale",
            "Systemic framing may proceed with explicit human ownership.",
        ])
        .assert()
        .success();

    cli_command()
        .current_dir(workspace.path())
        .args(["resume", "--run", run_id])
        .assert()
        .success();

    let status = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_json: serde_json::Value = serde_json::from_slice(&status).expect("json");
    assert_eq!(status_json["state"], "Completed");
    assert_eq!(status_json["pending_invocation_approvals"], 0);
}
