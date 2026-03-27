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

#[test]
fn blocked_brownfield_run_returns_exit_code_2_and_mentions_preservation_gap() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("brownfield.md");
    fs::write(&brief_path, blocked_brief()).expect("brief file");

    cli_command()
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
        .stdout(contains("\"state\": \"Blocked\""));
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

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            &run_id,
            "--gate",
            "risk",
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
