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
    git(workspace, &["commit", "-m", "seed refactor repo"]);
}

fn complete_brief() -> &'static str {
    r#"# Refactor Brief

Feature Slice: Auth session boundary and repository composition inside the existing login subsystem.
Primary Upstream Mode: implementation

## Preserved Behavior
Session revocation formatting and audit ordering remain externally unchanged.

## Approved Exceptions
None.

## Refactor Scope
Auth session boundary and repository composition only.

## Allowed Paths
- src/auth/session.rs
- src/auth/repository.rs

## Structural Rationale
Isolate persistence concerns and internal composition without changing externally meaningful behavior.

## Untouched Surface
Public auth API, tests/session.md, deployment wiring, and analytics consumers stay unchanged.

## Safety-Net Evidence
Contract coverage protects revocation formatting and audit ordering before structural cleanup.

## Regression Findings
No regression findings are accepted in this bounded packet.

## Contract Drift
No public contract drift is allowed.

## Reviewer Notes
Reviewer confirmation is required before any drift or feature semantics are accepted.

## Feature Audit
No new feature behavior is introduced in this refactor packet.

## Decision
Preserve behavior and stop immediately if the surface expands or the packet starts to add feature semantics.
"#
}

fn incomplete_brief() -> &'static str {
    r#"# Refactor Brief

Feature Slice: Auth session boundary and repository composition inside the existing login subsystem.
Primary Upstream Mode: implementation

## Preserved Behavior
Session revocation formatting and audit ordering remain externally unchanged.

## Approved Exceptions
None.

## Refactor Scope
Auth session boundary and repository composition only.

## Allowed Paths
- src/auth/session.rs
- src/auth/repository.rs

## Structural Rationale
Isolate persistence concerns and internal composition without changing externally meaningful behavior.

## Untouched Surface
Public auth API, tests/session.md, deployment wiring, and analytics consumers stay unchanged.

## Safety-Net Evidence
Contract coverage protects revocation formatting and audit ordering before structural cleanup.

## Regression Findings
No regression findings are accepted in this bounded packet.

## Contract Drift
No public contract drift is allowed.

## Reviewer Notes
Reviewer confirmation is required before any drift or feature semantics are accepted.

## Feature Audit
No new feature behavior is introduced in this refactor packet.
"#
}

fn refactor_patch() -> &'static str {
    "diff --git a/src/auth/session.rs b/src/auth/session.rs\nindex f5337d3..b0dcc0f 100644\n--- a/src/auth/session.rs\n+++ b/src/auth/session.rs\n@@ -1,3 +1,7 @@\n+fn revoked_label(id: &str) -> String {\n+    format!(\"revoked:{}\", id.trim())\n+}\n+\n pub fn revoke_session(id: &str) -> String {\n-    format!(\"revoked:{id}\")\n+    revoked_label(id)\n }\n"
}

