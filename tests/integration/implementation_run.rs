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
        workspace.path().join("src/auth/session.rs"),
        "pub fn revoke_session(id: &str) -> String {\n    format!(\"revoked:{id}\")\n}\n",
    )
    .expect("source file");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "seed implementation repo"]);
}

fn complete_brief() -> &'static str {
    "# Implementation Brief\n\nTask Mapping: 1. Add bounded auth session repository helpers.\n2. Thread the helper through the revocation service without expanding the public API.\nMutation Bounds: src/auth/session.rs; src/auth/repository.rs\nAllowed Paths:\n- src/auth/session.rs\n- src/auth/repository.rs\nSafety-Net Evidence: contract coverage protects revocation formatting and audit ordering before mutation.\nIndependent Checks: cargo test --test session_contract\nRollback Triggers: revocation output drifts or audit ordering becomes unstable.\nRollback Steps: revert the bounded auth-session patch and redeploy the previous build.\n"
}

fn implementation_patch() -> &'static str {
    "diff --git a/src/auth/session.rs b/src/auth/session.rs\nindex f5337d3..90af012 100644\n--- a/src/auth/session.rs\n+++ b/src/auth/session.rs\n@@ -1,3 +1,7 @@\n pub fn revoke_session(id: &str) -> String {\n-    format!(\"revoked:{id}\")\n+    let normalized = id.trim();\n+    format!(\"revoked:{normalized}\")\n }\n+\n+pub fn session_repository_key(id: &str) -> String {\n+    format!(\"session:{}\", id.trim())\n+}\n"
}

#[test]
fn run_implementation_completes_with_recommendation_only_execution_posture() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    let brief_path = workspace.path().join("implementation.md");
    fs::write(&brief_path, complete_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "implementation",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
            "--input",
            "implementation.md",
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
        workspace.path().join(".canon").join("artifacts").join(run_id).join("implementation");

    assert_eq!(json["state"], "AwaitingApproval");
    assert_eq!(json["mode_result"]["execution_posture"].as_str(), Some("recommendation-only"));
    assert_eq!(json["mode_result"]["primary_artifact_title"].as_str(), Some("Task Mapping"));
    let approval_targets = json["approval_targets"].as_array().expect("approval targets array");
    assert!(
        approval_targets.iter().any(|target| target.as_str() == Some("gate:execution")),
        "approval_targets should contain gate:execution, got: {approval_targets:?}"
    );

    let chips = json["mode_result"]["action_chips"].as_array().expect("chips array");
    let chip_ids: Vec<&str> = chips.iter().filter_map(|chip| chip["id"].as_str()).collect();
    assert!(
        chip_ids.contains(&"open-primary-artifact"),
        "expected open-primary-artifact chip, got {chip_ids:?}"
    );
    assert!(
        chip_ids.contains(&"inspect-evidence"),
        "expected inspect-evidence chip, got {chip_ids:?}"
    );
    assert!(
        chip_ids.contains(&"approve-gate-execution"),
        "expected approve-gate-execution chip, got {chip_ids:?}"
    );
    let inspect_chip = chips
        .iter()
        .find(|chip| chip["id"].as_str() == Some("inspect-evidence"))
        .expect("inspect chip");
    assert_eq!(inspect_chip["intent"].as_str(), Some("Inspect"));
    assert_eq!(
        inspect_chip["text_fallback"].as_str(),
        Some(format!("Use $canon-inspect-evidence for run {run_id}.").as_str())
    );

    assert!(artifact_root.join("task-mapping.md").exists());
    assert!(artifact_root.join("mutation-bounds.md").exists());
    assert!(artifact_root.join("validation-hooks.md").exists());

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            run_id,
            "--target",
            "gate:execution",
            "--by",
            "maintainer",
            "--decision",
            "approve",
            "--rationale",
            "approved bounded execution",
        ])
        .assert()
        .success();

    let approved_status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let approved_status_json: serde_json::Value =
        serde_json::from_slice(&approved_status_output).expect("approved status json");
    assert_eq!(approved_status_json["state"], "AwaitingApproval");
    assert_eq!(
        approved_status_json["mode_result"]["execution_posture"].as_str(),
        Some("recommendation-only")
    );
    assert!(
        approved_status_json["approval_targets"]
            .as_array()
            .is_some_and(|targets| targets.is_empty())
    );
    assert_eq!(approved_status_json["recommended_next_action"]["action"].as_str(), Some("resume"));
    let approved_chip_ids: Vec<&str> = approved_status_json["mode_result"]["action_chips"]
        .as_array()
        .expect("approved chips array")
        .iter()
        .filter_map(|chip| chip["id"].as_str())
        .collect();
    assert!(approved_chip_ids.contains(&"resume-run"));
    assert!(
        !approved_chip_ids.iter().any(|id| id.starts_with("approve-")),
        "approved status should not surface stale approve chips: {approved_chip_ids:?}"
    );

    let resumed = cli_command()
        .current_dir(workspace.path())
        .args(["resume", "--run", run_id])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let resumed_json: serde_json::Value = serde_json::from_slice(&resumed).expect("resume json");
    assert_eq!(resumed_json["state"], "Completed");
    assert_eq!(
        resumed_json["mode_result"]["execution_posture"].as_str(),
        Some("approved-recommendation")
    );

    let resumed_chip_ids: Vec<&str> = resumed_json["mode_result"]["action_chips"]
        .as_array()
        .expect("chips array")
        .iter()
        .filter_map(|chip| chip["id"].as_str())
        .collect();
    assert!(resumed_chip_ids.contains(&"open-primary-artifact"));
    assert!(resumed_chip_ids.contains(&"inspect-evidence"));
    assert!(
        !resumed_chip_ids.iter().any(|id| id.starts_with("approve-")),
        "completed runs should not surface approve chips: {resumed_chip_ids:?}"
    );

    let status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_json: serde_json::Value = serde_json::from_slice(&status_output).expect("json");
    assert_eq!(status_json["state"], "Completed");
    assert_eq!(
        status_json["mode_result"]["execution_posture"].as_str(),
        Some("approved-recommendation")
    );

    cli_command().current_dir(workspace.path()).args(["publish", run_id]).assert().success();

    assert!(
        workspace
            .path()
            .join("docs")
            .join("implementation")
            .join(run_id)
            .join("task-mapping.md")
            .exists()
    );
}

