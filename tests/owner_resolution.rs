use std::fs;
use std::path::Path;
use std::process::Command as ProcessCommand;

use assert_cmd::Command;
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

fn git_env(global_config: &Path) -> [(&'static str, String); 2] {
    [
        ("GIT_CONFIG_GLOBAL", global_config.display().to_string()),
        ("GIT_CONFIG_NOSYSTEM", "1".to_string()),
    ]
}

fn git(workspace: &TempDir, global_config: &Path, args: &[&str]) {
    let mut command = ProcessCommand::new("git");
    command.args(args).current_dir(workspace.path());
    for (key, value) in git_env(global_config) {
        command.env(key, value);
    }

    let output = command.output().expect("git command");
    assert!(
        output.status.success(),
        "git {:?} failed: stdout=`{}` stderr=`{}`",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn manifest_owner(workspace: &TempDir, run_id: &str) -> String {
    let manifest = fs::read_to_string(
        workspace.path().join(".canon").join("runs").join(run_id).join("run.toml"),
    )
    .expect("run manifest");

    manifest
        .lines()
        .find_map(|line| line.strip_prefix("owner = \""))
        .and_then(|line| line.strip_suffix('"'))
        .map(ToString::to_string)
        .expect("owner in run manifest")
}

fn status_json(workspace: &TempDir, global_config: &Path, run_id: &str) -> serde_json::Value {
    let mut command = cli_command();
    command.current_dir(workspace.path());
    for (key, value) in git_env(global_config) {
        command.env(key, value);
    }

    let output = command
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    serde_json::from_slice(&output).expect("status json")
}

fn run_requirements(
    workspace: &TempDir,
    global_config: &Path,
    extra_args: &[&str],
) -> assert_cmd::assert::Assert {
    let mut command = cli_command();
    command.current_dir(workspace.path());
    for (key, value) in git_env(global_config) {
        command.env(key, value);
    }

    command
        .args([
            "run",
            "--mode",
            "requirements",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--input",
            "idea.md",
            "--output",
            "json",
        ])
        .args(extra_args)
        .assert()
}

#[test]
fn explicit_owner_overrides_git_identity() {
    let workspace = TempDir::new().expect("temp dir");
    let global_config = workspace.path().join("gitconfig-global");
    fs::write(&global_config, "").expect("global config file");
    fs::write(workspace.path().join("idea.md"), "Need bounded framing.\n").expect("idea file");

    git(&workspace, &global_config, &["init", "-b", "main"]);
    git(&workspace, &global_config, &["config", "user.name", "Local Owner"]);
    git(&workspace, &global_config, &["config", "user.email", "local@example.com"]);

    let output = run_requirements(
        &workspace,
        &global_config,
        &["--owner", "Explicit Owner <explicit@example.com>"],
    )
    .success()
    .get_output()
    .stdout
    .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("run json");
    let run_id = json["run_id"].as_str().expect("run id");
    assert_eq!(json["owner"], "Explicit Owner <explicit@example.com>");
    assert_eq!(manifest_owner(&workspace, run_id), "Explicit Owner <explicit@example.com>");

    let status = status_json(&workspace, &global_config, run_id);
    assert_eq!(status["owner"], "Explicit Owner <explicit@example.com>");
}

#[test]
fn run_uses_local_git_identity_when_owner_is_omitted() {
    let workspace = TempDir::new().expect("temp dir");
    let global_config = workspace.path().join("gitconfig-global");
    fs::write(&global_config, "").expect("global config file");
    fs::write(workspace.path().join("idea.md"), "Need bounded framing.\n").expect("idea file");

    git(&workspace, &global_config, &["init", "-b", "main"]);
    git(&workspace, &global_config, &["config", "user.name", "Local Owner"]);
    git(&workspace, &global_config, &["config", "user.email", "local@example.com"]);

    let output =
        run_requirements(&workspace, &global_config, &[]).success().get_output().stdout.clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("run json");
    let run_id = json["run_id"].as_str().expect("run id");
    assert_eq!(json["owner"], "Local Owner <local@example.com>");
    assert_eq!(manifest_owner(&workspace, run_id), "Local Owner <local@example.com>");

    let status = status_json(&workspace, &global_config, run_id);
    assert_eq!(status["owner"], "Local Owner <local@example.com>");
}

#[test]
fn run_uses_global_git_identity_when_local_identity_is_missing() {
    let workspace = TempDir::new().expect("temp dir");
    let global_config = workspace.path().join("gitconfig-global");
    fs::write(&global_config, "").expect("global config file");
    fs::write(workspace.path().join("idea.md"), "Need bounded framing.\n").expect("idea file");

    git(&workspace, &global_config, &["init", "-b", "main"]);

    let mut global_name = ProcessCommand::new("git");
    global_name.args(["config", "--global", "user.name", "Global Owner"]);
    for (key, value) in git_env(&global_config) {
        global_name.env(key, value);
    }
    let output = global_name.output().expect("set global name");
    assert!(output.status.success(), "set global name failed");

    let mut global_email = ProcessCommand::new("git");
    global_email.args(["config", "--global", "user.email", "global@example.com"]);
    for (key, value) in git_env(&global_config) {
        global_email.env(key, value);
    }
    let output = global_email.output().expect("set global email");
    assert!(output.status.success(), "set global email failed");

    let output =
        run_requirements(&workspace, &global_config, &[]).success().get_output().stdout.clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("run json");
    let run_id = json["run_id"].as_str().expect("run id");
    assert_eq!(json["owner"], "Global Owner <global@example.com>");
    assert_eq!(manifest_owner(&workspace, run_id), "Global Owner <global@example.com>");

    let status = status_json(&workspace, &global_config, run_id);
    assert_eq!(status["owner"], "Global Owner <global@example.com>");
}

#[test]
fn bounded_impact_run_without_owner_or_git_identity_fails_with_guidance() {
    let workspace = TempDir::new().expect("temp dir");
    let global_config = workspace.path().join("gitconfig-global");
    fs::write(&global_config, "").expect("global config file");
    fs::write(workspace.path().join("idea.md"), "Need bounded framing.\n").expect("idea file");

    git(&workspace, &global_config, &["init", "-b", "main"]);

    run_requirements(&workspace, &global_config, &[])
        .failure()
        .stderr(contains("pass --owner or configure git user.name and user.email"));
}