#[test]
fn run_refactor_completes_with_recommendation_only_execution_posture() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    let brief_path = workspace.path().join("refactor.md");
    fs::write(&brief_path, complete_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "refactor",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
            "--input",
            "refactor.md",
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
        workspace.path().join(".canon").join("artifacts").join(run_id).join("refactor");

    assert_eq!(json["state"], "AwaitingApproval");
    assert_eq!(json["mode_result"]["execution_posture"].as_str(), Some("recommendation-only"));
    assert_eq!(json["mode_result"]["primary_artifact_title"].as_str(), Some("Preserved Behavior"));
    let approval_targets = json["approval_targets"].as_array().expect("approval targets array");
    assert!(
        approval_targets.iter().any(|target| target.as_str() == Some("gate:execution")),
        "approval_targets should contain gate:execution, got: {approval_targets:?}"
    );
    assert!(artifact_root.join("preserved-behavior.md").exists());
    assert!(artifact_root.join("refactor-scope.md").exists());
    assert!(artifact_root.join("no-feature-addition.md").exists());

    let preserved_behavior =
        fs::read_to_string(artifact_root.join("preserved-behavior.md")).expect("artifact");
    assert!(preserved_behavior.contains("## Preserved Behavior"));
    assert!(preserved_behavior.contains("audit ordering remain externally unchanged"));

    let structural_rationale =
        fs::read_to_string(artifact_root.join("structural-rationale.md")).expect("artifact");
    assert!(structural_rationale.contains("## Structural Rationale"));
    assert!(
        structural_rationale.contains(
            "Isolate persistence concerns and internal composition without changing externally meaningful behavior."
        )
    );

    let no_feature_addition =
        fs::read_to_string(artifact_root.join("no-feature-addition.md")).expect("artifact");
    assert!(no_feature_addition.contains("## Decision"));
    assert!(no_feature_addition.contains(
        "stop immediately if the surface expands or the packet starts to add feature semantics"
    ));

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
            "approved bounded refactor execution",
        ])
        .assert()
        .success();

    let approved_status = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let approved_status_json: serde_json::Value =
        serde_json::from_slice(&approved_status).expect("approved status json");
    assert_eq!(approved_status_json["state"], "AwaitingApproval");
    assert_eq!(
        approved_status_json["mode_result"]["execution_posture"].as_str(),
        Some("recommendation-only")
    );
    assert_eq!(approved_status_json["recommended_next_action"]["action"].as_str(), Some("resume"));
    assert!(
        approved_status_json["approval_targets"]
            .as_array()
            .is_some_and(|targets| targets.is_empty())
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

    cli_command().current_dir(workspace.path()).args(["publish", run_id]).assert().success();

    assert!(
        workspace
            .path()
            .join("docs")
            .join("refactors")
            .join(run_id)
            .join("preserved-behavior.md")
            .exists()
    );
}

#[test]
fn refactor_run_emits_missing_body_marker_for_absent_canonical_heading() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    let brief_path = workspace.path().join("refactor.md");
    fs::write(&brief_path, incomplete_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "refactor",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
            "--input",
            "refactor.md",
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
    let no_feature_addition = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(run_id)
            .join("refactor")
            .join("no-feature-addition.md"),
    )
    .expect("artifact");

    let blocked_gates = json["blocked_gates"].as_array().expect("blocked gates");

    assert_eq!(json["state"], "Blocked");
    assert_eq!(json["blocking_classification"], "artifact-blocked");
    assert_eq!(json["mode_result"]["execution_posture"].as_str(), Some("recommendation-only"));
    assert!(no_feature_addition.contains("## Missing Authored Body"));
    assert!(no_feature_addition.contains("Decision"));
    assert!(blocked_gates.iter().any(|gate| {
        gate["blockers"].as_array().is_some_and(|blockers| {
            blockers
                .iter()
                .any(|blocker| blocker.as_str().is_some_and(|text| text.contains("Decision")))
        })
    }));
}

#[test]
fn red_zone_refactor_run_remains_recommendation_only_and_publishable() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    let brief_path = workspace.path().join("refactor.md");
    fs::write(&brief_path, complete_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "refactor",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "red",
            "--owner",
            "maintainer",
            "--input",
            "refactor.md",
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
            "approved red-zone refactor execution",
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
            .join("refactors")
            .join(run_id)
            .join("preserved-behavior.md")
            .exists()
    );
}

#[test]
fn approved_refactor_resume_applies_bounded_patch_to_workspace() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);

    let packet_dir = workspace.path().join("canon-input").join("refactor");
    fs::create_dir_all(&packet_dir).expect("packet dir");
    fs::write(packet_dir.join("brief.md"), complete_brief()).expect("brief file");
    fs::write(packet_dir.join("patch.diff"), refactor_patch()).expect("patch file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "refactor",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
            "--input",
            "canon-input/refactor",
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
            "approved bounded refactor execution",
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
    assert!(session_contents.contains("fn revoked_label"));
    assert!(session_contents.contains("revoked_label(id)"));
    assert!(!session_contents.contains("format!(\"revoked:{id}\")"));
}
