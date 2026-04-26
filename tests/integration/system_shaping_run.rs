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

fn complete_system_shaping_brief() -> &'static str {
    "# System Shaping Brief\n\nIntent: shape a new governed Canon workflow surface for incomplete analysis modes.\nConstraint: keep the runtime adapters, policy gates, and evidence model intact.\n\n## Goal\nAdd domain modeling to the system-shaping packet without widening Canon's execution model.\n\n## Users or Stakeholders\n- Canon maintainers who need reviewable boundary decisions.\n- Architects who need context boundaries before architecture mode.\n\n## Domain Responsibilities\n- Identify candidate bounded contexts.\n- Surface ubiquitous language and weak terminology.\n- Preserve domain invariants for downstream modes.\n\n## Constraints\n- Keep run identity, approvals, and publish destinations unchanged.\n- Keep non-target modes behaviorally unchanged.\n\n## Risks\n- Weak briefs may tempt the renderer to invent boundaries.\n- Boundary tradeoffs may be implied rather than stated.\n\n## Open Questions\n- Which bounded contexts are core versus supporting?\n- Which invariants must architecture preserve first?\n\n## Candidate Bounded Contexts\n- Runtime Governance: owns run lifecycle, approvals, and evidence lineage.\n- Artifact Authoring: owns packet structure and authored-body rendering.\n\n## Core And Supporting Domain Hypotheses\n- Runtime Governance is core because it protects Canon's operating model.\n- Artifact Authoring is supporting because it exists to make reviews durable.\n\n## Ubiquitous Language\n- Run: one governed Canon execution with durable evidence.\n- Packet: the artifact bundle produced by a mode.\n\n## Domain Invariants\n- Approval semantics remain unchanged.\n- Publish destinations remain unchanged.\n\n## Boundary Risks And Open Questions\n- The split between authoring and governance may still leak through shared helpers.\n"
}

#[test]
fn run_system_shaping_persists_completed_artifacts_and_validation_evidence() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("system-shaping.md");
    fs::write(&brief_path, complete_system_shaping_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "system-shaping",
            "--system-context",
            "new",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "architect",
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
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("system-shaping");

    assert_eq!(json["state"], "Completed");
    assert_eq!(json["invocations_total"], 3);
    assert_eq!(json["invocations_pending_approval"], 0);
    assert_eq!(json["mode_result"]["primary_artifact_title"].as_str(), Some("System Shape"));
    assert!(
        json["mode_result"]["primary_artifact_path"]
            .as_str()
            .is_some_and(|value| value.ends_with("/system-shaping/system-shape.md"))
    );

    for artifact in [
        "system-shape.md",
        "domain-model.md",
        "architecture-outline.md",
        "capability-map.md",
        "delivery-options.md",
        "risk-hotspots.md",
    ] {
        assert!(
            artifact_root.join(artifact).exists(),
            "{artifact} should exist in the system-shaping bundle"
        );
    }

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
        "system-shaping should persist generation evidence"
    );
    assert!(
        entry["validation_paths"].as_array().is_some_and(|paths| !paths.is_empty()),
        "system-shaping should persist critique validation evidence"
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
    assert_eq!(status_json["validation_independence_satisfied"], false);
    assert_eq!(status_json["mode_result"]["primary_artifact_title"].as_str(), Some("System Shape"));

    let domain_model =
        fs::read_to_string(artifact_root.join("domain-model.md")).expect("domain model");
    assert!(domain_model.starts_with("# Domain Model\n\n## Summary\n\nIntent:"));
    assert!(domain_model.contains("## Candidate Bounded Contexts"));
    assert!(domain_model.contains("## Core And Supporting Domain Hypotheses"));
    assert!(domain_model.contains("## Domain Invariants"));
    assert!(
        !domain_model.contains("# System Shaping Brief"),
        "domain-model.md should render canonical sections instead of dumping the full authored brief"
    );
}

#[test]
fn run_system_shaping_blocks_when_context_is_missing_intent_and_constraint_markers() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("system-shaping.md");
    fs::write(
        &brief_path,
        "# System Shaping Brief\n\nNeed a future-looking shape for analysis mode support.\n",
    )
    .expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "system-shaping",
            "--system-context",
            "new",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "architect",
            "--input",
            brief_path.file_name().expect("file name").to_str().expect("utf8"),
            "--output",
            "json",
        ])
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    assert_eq!(json["state"], "Blocked");
    assert_eq!(json["blocking_classification"], "artifact-blocked");

    let blocked_gates = json["blocked_gates"].as_array().expect("blocked gates");
    let architecture_gate = blocked_gates
        .iter()
        .find(|gate| gate["gate"] == "architecture")
        .expect("architecture gate blocker");
    assert!(
        architecture_gate["blockers"].as_array().is_some_and(|blockers| blockers.iter().any(
            |blocker| blocker
                .as_str()
                .is_some_and(|text| text.contains("lacks sufficient evidence"))
        )),
        "system-shaping runs with underspecified context should block on evidence quality"
    );
}
