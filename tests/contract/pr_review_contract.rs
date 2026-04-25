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

fn parse_run_id(output: &[u8]) -> String {
    let json: serde_json::Value = serde_json::from_slice(output).expect("json output");
    json["run_id"].as_str().expect("run id").to_string()
}

#[test]
fn pr_review_contract_includes_conventional_comments_artifact() {
    let contract = contract_for_mode(Mode::PrReview);
    let conventional_comments = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "conventional-comments.md")
        .expect("conventional-comments artifact requirement");

    assert_eq!(
        conventional_comments.required_sections,
        vec!["Summary", "Evidence Posture", "Conventional Comments", "Traceability",]
    );
}

#[test]
fn pr_review_requires_disposition_for_high_impact_findings() {
    let workspace = TempDir::new().expect("temp dir");
    init_review_repo(&workspace);
    add_high_impact_diff(&workspace);

    let run_output = cli_command()
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
        .stdout(contains("\"state\": \"AwaitingApproval\""))
        .get_output()
        .stdout
        .clone();
    let run_id = parse_run_id(&run_output);
    let run_json: serde_json::Value = serde_json::from_slice(&run_output).expect("run json");
    assert_eq!(run_json["mode_result"]["primary_artifact_title"], "Review Summary");
    assert_eq!(
        run_json["mode_result"]["primary_artifact_path"],
        format!(".canon/artifacts/{run_id}/pr-review/review-summary.md")
    );
    assert!(
        run_json["mode_result"]["headline"]
            .as_str()
            .is_some_and(|headline| headline.contains("waiting for explicit disposition"))
    );
    assert_eq!(run_json["recommended_next_action"]["action"], "inspect-artifacts");
    assert!(
        run_json["artifact_paths"].as_array().is_some_and(|paths| paths.len() == 8),
        "approval-gated pr-review runs should still expose the readable review packet"
    );

    let review_summary = workspace
        .path()
        .join(".canon")
        .join("artifacts")
        .join(&run_id)
        .join("pr-review")
        .join("review-summary.md");
    let review_summary_text = fs::read_to_string(review_summary).expect("review summary");
    assert!(
        review_summary_text.contains("Must-fix findings require explicit disposition"),
        "review-summary should retain unresolved must-fix findings"
    );
    assert!(
        review_summary_text.contains("contracts/public-api.md"),
        "review-summary should name the changed high-impact surface"
    );

    let conventional_comments = workspace
        .path()
        .join(".canon")
        .join("artifacts")
        .join(&run_id)
        .join("pr-review")
        .join("conventional-comments.md");
    let conventional_comments_text =
        fs::read_to_string(conventional_comments).expect("conventional comments artifact");
    assert!(
        conventional_comments_text.contains("issue:"),
        "high-impact contract drift should surface as an issue comment"
    );
    assert!(
        conventional_comments_text.contains("question:"),
        "decision-impact findings should surface as reviewer questions"
    );
    assert!(
        conventional_comments_text.contains("contracts/public-api.md"),
        "conventional-comments should retain changed surface traceability"
    );

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
    assert_eq!(status_json["mode_result"]["primary_artifact_title"], "Review Summary");
    assert_eq!(status_json["recommended_next_action"]["action"], "inspect-artifacts");

    cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", &run_id, "--output", "markdown"])
        .assert()
        .success()
        .stdout(contains("## Result"))
        .stdout(contains("review-summary.md"))
        .stdout(contains("waiting for explicit disposition"));

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            &run_id,
            "--gate",
            "review-disposition",
            "--by",
            "principal-engineer",
            "--decision",
            "approve",
            "--rationale",
            "Accept the contract drift with explicit downstream coordination.",
        ])
        .assert()
        .success()
        .stdout(contains(&run_id))
        .stdout(contains("Completed"));

    cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", &run_id, "--output", "json"])
        .assert()
        .success()
        .stdout(contains("\"state\": \"Completed\""))
        .stdout(contains("\"primary_artifact_title\": \"Review Summary\""))
        .stdout(contains("\"recommended_next_action\": null"));
}
