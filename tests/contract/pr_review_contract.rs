use std::fs;
use std::process::Command as ProcessCommand;

use assert_cmd::Command;
use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::mode::Mode;
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
fn pr_review_contract_includes_conventional_comments_artifact() {
    let contract = contract_for_mode(Mode::PrReview);
    let conventional_comments = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.slug() == "conventional-comments.md")
        .expect("conventional-comments artifact requirement");

    assert_eq!(conventional_comments.required_sections, vec!["Summary"]);
}

#[test]
fn pr_review_run_mode_is_removed() {
    let workspace = TempDir::new().expect("temp dir");
    init_review_repo(&workspace);
    add_high_impact_diff(&workspace);

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
        .stderr(contains("canon run --mode pr-review has been removed"))
        .stderr(contains("canon pr-review prepare"));
}
