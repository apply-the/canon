use std::path::PathBuf;

use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

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

#[test]
fn init_non_interactive_preserves_json_summary_output() {
    let workspace = TempDir::new().expect("temp dir");

    let output = cli_command()
        .current_dir(workspace.path())
        .args(["init", "--non-interactive", "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("init json");
    assert!(json["canon_root"].as_str().is_some_and(|value| value.ends_with("/.canon")));
    assert!(workspace.path().join(".canon").is_dir());
}

#[test]
fn init_non_interactive_passes_ai_selection_to_backend() {
    let workspace = TempDir::new().expect("temp dir");

    let output = cli_command()
        .current_dir(workspace.path())
        .args(["init", "--non-interactive", "--ai", "copilot", "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("init json");
    assert!(json["skills_materialized"].as_u64().is_some_and(|value| value > 0));
    assert!(workspace.path().join(".agents").is_dir());
}

#[test]
fn init_non_interactive_supports_cursor_and_antigravity_targets() {
    for ai in ["cursor", "antigravity"] {
        let workspace = TempDir::new().expect("temp dir");

        let output = cli_command()
            .current_dir(workspace.path())
            .args(["init", "--non-interactive", "--ai", ai, "--output", "json"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let json: serde_json::Value = serde_json::from_slice(&output).expect("init json");
        assert!(json["skills_materialized"].as_u64().is_some_and(|value| value > 0));
        assert!(workspace.path().join(".agents").is_dir());
    }
}

#[test]
fn init_falls_back_to_non_interactive_when_tty_is_unavailable() {
    let workspace = TempDir::new().expect("temp dir");

    cli_command()
        .current_dir(workspace.path())
        .arg("init")
        .assert()
        .success()
        .stdout(contains("\"canon_root\""));

    assert!(workspace.path().join(".canon").is_dir());
}

#[test]
fn structured_output_without_non_interactive_is_rejected_even_without_tty() {
    let workspace = TempDir::new().expect("temp dir");

    cli_command()
        .current_dir(workspace.path())
        .args(["init", "--output", "json"])
        .assert()
        .failure()
        .stderr(contains("requires --non-interactive"));

    assert!(!workspace.path().join(".canon").exists());
}
