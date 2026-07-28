use std::fs;
use std::process::Command as ProcessCommand;

use assert_cmd::Command;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::SystemContext;
use tempfile::TempDir;

#[path = "../support/historical_run.rs"]
mod historical_run;

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

fn init_repo(workspace: &TempDir) {
    git(workspace, &["init", "-b", "main"]);
    git(workspace, &["config", "user.name", "Canon Test"]);
    git(workspace, &["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("src/auth")).expect("src dir");
    fs::write(
        workspace.path().join("src/auth/tokens.rs"),
        "pub fn token_owner(id: &str) -> String {\n    format!(\"owner:{id}\")\n}\n",
    )
    .expect("source file");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "seed migration publish repo"]);
}

fn incomplete_brief() -> &'static str {
    "# Migration Brief\n\nCurrent State: auth-v1 serves login and token refresh traffic.\nTarget State: auth-v2 serves the same bounded traffic surface.\nTransition Boundaries: login and token refresh only.\nGuaranteed Compatibility:\n- existing tokens continue to validate\nTemporary Incompatibilities:\n- admin reporting stays on v1 during the rollout\nCoexistence Rules:\n- dual-write session metadata during cutover\nOrdered Steps:\n- enable shadow reads\n- start dual-write\n- cut traffic to auth-v2\nParallelizable Work:\n- docs and dashboards can update in parallel\nCutover Criteria:\n- error rate and token validation remain stable\nVerification Checks:\n- login and token validation pass against auth-v2\nResidual Risks:\n- admin reporting remains temporarily inconsistent\nRelease Readiness:\n- fallback credibility is not yet established\nMigration Decisions:\n- retain dual-write during the bounded cutover\nDeferred Decisions:\n- move admin reporting after the bounded migration completes\nApproval Notes:\n- explicit migration-lead sign-off is required before broader rollout\n"
}

fn default_publish_leaf(run_id: &str, descriptor: &str) -> String {
    format!("{}-{}-{}-{descriptor}", &run_id[2..6], &run_id[6..8], &run_id[8..10])
}

#[test]
fn blocked_migration_packet_is_publishable_with_honest_fallback_gaps() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    fs::write(workspace.path().join("migration.md"), incomplete_brief()).expect("brief file");

    let summary = historical_run::start(
        workspace.path(),
        Mode::Migration,
        RiskClass::LowImpact,
        UsageZone::Green,
        SystemContext::Existing,
        "migration-lead",
        "migration.md",
    )
    .expect("internal historical migration run");
    let json = serde_json::to_value(summary).expect("json summary");
    let run_id = json["run_id"].as_str().expect("run id");
    assert_eq!(json["state"], "Blocked");

    cli_command().current_dir(workspace.path()).args(["publish", run_id]).assert().success();

    let published = workspace
        .path()
        .join("tech-docs")
        .join("migrations")
        .join(default_publish_leaf(run_id, "migration"))
        .join("04-fallback-plan.md");
    let published_text = fs::read_to_string(published).expect("published fallback plan");
    assert!(
        workspace
            .path()
            .join("tech-docs")
            .join("migrations")
            .join(default_publish_leaf(run_id, "migration"))
            .join("packet-metadata.json")
            .exists()
    );
    assert!(published_text.contains("NOT CAPTURED"));
}
