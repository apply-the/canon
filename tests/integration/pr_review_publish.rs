use std::fs;
use std::process::Command as ProcessCommand;

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

fn git(workspace: &TempDir, args: &[&str]) {
    let output = ProcessCommand::new("git")
        .args(["-c", "commit.gpgsign=false", "-c", "tag.gpgsign=false"])
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
        "pub fn format_review(label: &str) -> String {\n    format!(\"review:{}\", label.trim())\n}\n",
    )
    .expect("base source");
    fs::write(
        workspace.path().join("tests/reviewer.md"),
        "# Review Checks\n\n- formatting trims trailing whitespace before labeling\n",
    )
    .expect("base tests");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "base review helper"]);
    git(workspace, &["checkout", "-b", "feature/pr-review"]);
}

fn add_completed_review_diff(workspace: &TempDir) {
    fs::write(
        workspace.path().join("src/reviewer.rs"),
        "pub fn format_review(label: &str) -> String {\n    format!(\"review:{}\", label.trim().to_uppercase())\n}\n",
    )
    .expect("updated source");
    fs::write(
        workspace.path().join("tests/reviewer.md"),
        "# Review Checks\n\n- formatting trims trailing whitespace before labeling\n- output uppercases the label for reviewer readability\n",
    )
    .expect("updated tests");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "uppercase review labels"]);
}

#[test]
fn pr_review_run_mode_is_removed() {
    let workspace = TempDir::new().expect("temp dir");
    init_review_repo(&workspace);
    add_completed_review_diff(&workspace);

    cli_command()
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
        ])
        .assert()
        .failure()
        .stderr(contains("canon run --mode pr-review has been removed"));
}
