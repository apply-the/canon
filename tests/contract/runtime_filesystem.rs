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

#[test]
fn init_materializes_the_runtime_contract_layout() {
    let workspace = TempDir::new().expect("temp dir");

    let mut command = cli_command();
    command.current_dir(workspace.path()).arg("init").assert().success();

    let canon = workspace.path().join(".canon");
    assert!(canon.exists(), ".canon should exist after init");
    assert!(canon.join("methods").exists(), "methods directory should exist");
    assert!(canon.join("policies").exists(), "policies directory should exist");
    assert!(canon.join("runs").exists(), "runs directory should exist");
    assert!(canon.join("artifacts").exists(), "artifacts directory should exist");
    assert!(
        canon.join("methods").join("backlog.toml").exists(),
        "backlog method file should exist"
    );
    assert!(
        canon.join("methods").join("requirements.toml").exists(),
        "requirements method file should exist"
    );
    assert!(canon.join("policies").join("risk.toml").exists(), "risk policy file should exist");

    let run_entries = fs::read_dir(canon.join("runs")).expect("runs dir is readable").count();
    assert_eq!(run_entries, 0, "init should not create runs");
}
