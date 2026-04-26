use std::fs;

use assert_cmd::Command;
use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::artifact::{ArtifactContract, ArtifactRequirement};
use canon_engine::domain::gate::{GateKind, GateStatus};
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::orchestrator::gatekeeper::{
    SystemShapingGateContext, evaluate_system_shaping_gates,
};
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

fn run_system_shaping_flow(workspace: &TempDir) -> String {
    fs::write(
        workspace.path().join("system-shaping.md"),
        "# System Shaping Brief\n\nIntent: define a clean analysis-mode surface without changing Canon's governance primitives.\nConstraint: preserve the existing policy, gate, and evidence contracts.\n\n## System Shape\nKeep the review surface grounded in authored packet sections rather than synthesized prose.\n\n## Boundary Decisions\n- Keep authored packet sections explicit per emitted artifact.\n- Keep gates, approvals, and publish destinations unchanged.\n\n## Domain Responsibilities\n- Bound the system shape.\n- Name candidate bounded contexts.\n- Surface invariants that later modes must preserve.\n\n## Candidate Bounded Contexts\n- Runtime Governance: owns run state, approvals, and evidence linkage.\n- Artifact Authoring: owns packet structure and authored section fidelity.\n\n## Core And Supporting Domain Hypotheses\n- Runtime Governance is core because it preserves Canon's operating model.\n- Artifact Authoring is supporting because it makes analysis reviewable.\n\n## Ubiquitous Language\n- Run: one governed execution with durable evidence.\n- Packet: the emitted artifact set for a mode.\n\n## Domain Invariants\n- Approval semantics remain unchanged.\n- Publish destinations remain unchanged.\n\n## Boundary Risks And Open Questions\n- Shared helpers may leak responsibilities across contexts.\n\n## Structural Options\n- Option 1 keeps authored-body preservation local to the current renderer helpers.\n- Option 2 introduces a new mapping layer before rendering.\n\n## Selected Boundaries\n- Runtime Governance remains separate from Artifact Authoring so packet fidelity does not blur approval semantics.\n\n## Rationale\n- Explicit authored sections make the packet reviewable without changing approvals or publish behavior.\n\n## Capabilities\n- Bounded system-shape definition.\n- Context and invariant capture.\n- Reviewable sequencing and risk surfacing.\n\n## Dependencies\n- Existing policy gates.\n- Existing evidence persistence.\n- Existing renderer helpers that already support authored-body preservation.\n\n## Gaps\n- Near-match heading handling still needs explicit tests.\n- Some user-facing docs still lag the runtime contract.\n\n## Delivery Phases\n1. Extend authored-body preservation to the remaining system-shaping artifacts.\n2. Synchronize skills, templates, and worked examples with the runtime contract.\n3. Close remaining validation and non-regression gaps.\n\n## Sequencing Rationale\n- Runtime fidelity must land before documentation and release guidance.\n\n## Risk per Phase\n- Phase 1: renderer changes could silently regress packet fidelity.\n- Phase 2: docs could drift from the runtime contract.\n- Phase 3: release notes could overstate rollout completeness.\n\n## Hotspots\n- Shared helpers that mix authored text with generated summaries.\n- Mode-specific artifact families that still rely on legacy headings.\n\n## Mitigation Status\n- Shared authored-section rendering is already available and can be reused.\n- Existing contract coverage can contain section-level regressions once expanded.\n\n## Unresolved Risks\n- Legacy worked examples could keep teaching inline labels unless updated.\n- Non-target modes still need explicit non-regression validation.\n",
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
            "system-shaping.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    json["run_id"].as_str().expect("run id").to_string()
}

fn render_artifact(requirement: &ArtifactRequirement) -> String {
    requirement
        .required_sections
        .iter()
        .map(|section| format!("## {section}\n\nRecorded content for {section}."))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn valid_artifacts(contract: &ArtifactContract) -> Vec<(String, String)> {
    contract
        .artifact_requirements
        .iter()
        .map(|requirement| (requirement.file_name.clone(), render_artifact(requirement)))
        .collect()
}

#[test]
fn system_shaping_contract_matches_spec_artifact_names_sections_and_gates() {
    let contract = contract_for_mode(Mode::SystemShaping);

    assert_eq!(contract.artifact_requirements.len(), 6);

    let names = contract
        .artifact_requirements
        .iter()
        .map(|requirement| requirement.file_name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        vec![
            "system-shape.md",
            "domain-model.md",
            "architecture-outline.md",
            "capability-map.md",
            "delivery-options.md",
            "risk-hotspots.md",
        ]
    );

    let sections_and_gates = contract
        .artifact_requirements
        .iter()
        .map(|requirement| {
            (
                requirement.file_name.as_str(),
                requirement.required_sections.iter().map(String::as_str).collect::<Vec<_>>(),
                requirement.gates.clone(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        sections_and_gates,
        vec![
            (
                "system-shape.md",
                vec!["Summary", "System Shape", "Boundary Decisions", "Domain Responsibilities"],
                vec![GateKind::Exploration, GateKind::Architecture],
            ),
            (
                "domain-model.md",
                vec![
                    "Summary",
                    "Candidate Bounded Contexts",
                    "Core And Supporting Domain Hypotheses",
                    "Ubiquitous Language",
                    "Domain Invariants",
                    "Boundary Risks And Open Questions",
                ],
                vec![GateKind::Exploration, GateKind::Architecture],
            ),
            (
                "architecture-outline.md",
                vec!["Summary", "Structural Options", "Selected Boundaries", "Rationale"],
                vec![GateKind::Architecture, GateKind::Risk],
            ),
            (
                "capability-map.md",
                vec!["Summary", "Capabilities", "Dependencies", "Gaps"],
                vec![GateKind::Exploration, GateKind::Architecture],
            ),
            (
                "delivery-options.md",
                vec!["Summary", "Delivery Phases", "Sequencing Rationale", "Risk per Phase"],
                vec![GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            (
                "risk-hotspots.md",
                vec!["Summary", "Hotspots", "Mitigation Status", "Unresolved Risks"],
                vec![GateKind::Risk, GateKind::ReleaseReadiness],
            ),
        ]
    );
}

#[test]
fn system_shaping_architecture_gate_blocks_when_required_artifacts_are_missing() {
    let contract = contract_for_mode(Mode::SystemShaping);
    let artifacts = valid_artifacts(&contract)
        .into_iter()
        .filter(|(file_name, _)| file_name != "domain-model.md")
        .collect::<Vec<_>>();

    let gates = evaluate_system_shaping_gates(
        &contract,
        &artifacts,
        SystemShapingGateContext {
            owner: "architect",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            approvals: &[],
            evidence_complete: true,
        },
    );
    let architecture =
        gates.iter().find(|gate| gate.gate == GateKind::Architecture).expect("architecture gate");

    assert_eq!(architecture.status, GateStatus::Blocked);
    assert!(
        architecture.blockers.iter().any(|blocker| blocker.contains("domain-model.md")),
        "architecture gate should cite the missing domain-model artifact"
    );
}

#[test]
fn system_shaping_contract_exposes_artifacts_invocations_and_evidence() {
    let workspace = TempDir::new().expect("temp dir");
    let run_id = run_system_shaping_flow(&workspace);

    cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "artifacts", "--run", &run_id])
        .assert()
        .success()
        .stdout(contains("system-shape.md"))
        .stdout(contains("domain-model.md"))
        .stdout(contains("risk-hotspots.md"));

    let invocations_output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "invocations", "--run", &run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let invocations_json: serde_json::Value =
        serde_json::from_slice(&invocations_output).expect("json output");
    let entries = invocations_json["entries"].as_array().expect("invocation entries");
    assert_eq!(
        entries.len(),
        3,
        "system-shaping should persist read, generate, and critique requests"
    );
    assert!(entries.iter().any(|entry| entry["capability"] == "ReadRepository"));
    assert!(entries.iter().any(|entry| entry["capability"] == "GenerateContent"));
    assert!(entries.iter().any(|entry| entry["capability"] == "CritiqueContent"));

    let evidence_output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "evidence", "--run", &run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let evidence_json: serde_json::Value =
        serde_json::from_slice(&evidence_output).expect("json output");
    let entry = evidence_json["entries"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("evidence entry");
    assert!(
        entry["artifact_provenance_links"].as_array().is_some_and(|paths| !paths.is_empty()),
        "system-shaping evidence should link to readable artifacts"
    );
    assert!(
        entry["validation_paths"].as_array().is_some_and(|paths| !paths.is_empty()),
        "system-shaping evidence should expose validation paths"
    );

    let contract_path = canon_engine::persistence::layout::ProjectLayout::new(workspace.path())
        .run_dir(&run_id)
        .join("artifact-contract.toml");
    let contract_toml = fs::read_to_string(contract_path).expect("artifact contract");
    assert!(contract_toml.contains("system-shape.md"));
    assert!(contract_toml.contains("domain-model.md"));
    assert!(contract_toml.contains("architecture-outline.md"));
    assert!(contract_toml.contains("risk-hotspots.md"));
}
