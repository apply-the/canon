use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
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
fn inspect_clarity_surfaces_targeted_questions_for_requirements_inputs() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("requirements.md"),
        "# Requirements Brief\n\n## Problem\n\nBuild a bounded USB flashing CLI for the Bird device.\n\n## Outcome\n\nOperators can flash firmware safely over USB with explicit logs.\n",
    )
    .expect("requirements brief");

    cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "clarity",
            "--mode",
            "requirements",
            "--input",
            "requirements.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .stdout(contains("\"target\": \"clarity\""))
        .stdout(contains("\"mode\": \"requirements\""))
        .stdout(contains("\"requires_clarification\": true"))
        .stdout(contains("Which constraints are non-negotiable for this work?"))
        .stdout(contains("What is explicitly out of scope or deferred for this packet?"));
}

#[test]
fn inspect_clarity_recurses_directory_inputs_and_accepts_multiple_paths_in_one_group() {
    let workspace = TempDir::new().expect("temp dir");
    let requirements_dir = workspace.path().join("canon-input").join("requirements");
    let support_dir = requirements_dir.join("supporting");
    fs::create_dir_all(&support_dir).expect("requirements dir");
    fs::write(
        requirements_dir.join("brief.md"),
        "# Requirements Brief\n\n## Problem\n\nBuild a bounded USB flashing CLI for the Bird device.\n\n## Outcome\n\nOperators can flash firmware safely over USB with explicit logs.\n",
    )
    .expect("brief");
    fs::write(
        requirements_dir.join("constraints.md"),
        "## Constraints\n\n- USB transport only\n\n## Tradeoffs\n\n- Safety over throughput\n",
    )
    .expect("constraints");
    fs::write(
        support_dir.join("scope-and-questions.md"),
        "## Out of Scope\n\n- No Bluetooth flashing in v1\n\n## Open Questions\n\n- How is the Bird identified over USB?\n",
    )
    .expect("scope questions");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "clarity",
            "--mode",
            "requirements",
            "--input",
            "canon-input/requirements",
            "canon-input/requirements/brief.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).expect("utf8 stdout");
    let json: serde_json::Value = serde_json::from_str(&text).expect("json output");
    let source_inputs = json["entries"][0]["source_inputs"]
        .as_array()
        .expect("source inputs array")
        .iter()
        .filter_map(|value| value.as_str())
        .collect::<Vec<_>>();

    assert!(source_inputs.contains(&"canon-input/requirements/brief.md"));
    assert!(source_inputs.contains(&"canon-input/requirements/constraints.md"));
    assert!(source_inputs.contains(&"canon-input/requirements/supporting/scope-and-questions.md"));
    assert_eq!(source_inputs.len(), 3, "source inputs should be de-duplicated");
    assert_eq!(json["entries"][0]["requires_clarification"].as_bool(), Some(true));
    assert!(text.contains("How is the Bird identified over USB?"));
}

#[test]
fn inspect_clarity_surfaces_targeted_questions_for_supply_chain_inputs() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("supply-chain-analysis.md"),
        "# Supply Chain Analysis Brief\n\n## Declared Scope\n\n- Cargo workspace dependency and release posture only\n\n## Ecosystems In Scope\n\n- Cargo workspace manifests\n\n## Source Inputs\n\n- Cargo.toml\n",
    )
    .expect("supply-chain brief");

    cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "clarity",
            "--mode",
            "supply-chain-analysis",
            "--input",
            "supply-chain-analysis.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .stdout(contains("\"target\": \"clarity\""))
        .stdout(contains("\"mode\": \"supply-chain-analysis\""))
        .stdout(contains("\"requires_clarification\": true"))
        .stdout(contains("What licensing posture governs this repository surface"))
        .stdout(contains("Are non-OSS scanner proposals allowed"));
}
