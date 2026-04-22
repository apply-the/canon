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

fn complete_brief() -> &'static str {
    "# Change Brief\n\nSystem Slice: auth session boundary and persistence layer.\nLegacy Invariants: session revocation remains eventually consistent and audit log ordering stays stable.\nChange Surface: session repository, auth service, and token cleanup job.\nImplementation Plan: add bounded repository methods and preserve the public auth contract.\nValidation Strategy: contract tests, invariant checks, and rollback rehearsal.\nDecision Record: prefer additive change over normalization to preserve operator expectations.\n"
}

fn parse_run_id(output: &[u8]) -> String {
    let json: serde_json::Value = serde_json::from_slice(output).expect("json output");
    json["run_id"].as_str().expect("run id").to_string()
}

fn pending_request_id(workspace: &TempDir, global_config: &Path, run_id: &str) -> String {
    let mut command = cli_command();
    command.current_dir(workspace.path());
    for (key, value) in git_env(global_config) {
        command.env(key, value);
    }

    let output = command
        .args(["inspect", "invocations", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    json["entries"]
        .as_array()
        .and_then(|entries| {
            entries.iter().find_map(|entry| {
                if entry["policy_decision"] == "NeedsApproval" {
                    entry["request_id"].as_str().map(ToString::to_string)
                } else {
                    None
                }
            })
        })
        .expect("pending request")
}

fn approval_record_by(workspace: &TempDir, run_id: &str) -> String {
    let record = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("runs")
            .join(run_id)
            .join("approvals")
            .join("approval-00.toml"),
    )
    .expect("approval record");

    record
        .lines()
        .find_map(|line| line.strip_prefix("by = \""))
        .and_then(|line| line.strip_suffix('"'))
        .map(ToString::to_string)
        .expect("approver in approval record")
}

fn approval_summary_json(output: &[u8]) -> serde_json::Value {
    serde_json::from_slice(output).expect("approval json")
}

fn start_gated_change_run(workspace: &TempDir, global_config: &Path) -> (String, String) {
    let brief_path = workspace.path().join("change.md");
    fs::write(&brief_path, complete_brief()).expect("brief file");

    let mut command = cli_command();
    command.current_dir(workspace.path());
    for (key, value) in git_env(global_config) {
        command.env(key, value);
    }

    let output = command
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
            brief_path.file_name().expect("file name").to_str().expect("utf8"),
            "--output",
            "json",
        ])
        .assert()
        .code(3)
        .get_output()
        .stdout
        .clone();

    let run_id = parse_run_id(&output);
    let request_id = pending_request_id(workspace, global_config, &run_id);
    (run_id, request_id)
}

fn approve_run(
    workspace: &TempDir,
    global_config: &Path,
    run_id: &str,
    request_id: &str,
    extra_args: &[&str],
) -> assert_cmd::assert::Assert {
    let mut command = cli_command();
    command.current_dir(workspace.path());
    for (key, value) in git_env(global_config) {
        command.env(key, value);
    }

    command
        .args([
            "approve",
            "--run",
            run_id,
            "--target",
            &format!("invocation:{request_id}"),
            "--decision",
            "approve",
            "--rationale",
            "Systemic work remains recommendation-only and the preserved surface is explicit.",
        ])
        .args(extra_args)
        .assert()
}

