use std::fs;

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

fn run_requirements_flow(workspace: &TempDir) -> String {
    let idea_path = workspace.path().join("idea.md");
    fs::write(
        &idea_path,
        "# Idea\n\nBound AI-assisted engineering work with explicit governance.\n",
    )
    .expect("idea file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "requirements",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "product-lead",
            "--input",
            idea_path.file_name().expect("file name").to_str().expect("utf8"),
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).expect("utf8 stdout");
    let json: serde_json::Value = serde_json::from_str(&text).expect("json output");
    json["run_id"].as_str().expect("run id").to_string()
}

#[test]
fn inspect_artifacts_lists_the_requirements_bundle() {
    let workspace = TempDir::new().expect("temp dir");
    let run_id = run_requirements_flow(&workspace);

    let inspect_output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "artifacts", "--run", &run_id, "--output", "json"])
        .assert()
        .success()
        .stdout(contains("problem-statement.md"))
        .stdout(contains("decision-checklist.md"))
        .get_output()
        .stdout
        .clone();

    let inspect_text = String::from_utf8(inspect_output).expect("utf8 stdout");
    let inspect_json: serde_json::Value = serde_json::from_str(&inspect_text).expect("json output");
    let inspect_snapshot = serde_json::to_string_pretty(&inspect_json).expect("json snapshot");
    assert_eq!(
        inspect_snapshot.trim(),
        include_str!("snapshots/requirements_artifact_inspect.json").trim()
    );

    let contract_path =
        workspace.path().join(".canon").join("runs").join(&run_id).join("artifact-contract.toml");
    let contract_toml = fs::read_to_string(contract_path).expect("artifact contract");
    assert_eq!(
        contract_toml.trim(),
        include_str!("snapshots/requirements_artifact_contract.toml").trim()
    );
}
