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
fn inspect_modes_returns_the_frozen_stable_profile_registry() {
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
            "architecture",
            "backlog",
            "change",
            "refactor",
            "verification",
            "pr-review",
            "incident",
        ])
    );
}

#[test]
fn inspect_modes_text_output_excludes_nonstable_and_implementation_modes() {
    let output =
        cli_command().args(["inspect", "modes"]).assert().success().get_output().stdout.clone();

    let text = String::from_utf8(output).expect("utf8 output");

    assert!(text.contains("backlog"));
    assert!(text.contains("refactor"));
    assert!(text.contains("incident"));
    assert!(text.contains("verification"));
    assert!(text.contains("pr-review"));
    assert!(!text.contains("implementation"));
    assert!(!text.contains("system-assessment"));
    assert!(!text.contains("migration"));
    assert!(!text.contains("security-assessment"));
    assert!(!text.contains("supply-chain-analysis"));
    assert!(!text.contains("debugging"));
    assert!(!text.contains("brainstorming"));
    assert_eq!(text.matches("incident").count(), 1);
}
