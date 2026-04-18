use std::fs;

use assert_cmd::Command;
use predicates::prelude::PredicateBooleanExt;
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
    let entries = inspect_json["entries"].as_array().expect("artifact entries");
    let actual_paths =
        entries.iter().map(|entry| entry.as_str().expect("artifact path")).collect::<Vec<_>>();
    let expected_paths = vec![
        format!(".canon/artifacts/{run_id}/requirements/constraints.md"),
        format!(".canon/artifacts/{run_id}/requirements/decision-checklist.md"),
        format!(".canon/artifacts/{run_id}/requirements/options.md"),
        format!(".canon/artifacts/{run_id}/requirements/problem-statement.md"),
        format!(".canon/artifacts/{run_id}/requirements/scope-cuts.md"),
        format!(".canon/artifacts/{run_id}/requirements/tradeoffs.md"),
    ];
    assert_eq!(actual_paths, expected_paths);

    cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "artifacts", "--run", &run_id])
        .assert()
        .success()
        .stdout(contains("# artifacts"))
        .stdout(contains(format!("Run ID: {run_id}")))
        .stdout(contains(format!(".canon/artifacts/{run_id}/requirements/problem-statement.md")));

    let contract_path =
        workspace.path().join(".canon").join("runs").join(&run_id).join("artifact-contract.toml");
    let contract_toml = fs::read_to_string(contract_path).expect("artifact contract");
    assert_eq!(
        contract_toml.trim(),
        include_str!("snapshots/requirements_artifact_contract.toml").trim()
    );
}

#[test]
fn run_requirements_markdown_surfaces_primary_result_without_raw_json() {
    let workspace = TempDir::new().expect("temp dir");
    let idea_path = workspace.path().join("idea.md");
    fs::write(
        &idea_path,
        "# Requirements Brief\n\n## Problem\nBuild a bounded USB flashing CLI for the Bird device.\n\n## Outcome\nOperators can flash firmware safely over USB with explicit logs.\n\n## Constraints\n- USB only\n- No Bluetooth in v1\n\n## Out of Scope\n- GUI\n\n## Open Questions\n- How is bootloader mode entered?\n",
    )
    .expect("idea file");

    cli_command()
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
            "--output",
            "markdown",
        ])
        .assert()
        .success()
        .stdout(contains("# run"))
        .stdout(contains("## Result"))
        .stdout(contains("Primary Artifact: .canon/artifacts/"))
        .stdout(contains("Primary Artifact Action: Open primary artifact (.canon/artifacts/"))
        .stdout(contains("Build a bounded USB flashing CLI"))
        .stdout(predicates::str::contains("## Recommended Next Step").not())
        .stdout(predicates::str::is_match("\\\"run_id\\\"").expect("regex").not());
}