#[test]
fn explicit_approver_overrides_git_identity() {
    let workspace = TempDir::new().expect("temp dir");
    let global_config = workspace.path().join("gitconfig-global");
    fs::write(&global_config, "").expect("global config file");

    git(&workspace, &global_config, &["init", "-b", "main"]);
    git(&workspace, &global_config, &["config", "user.name", "Local Approver"]);
    git(&workspace, &global_config, &["config", "user.email", "local-approver@example.com"]);

    let (run_id, request_id) = start_gated_change_run(&workspace, &global_config);

    let output = approve_run(
        &workspace,
        &global_config,
        &run_id,
        &request_id,
        &["--by", "Explicit Approver <explicit@example.com>"],
    )
    .success()
    .get_output()
    .stdout
    .clone();

    let json = approval_summary_json(&output);
    assert_eq!(json["approved_by"], "Explicit Approver <explicit@example.com>");
    assert!(
        json["recorded_at"].as_str().is_some_and(|value| !value.is_empty()),
        "approval summary should expose a non-empty recorded_at timestamp"
    );

    assert_eq!(approval_record_by(&workspace, &run_id), "Explicit Approver <explicit@example.com>");
}

#[test]
fn approve_uses_local_git_identity_when_by_is_omitted() {
    let workspace = TempDir::new().expect("temp dir");
    let global_config = workspace.path().join("gitconfig-global");
    fs::write(&global_config, "").expect("global config file");

    git(&workspace, &global_config, &["init", "-b", "main"]);
    git(&workspace, &global_config, &["config", "user.name", "Local Approver"]);
    git(&workspace, &global_config, &["config", "user.email", "local-approver@example.com"]);

    let (run_id, request_id) = start_gated_change_run(&workspace, &global_config);

    let output = approve_run(&workspace, &global_config, &run_id, &request_id, &[])
        .success()
        .get_output()
        .stdout
        .clone();

    let json = approval_summary_json(&output);
    assert_eq!(json["approved_by"], "Local Approver <local-approver@example.com>");
    assert!(
        json["recorded_at"].as_str().is_some_and(|value| !value.is_empty()),
        "approval summary should expose a non-empty recorded_at timestamp"
    );

    assert_eq!(
        approval_record_by(&workspace, &run_id),
        "Local Approver <local-approver@example.com>"
    );
}

#[test]
fn approve_uses_global_git_identity_when_local_identity_is_missing() {
    let workspace = TempDir::new().expect("temp dir");
    let global_config = workspace.path().join("gitconfig-global");
    fs::write(&global_config, "").expect("global config file");

    git(&workspace, &global_config, &["init", "-b", "main"]);

    let mut global_name = ProcessCommand::new("git");
    global_name.args(["config", "--global", "user.name", "Global Approver"]);
    for (key, value) in git_env(&global_config) {
        global_name.env(key, value);
    }
    let output = global_name.output().expect("set global name");
    assert!(output.status.success(), "set global name failed");

    let mut global_email = ProcessCommand::new("git");
    global_email.args(["config", "--global", "user.email", "global-approver@example.com"]);
    for (key, value) in git_env(&global_config) {
        global_email.env(key, value);
    }
    let output = global_email.output().expect("set global email");
    assert!(output.status.success(), "set global email failed");

    let (run_id, request_id) = start_gated_change_run(&workspace, &global_config);

    let output = approve_run(&workspace, &global_config, &run_id, &request_id, &[])
        .success()
        .get_output()
        .stdout
        .clone();

    let json = approval_summary_json(&output);
    assert_eq!(json["approved_by"], "Global Approver <global-approver@example.com>");
    assert!(
        json["recorded_at"].as_str().is_some_and(|value| !value.is_empty()),
        "approval summary should expose a non-empty recorded_at timestamp"
    );

    assert_eq!(
        approval_record_by(&workspace, &run_id),
        "Global Approver <global-approver@example.com>"
    );
}

#[test]
fn approve_without_by_or_git_identity_fails_with_guidance() {
    let workspace = TempDir::new().expect("temp dir");
    let global_config = workspace.path().join("gitconfig-global");
    fs::write(&global_config, "").expect("global config file");

    git(&workspace, &global_config, &["init", "-b", "main"]);

    let (run_id, request_id) = start_gated_change_run(&workspace, &global_config);

    approve_run(&workspace, &global_config, &run_id, &request_id, &[])
        .failure()
        .stderr(contains("pass --by or configure git user.name and user.email"));
}