#[test]
fn systemic_implementation_run_remains_recommendation_only_and_publishable() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    let brief_path = workspace.path().join("implementation.md");
    fs::write(&brief_path, complete_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "implementation",
            "--system-context",
            "existing",
            "--risk",
            "systemic-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
            "--input",
            "implementation.md",
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

    assert_eq!(json["state"], "AwaitingApproval");
    assert_eq!(json["mode_result"]["execution_posture"].as_str(), Some("recommendation-only"));
    let approval_targets = json["approval_targets"].as_array().expect("approval targets array");
    assert!(
        approval_targets.iter().any(|target| target.as_str() == Some("gate:execution")),
        "approval_targets should contain gate:execution, got: {approval_targets:?}"
    );

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            run_id,
            "--target",
            "gate:execution",
            "--by",
            "maintainer",
            "--decision",
            "approve",
            "--rationale",
            "approved systemic execution",
        ])
        .assert()
        .success();

    cli_command()
        .current_dir(workspace.path())
        .args(["resume", "--run", run_id])
        .assert()
        .success();

    cli_command().current_dir(workspace.path()).args(["publish", run_id]).assert().success();

    assert!(
        workspace
            .path()
            .join("docs")
            .join("implementation")
            .join(run_id)
            .join("task-mapping.md")
            .exists()
    );
}

#[test]
fn approved_implementation_resume_applies_bounded_patch_to_workspace() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);

    let packet_dir = workspace.path().join("canon-input").join("implementation");
    fs::create_dir_all(&packet_dir).expect("packet dir");
    fs::write(packet_dir.join("brief.md"), complete_brief()).expect("brief file");
    fs::write(packet_dir.join("patch.diff"), implementation_patch()).expect("patch file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "implementation",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
            "--input",
            "canon-input/implementation",
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
    let original = fs::read_to_string(workspace.path().join("src/auth/session.rs"))
        .expect("original session contents");
    assert!(original.contains("format!(\"revoked:{id}\")"));

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            run_id,
            "--target",
            "gate:execution",
            "--by",
            "maintainer",
            "--decision",
            "approve",
            "--rationale",
            "approved bounded execution",
        ])
        .assert()
        .success();

    let resumed = cli_command()
        .current_dir(workspace.path())
        .args(["resume", "--run", run_id])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let resumed_json: serde_json::Value = serde_json::from_slice(&resumed).expect("resume json");
    assert_eq!(resumed_json["state"], "Completed");
    assert_eq!(resumed_json["mode_result"]["execution_posture"].as_str(), Some("mutating"));

    let session_contents = fs::read_to_string(workspace.path().join("src/auth/session.rs"))
        .expect("mutated session contents");
    assert!(session_contents.contains("let normalized = id.trim();"));
    assert!(session_contents.contains("pub fn session_repository_key"));
    assert!(!session_contents.contains("format!(\"revoked:{id}\")"));
    assert!(
        !workspace.path().join("src/auth/repository.rs").exists(),
        "the bounded patch should not materialize untouched allowed paths"
    );
}
