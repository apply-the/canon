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

fn complete_requirements_brief(problem: &str, outcome: &str) -> String {
    format!(
        "# Requirements Brief\n\n## Problem\n\n{problem}\n\n## Outcome\n\n{outcome}\n\n## Constraints\n\n- Keep evidence local\n- Preserve approvals\n\n## Non-Negotiables\n\n- Human ownership remains explicit\n- Artifacts remain inspectable\n\n## Options\n\n1. Keep execution governed.\n2. Defer renderer-only shortcuts.\n\n## Recommended Path\n\nKeep execution governed.\n\n## Tradeoffs\n\n- Governance adds steps\n\n## Consequences\n\n- Reviewers can inspect the packet honestly.\n\n## Out of Scope\n\n- No ungoverned mutation path\n\n## Deferred Work\n\n- Automation breadth can expand later.\n\n## Decision Checklist\n\n- [x] Governing owner is explicit\n\n## Open Questions\n\n- Which downstream mode consumes this packet first?\n"
    )
}

#[test]
fn bounded_requirements_run_reports_governed_invocation_counts_and_evidence() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("idea.md"),
        complete_requirements_brief(
            "Use governed execution instead of renderer-only output.",
            "Operators can review governed artifacts before follow-on planning.",
        ),
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
        complete_requirements_brief(
            "Systemic requirements framing still needs governed approval.",
            "The governed packet remains reviewable after approval and resume.",
        ),
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
