use std::fs;
use std::process::Command as ProcessCommand;

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

fn git(workspace: &TempDir, args: &[&str]) {
    let output = ProcessCommand::new("git")
        .args(args)
        .current_dir(workspace.path())
        .output()
        .expect("git command");
    assert!(
        output.status.success(),
        "git {:?} failed: stdout=`{}` stderr=`{}`",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_brownfield_repo(workspace: &TempDir) {
    git(workspace, &["init", "-b", "main"]);
    git(workspace, &["config", "user.name", "Canon Test"]);
    git(workspace, &["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("src/auth")).expect("src dir");
    fs::create_dir_all(workspace.path().join("tests")).expect("tests dir");
    fs::write(
        workspace.path().join("src/auth/session.rs"),
        "pub fn revoke_session(id: &str) -> String {\n    format!(\"revoked:{id}\")\n}\n",
    )
    .expect("source file");
    fs::write(
        workspace.path().join("tests/session.md"),
        "# Session Checks\n\n- revocation formatting remains stable\n",
    )
    .expect("test file");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "seed brownfield repo"]);
}

fn complete_brief() -> &'static str {
    "# Brownfield Brief\n\nSystem Slice: auth session boundary and persistence layer.\nLegacy Invariants: session revocation remains eventually consistent and audit log ordering stays stable.\nChange Surface: session repository, auth service, and token cleanup job.\nImplementation Plan: add bounded repository methods and preserve the public auth contract.\nValidation Strategy: contract tests, invariant checks, and rollback rehearsal.\nDecision Record: prefer additive change over normalization to preserve operator expectations.\n"
}

#[test]
fn brownfield_governed_run_persists_evidence_and_independent_validation_paths() {
    let workspace = TempDir::new().expect("temp dir");
    init_brownfield_repo(&workspace);
    fs::write(workspace.path().join("brownfield.md"), complete_brief()).expect("brief file");

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
            "brownfield.md",
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

    assert!(
        json["invocations_total"].as_u64().is_some_and(|count| count >= 3),
        "brownfield run should record governed repository, generation, and validation requests"
    );
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
    assert!(
        entries.iter().any(|entry| entry["capability"] == "ReadRepository"),
        "brownfield run should persist repository context capture"
    );
    assert!(
        entries.iter().any(|entry| entry["capability"] == "GenerateContent"),
        "brownfield run should persist bounded generation"
    );
    assert!(
        entries.iter().any(|entry| entry["capability"] == "ValidateWithTool"),
        "brownfield run should persist independent validation-tool execution"
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
    let bundle = evidence_json["entries"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("evidence bundle");
    assert!(
        bundle["generation_paths"].as_array().is_some_and(|paths| !paths.is_empty()),
        "brownfield evidence should include a generation path"
    );
    assert!(
        bundle["validation_paths"].as_array().is_some_and(|paths| !paths.is_empty()),
        "brownfield evidence should include a validation path"
    );

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
    assert_eq!(status_json["validation_independence_satisfied"], true);
}
