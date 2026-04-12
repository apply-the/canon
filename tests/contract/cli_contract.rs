use assert_cmd::Command;
use predicates::str::contains;

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
fn help_lists_the_expected_top_level_commands() {
    let mut command = cli_command();
    command
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("init"))
        .stdout(contains("run"))
        .stdout(contains("resume"))
        .stdout(contains("status"))
        .stdout(contains("approve"))
        .stdout(contains("verify"))
        .stdout(contains("inspect"))
        .stdout(contains("skills"));
}
