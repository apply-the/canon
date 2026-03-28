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

fn init_review_repo(workspace: &TempDir) {
    git(workspace, &["init", "-b", "main"]);
    git(workspace, &["config", "user.name", "Canon Test"]);
    git(workspace, &["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("contracts")).expect("contracts dir");
    fs::create_dir_all(workspace.path().join("src")).expect("src dir");
    fs::write(workspace.path().join("contracts/public-api.md"), "# Public API\n\nStatus: stable\n")
        .expect("base contract");
    fs::write(
        workspace.path().join("src/http_boundary.rs"),
        "pub fn public_response() -> &'static str {\n    \"ok\"\n}\n",
    )
    .expect("base boundary");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "base boundary"]);
    git(workspace, &["checkout", "-b", "feature/pr-review"]);
}

fn add_high_impact_diff(workspace: &TempDir) {
    fs::write(
        workspace.path().join("contracts/public-api.md"),
        "# Public API\n\nStatus: breaking\n\nError shape now includes a retry hint.\n",
    )
    .expect("updated contract");
    fs::write(
        workspace.path().join("src/http_boundary.rs"),
        "pub fn public_response() -> &'static str {\n    \"retry-required\"\n}\n",
    )
    .expect("updated boundary");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "change public response contract"]);
}

#[test]
fn pr_review_run_persists_invocation_evidence_and_independent_validation_paths() {
    let workspace = TempDir::new().expect("temp dir");
    init_review_repo(&workspace);
    add_high_impact_diff(&workspace);

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "pr-review",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "reviewer",
            "--input",
            "refs/heads/main",
            "--input",
            "HEAD",
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
    assert_eq!(json["invocations_total"], 2);
    assert_eq!(json["invocations_denied"], 0);
    assert_eq!(json["invocations_pending_approval"], 0);
    assert_eq!(json["evidence_bundle"], format!("runs/{run_id}/evidence.toml"));

    let status = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_json: serde_json::Value = serde_json::from_slice(&status).expect("json");
    assert_eq!(status_json["validation_independence_satisfied"], true);
    assert_eq!(status_json["evidence_bundle"], format!("runs/{run_id}/evidence.toml"));

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
    assert_eq!(entries.len(), 2, "pr-review should persist diff and critique requests");
    assert!(
        entries.iter().any(|entry| entry["capability"] == "InspectDiff"),
        "pr-review should persist the diff inspection request"
    );
    assert!(
        entries.iter().any(|entry| entry["capability"] == "CritiqueContent"),
        "pr-review should persist the critique request"
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
    let entry = evidence_json["entries"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("evidence entry");
    assert!(
        entry["generation_paths"].as_array().is_some_and(|paths| !paths.is_empty()),
        "pr-review should expose a generation path for critique output"
    );
    assert!(
        entry["validation_paths"].as_array().is_some_and(|paths| !paths.is_empty()),
        "pr-review should expose a validation path for deterministic diff inspection"
    );
    assert!(
        entry["artifact_provenance_links"].as_array().is_some_and(|paths| !paths.is_empty()),
        "evidence should link back to review artifacts"
    );
}
