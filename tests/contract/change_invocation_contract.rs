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

fn init_change_repo(workspace: &TempDir) {
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
    git(workspace, &["commit", "-m", "seed change repo"]);
}

fn complete_brief() -> &'static str {
    "# Change Brief\n\n## System Slice\n\nauth session boundary and persistence layer.\n\n## Excluded Areas\n\n- payment settlement\n- billing reports\n\n## Intended Change\n\nAdd bounded repository methods while preserving the public auth contract.\n\n## Legacy Invariants\n\n- session revocation remains eventually consistent\n- audit log ordering stays stable\n\n## Forbidden Normalization\n\n- Do not collapse audit-ordering quirks that operators still rely on.\n\n## Change Surface\n\n- session repository\n- auth service\n- token cleanup job\n\n## Ownership\n\n- primary owner: maintainer\n\n## Implementation Plan\n\nAdd bounded repository methods and preserve the public auth contract.\n\n## Sequencing\n\n1. Add bounded repository methods.\n2. Switch callers behind the preserved contract.\n\n## Validation Strategy\n\n- contract tests\n- invariant checks\n\n## Independent Checks\n\n- rollback rehearsal by a separate operator\n\n## Decision Record\n\nPrefer additive change over normalization to preserve operator expectations.\n\n## Consequences\n\n- preserved surface remains explicit and reviewable\n\n## Unresolved Questions\n\n- should the cleanup job roll out in the same slice?\n\nOwner: maintainer\nRisk Level: bounded-impact\nZone: yellow\n"
}

fn broad_surface_brief() -> &'static str {
    "# Change Brief\n\n## System Slice\n\nauth session boundary and persistence layer.\n\n## Excluded Areas\n\n- payment settlement\n\n## Intended Change\n\nTighten the auth persistence boundary while preserving the public auth contract.\n\n## Legacy Invariants\n\n- session revocation remains eventually consistent\n- audit log ordering stays stable\n\n## Forbidden Normalization\n\n- Do not collapse audit-ordering quirks that operators still rely on.\n\n## Change Surface\n\n- auth service\n- session repository\n- adjacent modules\n\n## Ownership\n\n- primary owner: maintainer\n\n## Implementation Plan\n\nTighten the auth persistence boundary while preserving the public auth contract.\n\n## Sequencing\n\n1. Tighten the persistence boundary.\n2. Validate adjacent callers before rollout.\n\n## Validation Strategy\n\n- contract tests\n- invariant checks\n\n## Independent Checks\n\n- rollback rehearsal by a separate operator\n\n## Decision Record\n\nPrefer additive change over normalization to preserve operator expectations.\n\n## Consequences\n\n- preserved surface remains explicit and reviewable\n\n## Unresolved Questions\n\n- do adjacent modules stay in the first rollout?\n\nOwner: maintainer\nRisk Level: bounded-impact\nZone: yellow\n"
}

#[test]
fn systemic_change_run_persists_approval_gated_and_recommendation_only_requests() {
    let workspace = TempDir::new().expect("temp dir");
    init_change_repo(&workspace);
    fs::write(workspace.path().join("change.md"), complete_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "change",
            "--system-context",
            "existing",
            "--risk",
            "systemic-impact",
            "--zone",
            "yellow",
            "--owner",
            "architect",
            "--input",
            "change.md",
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
    let pending_request_id = entries
        .iter()
        .find_map(|entry| {
            if entry["policy_decision"] == "NeedsApproval" {
                entry["request_id"].as_str().map(ToString::to_string)
            } else {
                None
            }
        })
        .expect("pending request");
    assert!(
        entries.iter().any(|entry| entry["latest_outcome"] == "RecommendationOnly"),
        "systemic change work should persist a recommendation-only mutation request"
    );

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            run_id,
            "--target",
            &format!("invocation:{pending_request_id}"),
            "--by",
            "principal-engineer",
            "--decision",
            "approve",
            "--rationale",
            "Allow bounded systemic change generation with explicit ownership.",
        ])
        .assert()
        .success();

    cli_command()
        .current_dir(workspace.path())
        .args(["resume", "--run", run_id])
        .assert()
        .success();

    cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .stdout(predicates::str::contains("\"state\": \"Completed\""));
}

#[test]
fn broad_change_change_surface_escalates_mutation_before_completion() {
    let workspace = TempDir::new().expect("temp dir");
    init_change_repo(&workspace);
    fs::write(workspace.path().join("change.md"), broad_surface_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "change",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
            "--input",
            "change.md",
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
    assert_eq!(json["blocking_classification"], "approval-gated");
    assert_eq!(json["recommended_next_action"]["action"], "inspect-artifacts");
    assert!(
        json["artifact_paths"].as_array().is_some_and(|paths| paths.len() == 6),
        "scope-broadening escalation should still expose the bounded artifact packet"
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
        entries.iter().any(|entry| {
            entry["policy_decision"] == "NeedsApproval"
                && entry["latest_outcome"] == "AwaitingApproval"
        }),
        "broad change surfaces should escalate the mutation request to explicit approval"
    );
}
