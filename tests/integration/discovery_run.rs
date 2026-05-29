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

fn complete_discovery_brief(problem_domain: &str, downstream_handoff: &str) -> String {
    format!(
        "# Discovery Brief\n\n## Problem Domain\n\n{problem_domain}\n\n## Repo Surface\n\n- crates/canon-engine/src/orchestrator/service/\n- tests/integration/discovery_run.rs\n\n## Immediate Tensions\n\n- Discovery authoring should stay explicit and reviewable.\n\n## Downstream Handoff\n\n{downstream_handoff}\n\n## Unknowns\n\n- Which downstream mode should consume repo-grounded discovery first?\n\n## Assumptions\n\n- The existing Canon persistence model remains stable.\n\n## Validation Targets\n\n- Confirm authored headings survive into emitted artifacts.\n\n## Confidence Levels\n\n- Medium until end-to-end runs confirm the new contract.\n\n## In-Scope Context\n\n- Governed analysis modes only.\n\n## Out-of-Scope Context\n\n- No architecture or review-mode changes in this packet.\n\n## Translation Trigger\n\n{downstream_handoff}\n\n## Options\n\n1. Tighten discovery authoring contracts first.\n2. Defer to a later roadmap slice.\n\n## Constraints\n\n- Preserve the existing runtime shape and evidence model.\n\n## Recommended Direction\n\nTighten discovery authoring contracts first.\n\n## Next-Phase Shape\n\nTranslate this packet into requirements mode with explicit scope cuts and handoff artifacts.\n\n## Pressure Points\n\n- Repo-local skills and runtime outputs can drift without a shared authored contract.\n\n## Blocking Decisions\n\n- Decide whether the first slice stays bounded to discovery or spans multiple modes.\n\n## Open Questions\n\n- Which downstream mode should consume repo-grounded discovery first?\n\n## Recommended Owner\n\n- researcher\n"
    )
}

#[test]
fn run_discovery_starts_draft_refinement_and_persists_working_brief() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("discovery.md");
    fs::write(
        &brief_path,
        complete_discovery_brief(
            "Reconcile Canon mode coverage with real governed runtime depth.",
            "Translate this packet into requirements mode with explicit scope cuts and handoff artifacts.",
        ),
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
    let input_snapshot = run_root.join("inputs").join("input-00-discovery.md");
    let working_brief_path =
        json["refinement_state"]["working_brief_path"].as_str().expect("working brief path");
    let working_brief =
        fs::read_to_string(workspace.path().join(working_brief_path)).expect("working brief");

    assert_eq!(json["state"], "Draft");
    assert_eq!(json["invocations_total"], 0);
    assert_eq!(json["invocations_pending_approval"], 0);
    assert_eq!(json["refinement_state"]["current_mode"], "discovery");

    assert!(run_root.join("run.toml").exists(), "run manifest should exist");
    assert!(run_root.join("context.toml").exists(), "context file should exist");
    assert!(run_root.join("inputs").is_dir(), "input snapshot directory should exist");
    assert!(input_snapshot.exists(), "authored input snapshot should exist");
    assert!(run_root.join("artifact-contract.toml").exists(), "artifact contract should exist");
    assert!(run_root.join("state.toml").exists(), "state file should exist");
    assert!(working_brief.contains("# Discovery Brief"));
    assert!(working_brief.contains("## Repo Surface"));
    assert!(working_brief.contains("tests/integration/discovery_run.rs"));
    assert!(working_brief.contains("## Downstream Handoff"));
    assert!(working_brief.contains("## Clarification Provenance"));
    assert!(working_brief.contains("## Readiness Delta"));

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
    assert_eq!(status_json["state"], "Draft");
    assert_eq!(status_json["refinement_state"]["current_mode"], "discovery");
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
        complete_discovery_brief(
            "Make discovery packets translate cleanly into governed planning.",
            "Translate this packet into requirements mode with explicit scope cuts.",
        ),
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
    assert_eq!(json["state"], "Draft");
    let working_brief_path =
        json["refinement_state"]["working_brief_path"].as_str().expect("working brief path");
    let working_brief =
        fs::read_to_string(workspace.path().join(working_brief_path)).expect("working brief");

    assert!(working_brief.contains("## Downstream Handoff"));
    assert!(working_brief.contains("requirements mode"));
    assert!(working_brief.contains("tests/integration/discovery_run.rs"));
}
