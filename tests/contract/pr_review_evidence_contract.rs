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
fn pr_review_attempts_retain_payload_refs_and_artifact_provenance() {
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

    let invocations = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "invocations", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let invocation_json: serde_json::Value = serde_json::from_slice(&invocations).expect("json");
    let diff_request_id = invocation_json["entries"]
        .as_array()
        .and_then(|entries| {
            entries.iter().find_map(|entry| {
                if entry["capability"] == "InspectDiff" {
                    entry["request_id"].as_str().map(ToString::to_string)
                } else {
                    None
                }
            })
        })
        .expect("inspect diff request");

    let attempt = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("runs")
            .join(run_id)
            .join("invocations")
            .join(&diff_request_id)
            .join("attempt-01.toml"),
    )
    .expect("attempt manifest");
    assert!(attempt.contains("payload_refs"), "attempt manifests should retain payload refs");
    assert!(
        attempt.contains("payload/diff.patch"),
        "diff inspection should retain a bounded patch payload reference"
    );

    let manifest = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(run_id)
            .join("pr-review")
            .join("manifest.toml"),
    )
    .expect("artifact manifest");
    assert!(manifest.contains("provenance"), "pr-review artifacts should carry provenance");
    assert!(
        manifest.contains(&format!("runs/{run_id}/evidence.toml")),
        "pr-review artifact provenance should link back to the evidence bundle"
    );
}
