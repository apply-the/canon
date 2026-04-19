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
fn run_requirements_persists_a_run_contract_and_artifact_bundle() {
    let workspace = TempDir::new().expect("temp dir");
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
            "--risk-source",
            "inferred-confirmed",
            "--risk-rationale",
            "Production boundary detected in the authored brief.",
            "--risk-signal",
            "Detected bounded-impact signal `boundary` in the intake.",
            "--zone-source",
            "inferred-overridden",
            "--zone-rationale",
            "Operator overrode the provisional zone before run start.",
            "--zone-signal",
            "Detected yellow-zone signal `production` in the intake.",
            "--owner",
            "product-lead",
            "--input",
            idea_path.file_name().expect("file name").to_str().expect("utf8"),
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
    let run_id = json["run_id"].as_str().expect("run id");
    assert_eq!(json["mode_result"]["primary_artifact_title"].as_str(), Some("Problem Statement"));
    assert_eq!(
        json["mode_result"]["primary_artifact_action"]["id"].as_str(),
        Some("open-primary-artifact")
    );
    assert!(
        json["mode_result"]["primary_artifact_path"]
            .as_str()
            .is_some_and(|value| value.ends_with("/requirements/problem-statement.md"))
    );
    assert!(json["recommended_next_action"].is_null());

    let run_root = workspace.path().join(".canon").join("runs").join(run_id);
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("requirements");
    let input_snapshot = run_root.join("inputs").join("input-00-idea.md");

    assert!(run_root.join("run.toml").exists(), "run manifest should exist");
    assert!(run_root.join("context.toml").exists(), "context file should exist");
    assert!(run_root.join("inputs").is_dir(), "input snapshot directory should exist");
    assert!(input_snapshot.exists(), "authored input snapshot should exist");
    assert!(run_root.join("artifact-contract.toml").exists(), "artifact contract should exist");
    assert!(run_root.join("state.toml").exists(), "state file should exist");
    assert!(run_root.join("links.toml").exists(), "links file should exist");
    assert!(run_root.join("gates").is_dir(), "gates directory should exist");
    assert!(
        run_root.join("gates").join("exploration.toml").exists(),
        "exploration gate should be persisted"
    );
    assert!(run_root.join("gates").join("risk.toml").exists(), "risk gate should be persisted");
    assert!(
        run_root.join("gates").join("release-readiness.toml").exists(),
        "release readiness gate should be persisted"
    );
    assert!(run_root.join("verification").is_dir(), "verification directory should exist");
    assert!(
        run_root.join("verification").join("verification-00.toml").exists(),
        "self-critique verification record should exist"
    );
    assert!(
        run_root.join("verification").join("verification-01.toml").exists(),
        "adversarial verification record should exist"
    );

    for artifact in [
        "problem-statement.md",
        "constraints.md",
        "options.md",
        "tradeoffs.md",
        "scope-cuts.md",
        "decision-checklist.md",
    ] {
        assert!(
            artifact_root.join(artifact).exists(),
            "{artifact} should exist in the requirements bundle"
        );
    }

    let context_toml = fs::read_to_string(run_root.join("context.toml")).expect("context file");
    let context: toml::Value = toml::from_str(&context_toml).expect("context toml");
    let run_toml = fs::read_to_string(run_root.join("run.toml")).expect("run file");
    let run_manifest: toml::Value = toml::from_str(&run_toml).expect("run toml");
    let fingerprint = context["input_fingerprints"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("input fingerprint");
    let expected_snapshot_ref = format!("runs/{run_id}/inputs/input-00-idea.md");
    assert!(
        fingerprint["content_digest_sha256"].as_str().is_some_and(|value| !value.is_empty()),
        "input fingerprint should include a content digest"
    );
    assert_eq!(
        fingerprint["snapshot_ref"].as_str(),
        Some(expected_snapshot_ref.as_str()),
        "input fingerprint should reference the persisted snapshot"
    );
    assert_eq!(
        run_manifest["classification"]["risk"]["source"].as_str(),
        Some("inferred-confirmed")
    );
    assert_eq!(
        run_manifest["classification"]["zone"]["source"].as_str(),
        Some("inferred-overridden")
    );
    assert_eq!(
        run_manifest["classification"]["risk"]["signals"]
            .as_array()
            .and_then(|entries| entries.first())
            .and_then(toml::Value::as_str),
        Some("Detected bounded-impact signal `boundary` in the intake.")
    );

    let status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_text = String::from_utf8(status_output).expect("utf8 stdout");
    let status_json: serde_json::Value = serde_json::from_str(&status_text).expect("json output");
    assert_eq!(status_json["state"], "Completed");
    assert_eq!(
        status_json["mode_result"]["primary_artifact_title"].as_str(),
        Some("Problem Statement")
    );
    assert!(status_json["recommended_next_action"].is_null());
}

#[test]
fn run_requirements_expands_directory_inputs_into_snapshotted_files() {
    let workspace = TempDir::new().expect("temp dir");
    let input_dir = workspace.path().join("canon-input").join("requirements");
    fs::create_dir_all(&input_dir).expect("input dir");
    fs::write(input_dir.join("00-problem.md"), "# Problem\n\nClarify the bounded problem space.\n")
        .expect("problem file");
    fs::write(
        input_dir.join("01-constraints.md"),
        "# Constraints\n\nKeep governance durable and local-first.\n",
    )
    .expect("constraints file");

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
            "canon-input/requirements",
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
    let run_id = json["run_id"].as_str().expect("run id");
    let inputs_dir = workspace.path().join(".canon").join("runs").join(run_id).join("inputs");
    let problem_statement = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(run_id)
            .join("requirements")
            .join("problem-statement.md"),
    )
    .expect("problem statement");

    let mut snapshots = fs::read_dir(&inputs_dir)
        .expect("inputs dir")
        .map(|entry| entry.expect("snapshot entry").file_name().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    snapshots.sort();
    assert_eq!(snapshots.len(), 2, "directory input should snapshot each authored file");

    let context_toml = fs::read_to_string(
        workspace.path().join(".canon").join("runs").join(run_id).join("context.toml"),
    )
    .expect("context file");
    let context: toml::Value = toml::from_str(&context_toml).expect("context toml");
    let fingerprints = context["input_fingerprints"].as_array().expect("input fingerprints");
    assert_eq!(fingerprints.len(), 2, "directory input should fingerprint each authored file");
    assert!(
        fingerprints.iter().all(|entry| {
            entry["snapshot_ref"]
                .as_str()
                .is_some_and(|value| value.starts_with(&format!("runs/{run_id}/inputs/")))
        }),
        "each directory-backed fingerprint should reference a persisted snapshot"
    );
    assert!(
        !problem_statement.contains("## Input:"),
        "primary requirements artifact should not replay raw input labels"
    );
}

#[test]
fn run_requirements_persists_inline_input_only_under_run_snapshots() {
    let workspace = TempDir::new().expect("temp dir");

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
            "--input-text",
            "# Requirements Brief\n\n## Problem\nBound inline authored input handling.\n\n## Constraints\n- Keep evidence local\n- Preserve explicit approvals",
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
    let run_id = json["run_id"].as_str().expect("run id");
    let run_root = workspace.path().join(".canon").join("runs").join(run_id);
    let snapshot_path = run_root.join("inputs").join("input-00-inline-input-01.md");

    assert!(snapshot_path.exists(), "inline authored input should be snapshotted");
    assert!(
        !workspace.path().join("canon-input").exists(),
        "inline input should not materialize canon-input files in the repo"
    );

    let context_toml = fs::read_to_string(run_root.join("context.toml")).expect("context file");
    let context: toml::Value = toml::from_str(&context_toml).expect("context toml");
    let inputs = context["inputs"].as_array().expect("context inputs");
    let fingerprint = context["input_fingerprints"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("input fingerprint");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].as_str(), Some("inline-input-01.md"));
    assert_eq!(fingerprint["path"].as_str(), Some("inline-input-01.md"));
    assert_eq!(fingerprint["source_kind"].as_str(), Some("inline"));
    assert_eq!(
        fingerprint["snapshot_ref"].as_str(),
        Some(format!("runs/{run_id}/inputs/input-00-inline-input-01.md").as_str())
    );

    let invocation_root = run_root.join("invocations");
    for entry in fs::read_dir(&invocation_root).expect("invocation dir") {
        let entry = entry.expect("invocation entry");
        let request_toml =
            fs::read_to_string(entry.path().join("request.toml")).expect("invocation request file");
        let request: toml::Value = toml::from_str(&request_toml).expect("request toml");
        let requested_scope = request["requested_scope"].as_array().expect("requested scope array");

        assert_eq!(requested_scope.len(), 1);
        assert_eq!(requested_scope[0].as_str(), Some("inline-input-01.md"));
    }
}
