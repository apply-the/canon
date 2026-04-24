use assert_cmd::Command;

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
fn inspect_modes_returns_the_full_mode_taxonomy() {
    let output = cli_command()
        .args(["inspect", "modes", "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    assert_eq!(json["target"], "modes");
    assert_eq!(
        json["entries"],
        serde_json::json!([
            "discovery",
            "requirements",
            "system-shaping",
            "architecture",
            "change",
            "backlog",
            "pr-review",
            "implementation",
            "refactor",
            "verification",
            "review",
            "incident",
            "migration",
        ])
    );
}

#[test]
fn inspect_modes_text_output_keeps_execution_heavy_modes_visible() {
    let output =
        cli_command().args(["inspect", "modes"]).assert().success().get_output().stdout.clone();

    let text = String::from_utf8(output).expect("utf8 output");

    assert!(text.contains("backlog"));
    assert!(text.contains("implementation"));
    assert!(text.contains("refactor"));
}
