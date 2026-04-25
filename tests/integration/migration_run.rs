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
    git(workspace, &["commit", "-m", "seed migration repo"]);
}

fn complete_brief() -> &'static str {
    "# Migration Brief\n\nCurrent State: auth-v1 serves login and token refresh traffic.\nTarget State: auth-v2 serves the same bounded traffic surface.\nTransition Boundaries: login and token refresh only.\nGuaranteed Compatibility:\n- existing tokens continue to validate\nTemporary Incompatibilities:\n- admin reporting stays on v1 during the rollout\nCoexistence Rules:\n- dual-write session metadata during cutover\nOrdered Steps:\n- enable shadow reads\n- start dual-write\n- cut traffic to auth-v2\nParallelizable Work:\n- docs and dashboards can update in parallel\nCutover Criteria:\n- error rate and token validation remain stable\nRollback Triggers:\n- token validation failures or elevated login errors\nFallback Paths:\n- route bounded traffic back to auth-v1\nRe-Entry Criteria:\n- compatibility regressions are resolved and revalidated\nVerification Checks:\n- login and token validation pass against auth-v2\nResidual Risks:\n- admin reporting remains temporarily inconsistent\nRelease Readiness:\n- keep recommendation-only posture until the owner accepts the packet\nMigration Decisions:\n- retain dual-write during the bounded cutover\nDeferred Decisions:\n- move admin reporting after the bounded migration completes\nApproval Notes:\n- explicit migration-lead sign-off is required before broader rollout\n"
}

#[test]
fn run_migration_emits_a_compatibility_packet_and_publishes_after_risk_approval() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    fs::write(workspace.path().join("migration.md"), complete_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "migration",
            "--system-context",
            "existing",
            "--risk",
            "systemic-impact",
            "--zone",
            "yellow",
            "--owner",
            "migration-lead",
            "--input",
            "migration.md",
            "--output",
            "json",
        ])
        .assert()
        .code(3)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("migration");

    assert_eq!(json["state"], "AwaitingApproval");
    assert_eq!(json["artifact_count"], 6);
    assert_eq!(json["mode_result"]["execution_posture"].as_str(), Some("recommendation-only"));
    assert_eq!(json["mode_result"]["primary_artifact_title"].as_str(), Some("Source-Target Map"));
    assert!(
        json["approval_targets"].as_array().is_some_and(|targets| targets
            .iter()
            .any(|target| target.as_str() == Some("gate:risk")))
    );
    assert!(artifact_root.join("source-target-map.md").exists());
    assert!(artifact_root.join("compatibility-matrix.md").exists());
    assert!(artifact_root.join("fallback-plan.md").exists());

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            run_id,
            "--target",
            "gate:risk",
            "--by",
            "migration-lead",
            "--decision",
            "approve",
            "--rationale",
            "bounded compatibility packet accepted for rollout review",
        ])
        .assert()
        .success();

    let status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_json: serde_json::Value =
        serde_json::from_slice(&status_output).expect("status json");
    assert_eq!(status_json["state"], "Completed");
    assert_eq!(
        status_json["mode_result"]["primary_artifact_title"].as_str(),
        Some("Source-Target Map")
    );

    cli_command().current_dir(workspace.path()).args(["publish", run_id]).assert().success();

    assert!(
        workspace
            .path()
            .join("docs")
            .join("migrations")
            .join(run_id)
            .join("source-target-map.md")
            .exists()
    );
}
