use std::fs;

use assert_cmd::Command;
use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::artifact::{ArtifactContract, ArtifactRequirement};
use canon_engine::domain::gate::{GateKind, GateStatus};
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::orchestrator::gatekeeper::{DiscoveryGateContext, evaluate_discovery_gates};
use predicates::str::contains;
use tempfile::TempDir;
use time::OffsetDateTime;

use canon_engine::domain::approval::{ApprovalDecision, ApprovalRecord};

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

fn run_discovery_flow(workspace: &TempDir) -> String {
    fs::write(
        workspace.path().join("discovery.md"),
        "# Discovery Brief\n\nProblem: clarify governed runtime depth for analysis modes.\nConstraints: stay within the existing Canon persistence model.\nNext Phase: translate this packet into requirements mode with concrete scope cuts.\n",
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
            "discovery.md",
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
fn discovery_contract_matches_spec_artifact_names_sections_and_gates() {
    let contract = contract_for_mode(Mode::Discovery);

    assert_eq!(contract.artifact_requirements.len(), 5);

    let names = contract
        .artifact_requirements
        .iter()
        .map(|requirement| requirement.file_name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        vec![
            "problem-map.md",
            "unknowns-and-assumptions.md",
            "context-boundary.md",
            "exploration-options.md",
            "decision-pressure-points.md",
        ]
    );

    let problem_map = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "problem-map.md")
        .expect("problem map requirement");
    assert_eq!(
        problem_map.required_sections,
        vec![
            "Summary",
            "Repo Signals",
            "Problem Domain",
            "Immediate Tensions",
            "Downstream Handoff",
        ]
    );
    assert_eq!(problem_map.gates, vec![GateKind::Exploration, GateKind::Risk]);
}

#[test]
fn discovery_gate_blocks_when_bounded_problem_artifacts_are_missing() {
    let contract = contract_for_mode(Mode::Discovery);
    let artifacts = valid_artifacts(&contract)
        .into_iter()
        .filter(|(file_name, _)| file_name != "context-boundary.md")
        .collect::<Vec<_>>();

    let gates = evaluate_discovery_gates(
        &contract,
        &artifacts,
        DiscoveryGateContext {
            owner: "researcher",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    let exploration =
        gates.iter().find(|gate| gate.gate == GateKind::Exploration).expect("exploration gate");

    assert_eq!(exploration.status, GateStatus::Blocked);
    assert!(
        exploration.blockers.iter().any(|blocker| blocker.contains("context-boundary.md")),
        "exploration gate should cite the missing context boundary artifact"
    );
}

#[test]
fn discovery_gate_requires_approval_for_systemic_red_runs() {
    let contract = contract_for_mode(Mode::Discovery);
    let artifacts = valid_artifacts(&contract);

    let gates = evaluate_discovery_gates(
        &contract,
        &artifacts,
        DiscoveryGateContext {
            owner: "researcher",
            risk: RiskClass::SystemicImpact,
            zone: UsageZone::Red,
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    let risk = gates.iter().find(|gate| gate.gate == GateKind::Risk).expect("risk gate");
    assert_eq!(risk.status, GateStatus::NeedsApproval);

    let approvals = vec![ApprovalRecord::for_gate(
        GateKind::Risk,
        "principal-engineer".to_string(),
        ApprovalDecision::Approve,
        "accepted".to_string(),
        OffsetDateTime::UNIX_EPOCH,
    )];
    let gates = evaluate_discovery_gates(
        &contract,
        &artifacts,
        DiscoveryGateContext {
            owner: "researcher",
            risk: RiskClass::SystemicImpact,
            zone: UsageZone::Red,
            approvals: &approvals,
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    let risk = gates.iter().find(|gate| gate.gate == GateKind::Risk).expect("risk gate");
    assert_eq!(risk.status, GateStatus::Overridden);
}

#[test]
fn discovery_release_readiness_requires_independent_validation() {
    let contract = contract_for_mode(Mode::Discovery);
    let artifacts = valid_artifacts(&contract);

    let gates = evaluate_discovery_gates(
        &contract,
        &artifacts,
        DiscoveryGateContext {
            owner: "researcher",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            approvals: &[],
            validation_independence_satisfied: false,
            evidence_complete: true,
        },
    );
    let release =
        gates.iter().find(|gate| gate.gate == GateKind::ReleaseReadiness).expect("release gate");

    assert_eq!(release.status, GateStatus::Blocked);
    assert!(
        release
            .blockers
            .iter()
            .any(|blocker| blocker.contains("independently recorded repository validation path")),
        "release readiness should require independent repository validation"
    );
}

#[test]
fn inspect_artifacts_lists_the_discovery_bundle_and_contract() {
    let workspace = TempDir::new().expect("temp dir");
    let run_id = run_discovery_flow(&workspace);

    let inspect_output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "artifacts", "--run", &run_id, "--output", "json"])
        .assert()
        .success()
        .stdout(contains("problem-map.md"))
        .stdout(contains("decision-pressure-points.md"))
        .get_output()
        .stdout
        .clone();

    let inspect_json: serde_json::Value =
        serde_json::from_slice(&inspect_output).expect("json output");
    let entries = inspect_json["entries"].as_array().expect("artifact entries");
    let actual_paths =
        entries.iter().map(|entry| entry.as_str().expect("artifact path")).collect::<Vec<_>>();
    let expected_paths = vec![
        format!(".canon/artifacts/{run_id}/discovery/context-boundary.md"),
        format!(".canon/artifacts/{run_id}/discovery/decision-pressure-points.md"),
        format!(".canon/artifacts/{run_id}/discovery/exploration-options.md"),
        format!(".canon/artifacts/{run_id}/discovery/problem-map.md"),
        format!(".canon/artifacts/{run_id}/discovery/unknowns-and-assumptions.md"),
    ];
    assert_eq!(actual_paths, expected_paths);

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
        4,
        "discovery should persist read, generate, critique, and validation requests"
    );
    assert!(entries.iter().any(|entry| entry["capability"] == "ReadRepository"));
    assert!(entries.iter().any(|entry| entry["capability"] == "GenerateContent"));
    assert!(entries.iter().any(|entry| entry["capability"] == "CritiqueContent"));
    assert!(entries.iter().any(|entry| entry["capability"] == "ValidateWithTool"));

    let contract_path =
        workspace.path().join(".canon").join("runs").join(&run_id).join("artifact-contract.toml");
    let contract_toml = fs::read_to_string(contract_path).expect("artifact contract");
    assert!(contract_toml.contains("problem-map.md"));
    assert!(contract_toml.contains("unknowns-and-assumptions.md"));
    assert!(contract_toml.contains("context-boundary.md"));
}
