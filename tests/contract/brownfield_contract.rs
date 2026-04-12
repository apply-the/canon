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

fn blocked_brief() -> &'static str {
    "# Brownfield Brief\n\nSystem Slice: auth session boundary and persistence layer.\nImplementation Plan: keep the external auth API stable while tightening the persistence boundary.\n"
}

fn complete_brief() -> &'static str {
    "# Brownfield Brief\n\nSystem Slice: auth session boundary and persistence layer.\nLegacy Invariants: session revocation remains eventually consistent and audit log ordering stays stable.\nChange Surface: session repository, auth service, and token cleanup job.\nImplementation Plan: add bounded repository methods and preserve the public auth contract.\nValidation Strategy: contract tests, invariant checks, and rollback rehearsal.\nDecision Record: prefer additive change over normalization to preserve operator expectations.\n"
}

fn parse_run_id(output: &[u8]) -> String {
    let json: serde_json::Value = serde_json::from_slice(output).expect("json output");
    json["run_id"].as_str().expect("run id").to_string()
}

fn pending_request_id(workspace: &TempDir, run_id: &str) -> String {
    let output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "invocations", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    json["entries"]
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
        .expect("pending request")
}

#[test]
fn blocked_brownfield_run_returns_exit_code_2_and_mentions_preservation_gap() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("brownfield.md");
    fs::write(&brief_path, blocked_brief()).expect("brief file");

    let run_output = cli_command()
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
        .stdout(contains("\"state\": \"Blocked\""))
        .get_output()
        .stdout
        .clone();

    let run_json: serde_json::Value = serde_json::from_slice(&run_output).expect("run json");
    let run_id = run_json["run_id"].as_str().expect("run id");
    assert_eq!(run_json["blocking_classification"], "artifact-blocked");
    assert!(
        run_json["approval_targets"].as_array().is_some_and(|targets| targets.is_empty()),
        "blocked artifact runs should not imply approval targets when none exist"
    );
    assert_eq!(run_json["artifact_count"], 6);
    assert!(
        run_json["artifact_paths"].as_array().is_some_and(|paths| paths.len() == 6),
        "blocked brownfield runs should expose all readable artifact paths"
    );
    assert_eq!(run_json["recommended_next_action"]["action"], "inspect-artifacts");

    let blocked_gates = run_json["blocked_gates"].as_array().expect("blocked gates");
    let preservation_gate = blocked_gates
        .iter()
        .find(|gate| gate["gate"] == "brownfield-preservation")
        .expect("brownfield preservation gate");
    assert!(
        preservation_gate["blockers"]
            .as_array()
            .is_some_and(|blockers| blockers.iter().any(|blocker| blocker
                == "legacy-invariants.md is missing required section `Legacy Invariants`")),
        "status should expose the concrete gate blockers"
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
        serde_json::from_slice(&status_output).expect("status json");
    assert_eq!(status_json["state"], "Blocked");
    assert_eq!(status_json["blocking_classification"], "artifact-blocked");
    assert!(
        status_json["approval_targets"].as_array().is_some_and(|targets| targets.is_empty()),
        "blocked artifact runs should not advertise approval targets"
    );
    assert_eq!(status_json["recommended_next_action"]["action"], "inspect-artifacts");
}

#[test]
fn approve_unblocks_systemic_brownfield_runs_and_persists_the_approval_record() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("brownfield.md");
    fs::write(&brief_path, complete_brief()).expect("brief file");

    let run_output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "brownfield-change",
            "--risk",
            "systemic-impact",
            "--zone",
            "yellow",
            "--owner",
            "architect",
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
    let run_id = parse_run_id(&run_output);
    let run_json: serde_json::Value = serde_json::from_slice(&run_output).expect("run json");
    assert_eq!(run_json["state"], "AwaitingApproval");
    assert_eq!(run_json["blocking_classification"], "approval-gated");
    assert!(
        run_json["artifact_paths"].as_array().is_some_and(|paths| paths.is_empty()),
        "approval-gated brownfield runs should not advertise readable artifacts before generation emits them"
    );
    assert_eq!(run_json["recommended_next_action"]["action"], "inspect-evidence");
    let request_id = pending_request_id(&workspace, &run_id);

    let status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", &run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_json: serde_json::Value =
        serde_json::from_slice(&status_output).expect("status json");
    assert_eq!(status_json["state"], "AwaitingApproval");
    assert_eq!(status_json["recommended_next_action"]["action"], "inspect-evidence");

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            &run_id,
            "--target",
            &format!("invocation:{request_id}"),
            "--by",
            "principal-engineer",
            "--decision",
            "approve",
            "--rationale",
            "Systemic work remains recommendation-only and the preserved surface is explicit.",
        ])
        .assert()
        .success()
        .stdout(contains(&run_id));

    let approval_record = workspace
        .path()
        .join(".canon")
        .join("runs")
        .join(&run_id)
        .join("approvals")
        .join("approval-00.toml");
    assert!(approval_record.exists(), "approval record should be persisted");

    cli_command()
        .current_dir(workspace.path())
        .args(["resume", "--run", &run_id])
        .assert()
        .success()
        .stdout(contains(&run_id))
        .stdout(contains("Completed"));

    cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", &run_id, "--output", "json"])
        .assert()
        .success()
        .stdout(contains("\"state\": \"Completed\""));
}

#[test]
fn resume_re_evaluates_fixed_artifacts_and_refuses_stale_context() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("brownfield.md");
    fs::write(&brief_path, blocked_brief()).expect("brief file");

    let run_output = cli_command()
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
    let run_id = parse_run_id(&run_output);

    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(&run_id).join("brownfield-change");

    fs::write(
        artifact_root.join("legacy-invariants.md"),
        "# Legacy Invariants\n\n## Summary\n\nPreserve revocation semantics.\n\n## Legacy Invariants\n\n- Session revocation remains eventually consistent.\n- Audit log ordering stays stable.\n\n## Forbidden Normalization\n\n- Do not normalize away weird but required legacy timing.\n",
    )
    .expect("legacy invariants artifact");
    fs::write(
        artifact_root.join("change-surface.md"),
        "# Change Surface\n\n## Summary\n\nBound the affected modules.\n\n## Change Surface\n\n- session repository\n- auth service\n- token cleanup job\n\n## Ownership\n\n- maintainer\n",
    )
    .expect("change surface artifact");

    cli_command()
        .current_dir(workspace.path())
        .args(["resume", "--run", &run_id])
        .assert()
        .success()
        .stdout(contains(&run_id))
        .stdout(contains("Completed"));

    let second_run_output = cli_command()
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
    let stale_run_id = parse_run_id(&second_run_output);

    fs::write(
        &brief_path,
        "# Brownfield Brief\n\nSystem Slice: auth session boundary and persistence layer.\nChange Surface: auth service and repository.\n",
    )
    .expect("updated brief file");

    cli_command()
        .current_dir(workspace.path())
        .args(["resume", "--run", &stale_run_id])
        .assert()
        .code(5)
        .stderr(contains("stale run"));
}
