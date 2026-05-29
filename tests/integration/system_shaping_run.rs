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
fn run_system_shaping_starts_draft_and_blocks_follow_up_without_generated_artifacts() {
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

    assert_eq!(json["state"], "Draft");
    assert_eq!(json["invocations_total"], 0);
    assert_eq!(json["invocations_pending_approval"], 0);
    assert!(json["artifact_paths"].as_array().is_some_and(|paths| paths.is_empty()));
    assert!(json["mode_result"].is_null());
    assert_eq!(
        json["refinement_state"]["explicit_continuation_required"],
        serde_json::Value::Bool(true)
    );

    let resume_output = cli_command()
        .current_dir(workspace.path())
        .args(["resume", "--run", run_id])
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();
    let resume_json: serde_json::Value =
        serde_json::from_slice(&resume_output).expect("resume json");

    assert_eq!(resume_json["state"], "Blocked");
    assert_eq!(resume_json["blocking_classification"], "artifact-blocked");
    assert_eq!(resume_json["invocations_total"], 0);
    assert!(resume_json["artifact_paths"].as_array().is_some_and(|paths| paths.is_empty()));
    assert!(resume_json["mode_result"].is_null());
    assert_eq!(
        resume_json["refinement_state"]["explicit_continuation_required"],
        serde_json::Value::Bool(false)
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
    assert!(
        evidence_json["entries"].as_array().is_some_and(|entries| entries.is_empty()),
        "system-shaping should not persist evidence before generation begins"
    );

    let invocations_output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "invocations", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let invocations_json: serde_json::Value =
        serde_json::from_slice(&invocations_output).expect("invocations json");
    assert!(
        invocations_json["entries"].as_array().is_some_and(|entries| entries.is_empty()),
        "system-shaping should not persist invocation manifests before generation begins"
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
    assert_eq!(status_json["state"], "Blocked");
    assert_eq!(status_json["validation_independence_satisfied"], true);
    assert!(status_json["artifact_paths"].as_array().is_some_and(|paths| paths.is_empty()));
    assert!(status_json["mode_result"].is_null());
}

#[test]
fn run_system_shaping_preserves_incomplete_sections_in_the_working_brief() {
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
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let working_brief_path =
        json["refinement_state"]["working_brief_path"].as_str().expect("working brief path");
    let working_brief =
        fs::read_to_string(workspace.path().join(working_brief_path)).expect("working brief");

    assert_eq!(json["state"], "Draft");
    assert!(json["artifact_paths"].as_array().is_some_and(|paths| paths.is_empty()));
    assert!(working_brief.contains("## Structural Options"));
    assert!(working_brief.contains(
        "Option 1 keeps authored-body preservation local to the current renderer helpers."
    ));
    assert!(!working_brief.contains("## Selected Boundaries"));
    assert!(!working_brief.contains("## Missing Authored Body"));
    assert!(working_brief.contains("## Clarification Provenance"));
    assert!(working_brief.contains("## Readiness Delta"));
}

#[test]
fn run_system_shaping_surfaces_missing_context_in_refinement_readiness() {
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
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    assert_eq!(json["state"], "Draft");
    assert!(json["blocking_classification"].is_null());

    let readiness_items =
        json["refinement_state"]["readiness_items"].as_array().expect("readiness items");
    assert!(
        readiness_items.iter().any(|item| {
            item["source_kind"] == "missing-context"
                && item["summary"]
                    .as_str()
                    .is_some_and(|text| text.contains("Planning intent is missing"))
        }),
        "underspecified system-shaping runs should surface missing intent in readiness items"
    );
    assert!(
        readiness_items.iter().any(|item| {
            item["source_kind"] == "missing-context"
                && item["summary"]
                    .as_str()
                    .is_some_and(|text| text.contains("Planning boundary is missing"))
        }),
        "underspecified system-shaping runs should surface missing boundary context"
    );
    assert!(
        readiness_items.iter().any(|item| item["source_kind"] == "clarification-gap"),
        "underspecified system-shaping runs should surface a clarification gap"
    );
}
