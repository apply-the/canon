use std::fs;
use std::path::PathBuf;

use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

const TEST_INTERACTIVE_ENV: &str = "CANON_TUI_TEST_INTERACTIVE";
const TEST_SIZE_ENV: &str = "CANON_TUI_TEST_SIZE";
const TEST_EVENTS_ENV: &str = "CANON_TUI_TEST_EVENTS";
const TEST_CAPTURE_PATH_ENV: &str = "CANON_TUI_TEST_CAPTURE_PATH";
const DEFAULT_TERMINAL_SIZE: &str = "120x40";
const RESTORE_MARKER: &str = "terminal_restored=true";

fn cli_command() -> Command {
    if let Some(binary) = std::env::var_os("CARGO_BIN_EXE_canon") {
        return Command::new(binary);
    }

    let workspace_target = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"));
    let candidate =
        workspace_target.join("debug").join(format!("canon{}", std::env::consts::EXE_SUFFIX));
    if candidate.exists() {
        return Command::new(candidate);
    }

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

fn guided_command(workspace: &TempDir, capture_name: &str, events: &str) -> (Command, PathBuf) {
    let capture_path = workspace.path().join(capture_name);
    let mut command = cli_command();
    command
        .current_dir(workspace.path())
        .env(TEST_INTERACTIVE_ENV, "1")
        .env(TEST_SIZE_ENV, DEFAULT_TERMINAL_SIZE)
        .env(TEST_EVENTS_ENV, events)
        .env(TEST_CAPTURE_PATH_ENV, &capture_path);
    (command, capture_path)
}

#[test]
fn ctrl_c_restores_terminal_and_avoids_side_effects() {
    let workspace = TempDir::new().expect("temp dir");
    let (mut command, capture_path) = guided_command(&workspace, "guided-ctrl-c.log", "ctrl-c");

    command
        .arg("init")
        .assert()
        .failure()
        .stderr(contains("guided init interrupted before initialization"));

    let capture = fs::read_to_string(&capture_path).expect("guided capture");
    assert!(capture.contains(RESTORE_MARKER));
    assert!(!workspace.path().join(".canon").exists());
}

#[test]
fn guided_init_failure_restores_terminal_before_returning_error() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(workspace.path().join(".canon"), "blocking file").expect("blocking .canon file");
    let (mut command, capture_path) =
        guided_command(&workspace, "guided-failure.log", "enter,enter");

    command.arg("init").assert().failure();

    let capture = fs::read_to_string(&capture_path).expect("guided capture");
    assert!(capture.contains(RESTORE_MARKER));
    assert!(workspace.path().join(".canon").is_file());
}

#[test]
fn too_small_layout_rejects_before_guided_terminal_setup() {
    let workspace = TempDir::new().expect("temp dir");

    cli_command()
        .current_dir(workspace.path())
        .env(TEST_INTERACTIVE_ENV, "1")
        .env(TEST_SIZE_ENV, "40x10")
        .arg("init")
        .assert()
        .failure()
        .stderr(contains("layout is too small"));

    assert!(!workspace.path().join(".canon").exists());
}
