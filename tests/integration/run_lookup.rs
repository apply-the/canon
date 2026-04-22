use std::fs;

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
fn list_runs_returns_recent_run() {
    let workspace = init_workspace();
    let run_id = create_requirements_run(&workspace);
    let assert = cli_command()
        .current_dir(workspace.path())
        .args(["list", "runs", "--output", "json"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(stdout.contains(&run_id), "list runs should include `{run_id}`, got: {stdout}");
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
