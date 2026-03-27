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

#[test]
fn init_is_idempotent_and_creates_runtime_scaffolding() {
    let workspace = TempDir::new().expect("temp dir");

    let mut first = cli_command();
    first.current_dir(workspace.path()).arg("init").assert().success();

    let canon = workspace.path().join(".canon");
    assert!(canon.is_dir(), ".canon should be created");
    assert!(canon.join("sessions").is_dir(), "sessions directory should exist");
    assert!(canon.join("decisions").is_dir(), "decisions directory should exist");
    assert!(canon.join("traces").is_dir(), "traces directory should exist");

    let mut second = cli_command();
    second.current_dir(workspace.path()).arg("init").assert().success();

    assert!(canon.join("methods").join("pr-review.toml").exists());
    assert!(canon.join("policies").join("adapters.toml").exists());
}
