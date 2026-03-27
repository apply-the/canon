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

    fs::create_dir_all(workspace.path().join("src")).expect("src dir");
    fs::create_dir_all(workspace.path().join("tests")).expect("tests dir");
    fs::write(
        workspace.path().join("src/reviewer.rs"),
        "pub fn format_review(label: &str) -> String {\n    format!(\"review:{label}\")\n}\n",
    )
    .expect("base source");
    fs::write(
        workspace.path().join("tests/reviewer.md"),
        "# Review Checks\n\nExisting tests cover the formatting helper.\n",
    )
    .expect("base tests");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "base review helper"]);
    git(workspace, &["checkout", "-b", "feature/pr-review"]);
}

fn add_completed_review_diff(workspace: &TempDir) {
    fs::write(
        workspace.path().join("src/reviewer.rs"),
        "pub fn format_review(label: &str) -> String {\n    format!(\"review:{}\", label.trim())\n}\n",
    )
    .expect("updated source");
    fs::write(
        workspace.path().join("tests/reviewer.md"),
        "# Review Checks\n\n- formatting trims trailing whitespace before labeling\n",
    )
    .expect("updated tests");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "trim review labels"]);
}

#[test]
fn run_pr_review_emits_review_packet_and_maps_changed_surfaces() {
    let workspace = TempDir::new().expect("temp dir");
    init_review_repo(&workspace);
    add_completed_review_diff(&workspace);

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
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).expect("utf8 stdout");
    let json: serde_json::Value = serde_json::from_str(&text).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");

    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("pr-review");

    for artifact in [
        "pr-analysis.md",
        "boundary-check.md",
        "duplication-check.md",
        "contract-drift.md",
        "missing-tests.md",
        "decision-impact.md",
        "review-summary.md",
    ] {
        assert!(
            artifact_root.join(artifact).exists(),
            "{artifact} should exist in the pr-review bundle"
        );
    }

    let pr_analysis =
        fs::read_to_string(artifact_root.join("pr-analysis.md")).expect("pr analysis artifact");
    assert!(
        pr_analysis.contains("src/reviewer.rs"),
        "pr-analysis should map the changed source file"
    );
    let review_summary = fs::read_to_string(artifact_root.join("review-summary.md"))
        .expect("review summary artifact");
    assert!(
        review_summary.contains("Ready with review notes"),
        "review-summary should record a non-blocking disposition"
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
    assert_eq!(status_json["state"], "Completed");
}
