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

#[test]
fn inspect_risk_zone_returns_a_confirmation_payload() {
    let workspace = tempfile::TempDir::new().expect("temp dir");
    std::fs::write(
        workspace.path().join("discovery.md"),
        "# Discovery Brief\n\nProblem: production boundary drift.\nConstraints: preserve repo-local evidence.\n",
    )
    .expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "risk-zone",
            "--mode",
            "discovery",
            "--input",
            "discovery.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let entry = json["entries"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("classification entry");

    assert_eq!(entry["mode"].as_str(), Some("discovery"));
    assert!(entry["risk"].as_str().is_some());
    assert!(entry["zone"].as_str().is_some());
    assert_eq!(entry["requires_confirmation"].as_bool(), Some(true));
    assert!(entry["confidence"].as_str().is_some());
}
