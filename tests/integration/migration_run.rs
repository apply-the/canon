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
    "# Migration Brief\n\n## Current State\n\n- auth-v1 serves login and token refresh traffic.\n\n## Target State\n\n- auth-v2 serves the same bounded traffic surface.\n\n## Transition Boundaries\n\n- login and token refresh only.\n\n## Guaranteed Compatibility\n\n- existing tokens continue to validate\n\n## Temporary Incompatibilities\n\n- admin reporting stays on v1 during the rollout\n\n## Coexistence Rules\n\n- dual-write session metadata during cutover\n\n## Options Matrix\n\n- Option 1 keeps dual-write through the cutover window.\n- Option 2 cuts directly to auth-v2 and accepts a tighter rollback window.\n\n## Ordered Steps\n\n1. enable shadow reads\n2. start dual-write\n3. cut traffic to auth-v2\n\n## Parallelizable Work\n\n- docs and dashboards can update in parallel\n\n## Cutover Criteria\n\n- error rate and token validation remain stable\n\n## Rollback Triggers\n\n- token validation failures or elevated login errors\n\n## Fallback Paths\n\n- route bounded traffic back to auth-v1\n\n## Re-Entry Criteria\n\n- compatibility regressions are resolved and revalidated\n\n## Adoption Implications\n\n- keep the auth token path bounded to auth-v2 before adjacent reporting workloads adopt it.\n\n## Verification Checks\n\n- login and token validation pass against auth-v2\n\n## Residual Risks\n\n- admin reporting remains temporarily inconsistent\n\n## Release Readiness\n\n- keep recommendation-only posture until the owner accepts the packet\n\n## Migration Decisions\n\n- retain dual-write during the bounded cutover\n\n## Tradeoff Analysis\n\n- dual-write raises temporary complexity but keeps rollback safer while the bounded surface proves stable\n\n## Recommendation\n\n- keep dual-write for the bounded auth token path and defer broader reporting migration\n\n## Ecosystem Health\n\n- auth-v2 dependencies are healthy enough for bounded cutover, but reporting integrations still lag behind\n\n## Deferred Decisions\n\n- move admin reporting after the bounded migration completes\n\n## Approval Notes\n\n- explicit migration-lead sign-off is required before broader rollout\n"
}

fn incomplete_brief() -> &'static str {
    "# Migration Brief\n\n## Current State\n\n- auth-v1 serves login and token refresh traffic.\n\n## Target State\n\n- auth-v2 serves the same bounded traffic surface.\n\n## Fallback Paths\n\n- route bounded traffic back to auth-v1\n\n## Re-Entry Criteria\n\n- compatibility regressions are resolved and revalidated\n"
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

    let compatibility =
        fs::read_to_string(artifact_root.join("compatibility-matrix.md")).expect("compatibility");
    assert!(compatibility.contains("## Options Matrix"));

    let fallback_plan =
        fs::read_to_string(artifact_root.join("fallback-plan.md")).expect("fallback plan");
    assert!(
        fallback_plan.contains(
            "## Rollback Triggers\n\n- token validation failures or elevated login errors"
        )
    );
    assert!(fallback_plan.contains("## Fallback Paths\n\n- route bounded traffic back to auth-v1"));
    assert!(fallback_plan.contains("## Adoption Implications"));
    assert!(!fallback_plan.contains("## Missing Authored Body"));

    let decision_record =
        fs::read_to_string(artifact_root.join("decision-record.md")).expect("decision record");
    assert!(decision_record.contains("## Tradeoff Analysis"));
    assert!(decision_record.contains("## Recommendation"));
    assert!(decision_record.contains("## Ecosystem Health"));

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

#[test]
fn run_migration_blocks_when_a_required_authored_section_is_missing() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    fs::write(workspace.path().join("migration.md"), incomplete_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "migration",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
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
        .code(2)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("migration");

    assert_eq!(json["state"], "Blocked");
    assert_eq!(json["blocking_classification"], "artifact-blocked");

    let fallback_plan =
        fs::read_to_string(artifact_root.join("fallback-plan.md")).expect("fallback plan");
    assert!(fallback_plan.contains("## Missing Authored Body"));
    assert!(fallback_plan.contains("`## Rollback Triggers`"));
}
