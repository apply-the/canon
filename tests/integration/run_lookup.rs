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

fn init_workspace() -> TempDir {
    let workspace = tempfile::tempdir().expect("tempdir");
    cli_command()
        .current_dir(workspace.path())
        .args(["init", "--output", "json"])
        .assert()
        .success();
    workspace
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

fn init_existing_repo(workspace: &TempDir) {
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

fn create_requirements_run(workspace: &TempDir) -> String {
    let input_dir = workspace.path().join("canon-input");
    fs::create_dir_all(&input_dir).unwrap();
    let unique = format!("Idea {}", uuid::Uuid::now_v7().as_simple());
    fs::write(input_dir.join("idea.md"), format!("# {unique}\n\nA brief.\n")).unwrap();

    let assert = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "requirements",
            "--risk",
            "low-impact",
            "--zone",
            "green",
            "--owner",
            "Owner <owner@example.com>",
            "--input",
            "canon-input/idea.md",
            "--output",
            "json",
        ])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("run json");
    json["run_id"].as_str().expect("run_id").to_string()
}

#[test]
fn run_creation_emits_canonical_display_id() {
    let workspace = init_workspace();
    let run_id = create_requirements_run(&workspace);
    assert!(
        run_id.starts_with("R-") && run_id.len() == 19,
        "run_id should look like R-YYYYMMDD-XXXXXXXX, got `{run_id}`"
    );
}

#[test]
fn status_resolves_short_id_prefix() {
    let workspace = init_workspace();
    let run_id = create_requirements_run(&workspace);
    let short = &run_id[run_id.len() - 8..];

    cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", short, "--output", "json"])
        .assert()
        .success();
}

#[test]
fn list_runs_reports_empty_workspace_in_text_format() {
    let workspace = init_workspace();

    let assert =
        cli_command().current_dir(workspace.path()).args(["list", "runs"]).assert().success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert_eq!(stdout.trim(), "(no runs)");
}

#[test]
fn list_runs_returns_structured_json_output() {
    let workspace = init_workspace();
    let run_id = create_requirements_run(&workspace);

    let assert = cli_command()
        .current_dir(workspace.path())
        .args(["list", "runs", "--output", "json"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let runs: Vec<serde_json::Value> = serde_json::from_str(&stdout).expect("list runs json");

    assert_eq!(runs.len(), 1, "expected one visible run in list output: {stdout}");
    assert_eq!(runs[0]["run_id"], run_id);
    assert_eq!(runs[0]["short_id"].as_str().map(str::len), Some(8));
    assert!(runs[0]["created_at"].as_str().is_some(), "expected created_at string: {stdout}");
    assert!(runs[0]["is_legacy"].is_boolean(), "expected is_legacy boolean: {stdout}");
}

#[test]
fn list_runs_renders_text_table_for_existing_runs() {
    let workspace = init_workspace();
    let run_id = create_requirements_run(&workspace);

    let assert =
        cli_command().current_dir(workspace.path()).args(["list", "runs"]).assert().success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(stdout.contains("RUN_ID"), "expected header row: {stdout}");
    assert!(stdout.contains("SHORT_ID"), "expected header row: {stdout}");
    assert!(stdout.contains(&run_id), "expected run in table: {stdout}");
}

#[test]
fn list_runs_supports_yaml_output() {
    let workspace = init_workspace();
    let run_id = create_requirements_run(&workspace);

    let assert = cli_command()
        .current_dir(workspace.path())
        .args(["list", "runs", "--output", "yaml"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(
        stdout.contains(&format!("run_id: {run_id}")),
        "expected run_id in yaml output: {stdout}"
    );
    assert!(stdout.contains("created_at:"), "expected created_at field in yaml output: {stdout}");
}

#[test]
fn publish_accepts_last_alias_and_writes_default_destination() {
    let workspace = init_workspace();
    let run_id = create_requirements_run(&workspace);

    cli_command().current_dir(workspace.path()).args(["publish", "@last"]).assert().success();

    assert!(workspace.path().join("specs").join(run_id).join("problem-statement.md").exists());
}

#[test]
fn publish_accepts_short_id_prefix_and_explicit_destination() {
    let workspace = init_workspace();
    let run_id = create_requirements_run(&workspace);
    let short = &run_id[run_id.len() - 8..];

    cli_command()
        .current_dir(workspace.path())
        .args(["publish", short, "--to", "docs/public/prd"])
        .assert()
        .success();

    assert!(workspace.path().join("docs/public/prd").join("problem-statement.md").exists());
}

#[test]
fn recommendation_only_implementation_runs_remain_resolvable_via_last_alias() {
    let workspace = tempfile::tempdir().expect("tempdir");
    init_existing_repo(&workspace);
    fs::write(
        workspace.path().join("implementation.md"),
        "# Implementation Brief\n\nTask Mapping: 1. Add bounded auth session repository helpers.\n2. Thread the helper through the revocation service without expanding the public API.\nMutation Bounds: src/auth/session.rs; src/auth/repository.rs\nAllowed Paths:\n- src/auth/session.rs\n- src/auth/repository.rs\nSafety-Net Evidence: contract coverage protects revocation formatting and audit ordering before mutation.\nIndependent Checks: cargo test --test session_contract\nRollback Triggers: revocation output drifts or audit ordering becomes unstable.\nRollback Steps: revert the bounded auth-session patch and redeploy the previous build.\n",
    )
    .expect("implementation brief");

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
    let json: serde_json::Value = serde_json::from_slice(&output).expect("run json");
    let run_id = json["run_id"].as_str().expect("run_id");

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

    cli_command()
        .current_dir(workspace.path())
        .args(["resume", "--run", run_id])
        .assert()
        .success();

    cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", "@last", "--output", "json"])
        .assert()
        .success()
        .stdout(predicates::str::contains(format!("\"run\": \"{run_id}\"")));

    cli_command().current_dir(workspace.path()).args(["publish", "@last"]).assert().success();

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
