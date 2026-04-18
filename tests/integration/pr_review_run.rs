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
    let expected_summary_path = format!(".canon/artifacts/{run_id}/pr-review/review-summary.md");
    assert_eq!(json["mode_result"]["primary_artifact_title"].as_str(), Some("Review Summary"));
    assert_eq!(
        json["mode_result"]["primary_artifact_path"].as_str(),
        Some(expected_summary_path.as_str())
    );
    assert!(
        json["mode_result"]["headline"].as_str().is_some_and(|headline| headline
            .contains("review note")
            && headline.contains("must-fix"))
    );
    assert!(
        json["mode_result"]["result_excerpt"]
            .as_str()
            .is_some_and(|excerpt| excerpt.contains("Ready with review notes"))
    );
    assert_eq!(
        json["mode_result"]["primary_artifact_action"]["id"].as_str(),
        Some("open-primary-artifact")
    );
    assert!(json["recommended_next_action"].is_null());

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
    assert_eq!(
        status_json["mode_result"]["primary_artifact_title"].as_str(),
        Some("Review Summary")
    );
    assert!(status_json["recommended_next_action"].is_null());
}

#[test]
fn run_pr_review_worktree_reviews_uncommitted_changes() {
    let workspace = TempDir::new().expect("temp dir");
    init_review_repo(&workspace);

    // Make changes without committing them
    fs::write(
        workspace.path().join("src/reviewer.rs"),
        "pub fn format_review(label: &str) -> String {\n    format!(\"review:{}\", label.to_uppercase())\n}\n",
    )
    .expect("uncommitted source change");

    let cmd_output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "pr-review",
            "--risk",
            "low-impact",
            "--zone",
            "green",
            "--owner",
            "reviewer",
            "--input",
            "refs/heads/main",
            "--input",
            "WORKTREE",
            "--output",
            "json",
        ])
        .assert()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(cmd_output).expect("utf8 stdout");
    let json: serde_json::Value = serde_json::from_str(&text).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");
    assert_eq!(json["state"], "AwaitingApproval");
    assert_eq!(json["mode_result"]["primary_artifact_title"].as_str(), Some("Review Summary"));
    assert_eq!(json["approval_targets"][0].as_str(), Some("gate:review-disposition"));
    assert_eq!(json["recommended_next_action"]["action"].as_str(), Some("inspect-artifacts"));

    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("pr-review");

    assert!(
        artifact_root.join("pr-analysis.md").exists(),
        "pr-analysis should exist for worktree review"
    );

    let pr_analysis =
        fs::read_to_string(artifact_root.join("pr-analysis.md")).expect("pr analysis artifact");
    assert!(
        pr_analysis.contains("src/reviewer.rs"),
        "pr-analysis should detect the uncommitted change in src/reviewer.rs"
    );
    assert!(
        pr_analysis.contains("WORKTREE"),
        "pr-analysis should reference WORKTREE as the head ref"
    );
}
