use std::fs;
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

    Command::new("canon")
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
