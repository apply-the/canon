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
        "# System Shaping Brief\n\nIntent: define a clean analysis-mode surface without changing Canon's governance primitives.\nConstraint: preserve the existing policy, gate, and evidence contracts.\n\n## Goal\nAdd domain modeling to the system-shaping packet without changing approvals or publish behavior.\n\n## Users or Stakeholders\n- Canon maintainers who review packet integrity.\n- Architects who consume shaping packets downstream.\n\n## Domain Responsibilities\n- Bound the system shape.\n- Name candidate bounded contexts.\n- Surface invariants that later modes must preserve.\n\n## Constraints\n- Preserve the existing policy, gate, and evidence contracts.\n- Keep non-target modes unchanged.\n\n## Risks\n- Weak briefs may produce invented boundaries.\n- Shared helpers may blur ownership between contexts.\n\n## Open Questions\n- Which context owns authored-body rendering?\n- Which context owns downstream architecture tradeoffs?\n\n## Candidate Bounded Contexts\n- Runtime Governance: owns run state, approvals, and evidence linkage.\n- Artifact Authoring: owns packet structure and authored section fidelity.\n\n## Core And Supporting Domain Hypotheses\n- Runtime Governance is core because it preserves Canon's operating model.\n- Artifact Authoring is supporting because it makes analysis reviewable.\n\n## Ubiquitous Language\n- Run: one governed execution with durable evidence.\n- Packet: the emitted artifact set for a mode.\n\n## Domain Invariants\n- Approval semantics remain unchanged.\n- Publish destinations remain unchanged.\n\n## Boundary Risks And Open Questions\n- Shared helpers may leak responsibilities across contexts.\n",
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

    let domain_model = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "domain-model.md")
        .expect("domain model requirement");
    assert_eq!(
        domain_model.required_sections,
        vec![
            "Summary",
            "Candidate Bounded Contexts",
            "Core And Supporting Domain Hypotheses",
            "Ubiquitous Language",
            "Domain Invariants",
            "Boundary Risks And Open Questions",
        ]
    );
    assert_eq!(domain_model.gates, vec![GateKind::Exploration, GateKind::Architecture]);

    let system_shape = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "system-shape.md")
        .expect("system shape requirement");
    assert_eq!(
        system_shape.required_sections,
        vec!["Summary", "System Shape", "Boundary Decisions", "Domain Responsibilities"]
    );
    assert_eq!(system_shape.gates, vec![GateKind::Exploration, GateKind::Architecture]);
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
