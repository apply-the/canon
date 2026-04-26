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

fn blocked_brief() -> &'static str {
    "# Refactor Brief\n\n## Refactor Scope\nAuth session boundary and repository composition only.\n\n## Allowed Paths\n- src/auth/session.rs\n- src/auth/repository.rs\n\n## Structural Rationale\nIsolate persistence concerns without changing externally meaningful behavior.\n"
}

#[test]
fn run_refactor_blocks_when_preservation_and_feature_audit_inputs_are_missing() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    let brief_path = workspace.path().join("refactor.md");
    fs::write(&brief_path, blocked_brief()).expect("brief file");

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
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("refactor");
    let preserved_behavior =
        fs::read_to_string(artifact_root.join("preserved-behavior.md")).expect("artifact");
    let no_feature_addition =
        fs::read_to_string(artifact_root.join("no-feature-addition.md")).expect("artifact");

    assert_eq!(json["state"], "Blocked");
    assert_eq!(json["mode_result"]["execution_posture"].as_str(), Some("recommendation-only"));
    assert_eq!(json["mode_result"]["primary_artifact_title"].as_str(), Some("Preserved Behavior"));
    assert!(
        json["mode_result"]["headline"]
            .as_str()
            .is_some_and(|value| value.contains("missing-context marker"))
    );
    assert!(artifact_root.join("preserved-behavior.md").exists());
    assert!(artifact_root.join("no-feature-addition.md").exists());
    assert!(preserved_behavior.contains("## Missing Authored Body"));
    assert!(preserved_behavior.contains("Preserved Behavior"));
    assert!(no_feature_addition.contains("## Missing Authored Body"));
    assert!(no_feature_addition.contains("Feature Audit"));
}
