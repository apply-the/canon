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
    r#"# System Shaping Brief

Intent: shape a new governed Canon workflow surface for incomplete analysis modes.
Constraint: keep the runtime adapters, policy gates, and evidence model intact.

## System Shape
Keep the review surface grounded in authored packet sections rather than synthesized prose.

## Boundary Decisions
- Keep authored packet sections explicit per emitted artifact.
- Keep gates, approvals, and publish destinations unchanged.

## Domain Responsibilities
- Identify candidate bounded contexts.
- Surface ubiquitous language and weak terminology.
- Preserve domain invariants for downstream modes.

## Candidate Bounded Contexts
- Runtime Governance: owns run lifecycle, approvals, and evidence lineage.
- Artifact Authoring: owns packet structure and authored-body rendering.

## Core And Supporting Domain Hypotheses
- Runtime Governance is core because it protects Canon's operating model.
- Artifact Authoring is supporting because it exists to make reviews durable.

## Ubiquitous Language
- Run: one governed Canon execution with durable evidence.
- Packet: the artifact bundle produced by a mode.

## Domain Invariants
- Approval semantics remain unchanged.
- Publish destinations remain unchanged.

## Boundary Risks And Open Questions
- The split between authoring and governance may still leak through shared helpers.

## Structural Options
- Option 1 keeps authored-body preservation local to the current renderer helpers.
- Option 2 introduces a new system-shaping-specific mapping layer before rendering.

## Selected Boundaries
- Runtime Governance remains separate from Artifact Authoring so packet fidelity does not blur approval semantics.

## Rationale
- Explicit authored sections make the packet reviewable without widening Canon's execution model.

## Why Not The Others
- A second renderer path would duplicate authored-body extraction rules and blur the contract boundary before the slice is stable.

## Capabilities
- Bounded system-shape definition.
- Context and invariant capture.
- Reviewable sequencing and risk surfacing.

## Dependencies
- Existing policy gates.
- Existing evidence persistence.
- Existing renderer helpers that already support authored-body preservation.

## Gaps
- Near-match heading handling still needs explicit tests.
- Some user-facing docs still lag the runtime contract.

## Delivery Phases
1. Extend authored-body preservation to the remaining system-shaping artifacts.
2. Synchronize skills, templates, and worked examples with the runtime contract.
3. Close remaining validation and non-regression gaps.

## Sequencing Rationale
- Runtime fidelity must land before documentation and release guidance so later surfaces describe real behavior.

## Risk per Phase
- Phase 1: renderer changes could silently regress packet fidelity.
- Phase 2: docs could drift from the runtime contract.
- Phase 3: release notes could overstate rollout completeness.

## Hotspots
- Shared helpers that mix authored text with generated summaries.
- Mode-specific artifact families that still rely on legacy headings.

## Mitigation Status
- Shared authored-section rendering is already available and can be reused.
- Existing contract coverage can contain section-level regressions once expanded.

## Unresolved Risks
- Legacy worked examples could keep teaching inline labels unless updated.
- Non-target modes still need explicit non-regression validation.
"#
}

fn incomplete_system_shaping_brief() -> &'static str {
    r#"# System Shaping Brief

Intent: shape a new governed Canon workflow surface for incomplete analysis modes.
Constraint: keep the runtime adapters, policy gates, and evidence model intact.

## System Shape
Keep the review surface grounded in authored packet sections rather than synthesized prose.

## Boundary Decisions
- Keep authored packet sections explicit per emitted artifact.

## Domain Responsibilities
- Identify candidate bounded contexts.

## Candidate Bounded Contexts
- Runtime Governance: owns run lifecycle, approvals, and evidence lineage.

## Core And Supporting Domain Hypotheses
- Runtime Governance is core because it protects Canon's operating model.

## Ubiquitous Language
- Run: one governed Canon execution with durable evidence.

## Domain Invariants
- Approval semantics remain unchanged.

## Boundary Risks And Open Questions
- The split between authoring and governance may still leak through shared helpers.

## Structural Options
- Option 1 keeps authored-body preservation local to the current renderer helpers.

## Rationale
- Explicit authored sections make the packet reviewable without widening Canon's execution model.

## Capabilities
- Bounded system-shape definition.

## Dependencies
- Existing policy gates.

## Gaps
- Near-match heading handling still needs explicit tests.

## Delivery Phases
1. Extend authored-body preservation to the remaining system-shaping artifacts.

## Sequencing Rationale
- Runtime fidelity must land before documentation and release guidance.

## Risk per Phase
- Phase 1: renderer changes could silently regress packet fidelity.

## Hotspots
- Shared helpers that mix authored text with generated summaries.

## Mitigation Status
- Shared authored-section rendering is already available and can be reused.

## Unresolved Risks
- Legacy worked examples could keep teaching inline labels unless updated.
"#
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
    assert!(
        domain_model
            .contains("Runtime Governance: owns run lifecycle, approvals, and evidence lineage.")
    );
    assert!(domain_model.contains("## Domain Invariants"));
    assert!(
        !domain_model.contains("# System Shaping Brief"),
        "domain-model.md should render canonical sections instead of dumping the full authored brief"
    );

    let architecture_outline = fs::read_to_string(artifact_root.join("architecture-outline.md"))
        .expect("architecture outline");
    assert!(architecture_outline.contains("## Structural Options"));
    assert!(architecture_outline.contains(
        "Option 1 keeps authored-body preservation local to the current renderer helpers."
    ));
    assert!(architecture_outline.contains("## Selected Boundaries"));
    assert!(
        architecture_outline.contains(
            "Runtime Governance remains separate from Artifact Authoring so packet fidelity does not blur approval semantics."
        )
    );
    assert!(architecture_outline.contains("## Rationale"));
    assert!(architecture_outline.contains("## Why Not The Others"));
    assert!(architecture_outline.contains(
        "A second renderer path would duplicate authored-body extraction rules and blur the contract boundary before the slice is stable."
    ));

    let risk_hotspots =
        fs::read_to_string(artifact_root.join("risk-hotspots.md")).expect("risk hotspots");
    assert!(risk_hotspots.contains("## Mitigation Status"));
    assert!(
        risk_hotspots
            .contains("Shared authored-section rendering is already available and can be reused.")
    );
}

#[test]
fn run_system_shaping_emits_missing_body_marker_for_absent_canonical_heading() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("system-shaping.md");
    fs::write(&brief_path, incomplete_system_shaping_brief()).expect("brief file");

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
    let run_id = json["run_id"].as_str().expect("run id");
    let architecture_outline = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(run_id)
            .join("system-shaping")
            .join("architecture-outline.md"),
    )
    .expect("architecture outline");

    let blocked_gates = json["blocked_gates"].as_array().expect("blocked gates");

    assert_eq!(json["state"], "Blocked");
    assert_eq!(json["blocking_classification"], "artifact-blocked");
    assert!(architecture_outline.contains("## Missing Authored Body"));
    assert!(architecture_outline.contains("Selected Boundaries"));
    assert!(blocked_gates.iter().any(|gate| {
        gate["blockers"].as_array().is_some_and(|blockers| {
            blockers.iter().any(|blocker| {
                blocker.as_str().is_some_and(|text| text.contains("Selected Boundaries"))
            })
        })
    }));
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
