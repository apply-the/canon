use std::fs;
use std::path::PathBuf;

use assert_cmd::Command;
use tempfile::TempDir;

const TEST_INTERACTIVE_ENV: &str = "CANON_TUI_TEST_INTERACTIVE";
const TEST_SIZE_ENV: &str = "CANON_TUI_TEST_SIZE";
const TEST_EVENTS_ENV: &str = "CANON_TUI_TEST_EVENTS";
const TEST_CAPTURE_PATH_ENV: &str = "CANON_TUI_TEST_CAPTURE_PATH";
const DEFAULT_TERMINAL_SIZE: &str = "120x40";
const COMPACT_TERMINAL_SIZE: &str = "42x11";
const RESTORE_MARKER: &str = "terminal_restored=true";
const SCREEN_RUNTIME_TITLE: &str = "AI-Assisted Engineering Governance Runtime";
const SCREEN_VERSION_PREFIX: &str = "Version ";

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

fn guided_command_with_size(
    workspace: &TempDir,
    capture_name: &str,
    events: &str,
    size: &str,
) -> (Command, PathBuf) {
    let capture_path = workspace.path().join(capture_name);
    let mut command = cli_command();
    command
        .current_dir(workspace.path())
        .env(TEST_INTERACTIVE_ENV, "1")
        .env(TEST_SIZE_ENV, size)
        .env(TEST_EVENTS_ENV, events)
        .env(TEST_CAPTURE_PATH_ENV, &capture_path);
    (command, capture_path)
}

fn guided_command(workspace: &TempDir, capture_name: &str, events: &str) -> (Command, PathBuf) {
    guided_command_with_size(workspace, capture_name, events, DEFAULT_TERMINAL_SIZE)
}

#[test]
fn init_launches_guided_ui_by_default_in_supported_terminal() {
    let workspace = TempDir::new().expect("temp dir");
    let (mut command, capture_path) =
        guided_command(&workspace, "guided-default.log", "enter,enter");

    command.arg("init").assert().success();

    let capture = fs::read_to_string(&capture_path).expect("guided capture");
    assert!(capture.contains(SCREEN_RUNTIME_TITLE));
    assert!(capture.contains(SCREEN_VERSION_PREFIX));
    assert!(capture.contains("Select an assistant for this workspace."));
    assert!(capture.contains(RESTORE_MARKER));
    assert!(workspace.path().join(".canon").is_dir());
}

#[test]
fn init_launches_guided_ui_by_default_in_compact_supported_terminal() {
    let workspace = TempDir::new().expect("temp dir");
    let (mut command, capture_path) = guided_command_with_size(
        &workspace,
        "guided-compact.log",
        "enter,enter",
        COMPACT_TERMINAL_SIZE,
    );

    command.arg("init").assert().success();

    let capture = fs::read_to_string(&capture_path).expect("guided capture");
    assert!(capture.contains(SCREEN_RUNTIME_TITLE));
    assert!(capture.contains(RESTORE_MARKER));
    assert!(workspace.path().join(".canon").is_dir());
}

#[test]
fn init_honors_ai_preselection_inside_guided_flow() {
    let workspace = TempDir::new().expect("temp dir");
    let (mut command, capture_path) =
        guided_command(&workspace, "guided-preselected.log", "enter,enter");

    command.args(["init", "--ai", "copilot"]).assert().success();

    let capture = fs::read_to_string(&capture_path).expect("guided capture");
    assert!(capture.contains("> Copilot"));
    assert!(capture.contains(RESTORE_MARKER));
    assert!(workspace.path().join(".canon").is_dir());
}

#[test]
fn init_honors_cursor_and_antigravity_preselection_inside_guided_flow() {
    let cursor_workspace = TempDir::new().expect("temp dir");
    let (mut cursor_command, cursor_capture_path) =
        guided_command(&cursor_workspace, "guided-preselected-cursor.log", "enter,enter");

    cursor_command.args(["init", "--ai", "cursor"]).assert().success();

    let cursor_capture = fs::read_to_string(&cursor_capture_path).expect("guided capture");
    assert!(cursor_capture.contains("> Cursor"));
    assert!(cursor_capture.contains(RESTORE_MARKER));
    assert!(cursor_workspace.path().join(".canon").is_dir());

    let antigravity_workspace = TempDir::new().expect("temp dir");
    let (mut antigravity_command, antigravity_capture_path) =
        guided_command(&antigravity_workspace, "guided-preselected-antigravity.log", "enter,enter");

    antigravity_command.args(["init", "--ai", "antigravity"]).assert().success();

    let antigravity_capture =
        fs::read_to_string(&antigravity_capture_path).expect("guided capture");
    assert!(antigravity_capture.contains("> Antigravity"));
    assert!(antigravity_capture.contains(RESTORE_MARKER));
    assert!(antigravity_workspace.path().join(".canon").is_dir());
}

#[test]
fn guided_flow_reaches_claude_within_ten_keypresses() {
    let workspace = TempDir::new().expect("temp dir");
    let events = ["down", "down", "down", "enter", "enter"];
    assert!(events.len() <= 10);
    let (mut command, capture_path) =
        guided_command(&workspace, "guided-keypress-budget.log", &events.join(","));

    command.arg("init").assert().success();

    let capture = fs::read_to_string(&capture_path).expect("guided capture");
    assert!(capture.contains("> Claude"));
    assert!(capture.contains("Initialize with Claude."));
    assert!(capture.contains(RESTORE_MARKER));
}
