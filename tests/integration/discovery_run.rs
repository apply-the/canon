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
fn run_discovery_persists_a_completed_run_and_artifact_bundle() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("discovery.md");
    fs::write(
        &brief_path,
        "# Discovery Brief\n\nProblem: reconcile Canon mode coverage with real governed runtime depth.\nConstraints: preserve the existing runtime shape and evidence model.\nUnknowns: which downstream mode should consume repo-grounded discovery first?\nNext Phase: translate this packet into requirements mode with explicit scope cuts and handoff artifacts.\n",
    )
    .expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "discovery",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "researcher",
            "--input",
            brief_path.file_name().expect("file name").to_str().expect("utf8"),
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");

    let run_root =
        canon_engine::persistence::layout::ProjectLayout::new(workspace.path()).run_dir(run_id);
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("discovery");
    let input_snapshot = run_root.join("inputs").join("input-00-discovery.md");

    assert_eq!(json["state"], "Completed");
    assert_eq!(json["invocations_total"], 4);
    assert_eq!(json["invocations_pending_approval"], 0);
    assert_eq!(json["mode_result"]["primary_artifact_title"].as_str(), Some("Problem Map"));
    assert!(
        json["mode_result"]["primary_artifact_path"]
            .as_str()
            .is_some_and(|value| value.ends_with("/discovery/problem-map.md"))
    );

    assert!(run_root.join("run.toml").exists(), "run manifest should exist");
    assert!(run_root.join("context.toml").exists(), "context file should exist");
    assert!(run_root.join("inputs").is_dir(), "input snapshot directory should exist");
    assert!(input_snapshot.exists(), "authored input snapshot should exist");
    assert!(run_root.join("artifact-contract.toml").exists(), "artifact contract should exist");
    assert!(run_root.join("state.toml").exists(), "state file should exist");
    assert!(
        run_root.join("gates").join("exploration.toml").exists(),
        "exploration gate should exist"
    );
    assert!(run_root.join("gates").join("risk.toml").exists(), "risk gate should exist");
    assert!(
        run_root.join("gates").join("release-readiness.toml").exists(),
        "release readiness gate should exist"
    );
    assert!(run_root.join("evidence.toml").exists(), "evidence bundle should exist");

    for artifact in [
        "problem-map.md",
        "unknowns-and-assumptions.md",
        "context-boundary.md",
        "exploration-options.md",
        "decision-pressure-points.md",
    ] {
        assert!(
            artifact_root.join(artifact).exists(),
            "{artifact} should exist in the discovery bundle"
        );
    }

    let problem_map =
        fs::read_to_string(artifact_root.join("problem-map.md")).expect("problem map contents");
    let boundary = fs::read_to_string(artifact_root.join("context-boundary.md"))
        .expect("context boundary contents");
    assert!(problem_map.contains("## Repo Signals"));
    assert!(problem_map.contains("discovery.md"));
    assert!(problem_map.contains("## Downstream Handoff"));
    assert!(boundary.contains("## Repo Surface"));
    assert!(boundary.contains("## Translation Trigger"));

    let context_toml = fs::read_to_string(run_root.join("context.toml")).expect("context file");
    let context: toml::Value = toml::from_str(&context_toml).expect("context toml");
    let fingerprint = context["input_fingerprints"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("input fingerprint");
    let expected_snapshot_ref = format!("runs/{run_id}/inputs/input-00-discovery.md");
    assert!(
        fingerprint["content_digest_sha256"].as_str().is_some_and(|value| !value.is_empty()),
        "input fingerprint should include a content digest"
    );
    assert_eq!(
        fingerprint["snapshot_ref"].as_str(),
        Some(expected_snapshot_ref.as_str()),
        "input fingerprint should reference the persisted snapshot"
    );

    let evidence_output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "evidence", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let evidence_json: serde_json::Value =
        serde_json::from_slice(&evidence_output).expect("evidence json");
    let entry = evidence_json["entries"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("evidence entry");
    assert!(
        entry["generation_paths"].as_array().is_some_and(|paths| !paths.is_empty()),
        "discovery should persist generation evidence"
    );
    assert!(
        entry["validation_paths"].as_array().is_some_and(|paths| !paths.is_empty()),
        "discovery should persist repository validation paths"
    );

    let status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_json: serde_json::Value =
        serde_json::from_slice(&status_output).expect("status json");
    assert_eq!(status_json["state"], "Completed");
    assert_eq!(status_json["mode_result"]["primary_artifact_title"].as_str(), Some("Problem Map"));
}

#[test]
fn run_discovery_rejects_generated_artifact_inputs() {
    let workspace = TempDir::new().expect("temp dir");
    let generated_input = workspace
        .path()
        .join(".canon")
        .join("artifacts")
        .join("seed-run")
        .join("discovery")
        .join("decision-pressure-points.md");
    fs::create_dir_all(generated_input.parent().expect("artifact parent")).expect("artifact dir");
    fs::write(
        &generated_input,
        "# Generated Discovery Artifact\n\nThis should not be reusable as authored input.\n",
    )
    .expect("generated artifact file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "discovery",
            "--risk",
            "low-impact",
            "--zone",
            "green",
            "--owner",
            "researcher",
            "--input",
            ".canon/artifacts/seed-run/discovery/decision-pressure-points.md",
        ])
        .assert()
        .code(5)
        .get_output()
        .stderr
        .clone();

    let stderr = String::from_utf8(output).expect("utf8 stderr");
    assert!(
        stderr.contains("points inside .canon/ and cannot be used as authored input for discovery"),
        "validation should reject generated artifact input paths: {stderr}"
    );
}

#[test]
fn run_discovery_reads_directory_backed_inputs_from_canon_input() {
    let workspace = TempDir::new().expect("temp dir");
    let discovery_dir = workspace.path().join("canon-input").join("discovery");
    fs::create_dir_all(&discovery_dir).expect("discovery input dir");
    fs::write(
        discovery_dir.join("brief.md"),
        "# Discovery Brief\n\nProblem: make discovery packets translate cleanly into governed planning.\nConstraints: preserve repository-specific signals and evidence links.\nNext Phase: translate this packet into requirements mode with explicit scope cuts.\n",
    )
    .expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "discovery",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "researcher",
            "--input",
            "canon-input/discovery",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");
    let problem_map = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(run_id)
            .join("discovery")
            .join("problem-map.md"),
    )
    .expect("problem map");

    assert!(problem_map.contains("## Downstream Handoff"));
    assert!(problem_map.contains("requirements mode"));
    assert!(problem_map.contains("brief.md"));
}
