use std::fs;

use assert_cmd::Command;
use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::artifact::{ArtifactContract, ArtifactRequirement};
use canon_engine::domain::gate::{GateKind, GateStatus};
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::orchestrator::gatekeeper::{
    ArchitectureGateContext, evaluate_architecture_gates,
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

fn parse_run_id(output: &[u8]) -> String {
    let json: serde_json::Value = serde_json::from_slice(output).expect("json output");
    json["run_id"].as_str().expect("run id").to_string()
}

fn architecture_brief() -> &'static str {
    "# Architecture Brief\n\nDecision focus: identify boundary ownership and tradeoffs for analysis-mode expansion.\nConstraint: preserve Canon runtime contracts, approvals, and evidence persistence.\n"
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

fn start_architecture_run(workspace: &TempDir, risk: &str, zone: &str) -> serde_json::Value {
    fs::write(workspace.path().join("architecture.md"), architecture_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "architecture",
            "--system-context",
            "existing",
            "--risk",
            risk,
            "--zone",
            zone,
            "--owner",
            "principal-architect",
            "--input",
            "architecture.md",
            "--output",
            "json",
        ])
        .assert()
        .code(3)
        .get_output()
        .stdout
        .clone();

    serde_json::from_slice(&output).expect("json output")
}

#[test]
fn architecture_contract_matches_spec_artifact_names_sections_and_gates() {
    let contract = contract_for_mode(Mode::Architecture);

    assert_eq!(contract.artifact_requirements.len(), 5);

    let names = contract
        .artifact_requirements
        .iter()
        .map(|requirement| requirement.file_name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        vec![
            "architecture-decisions.md",
            "invariants.md",
            "tradeoff-matrix.md",
            "boundary-map.md",
            "readiness-assessment.md",
        ]
    );

    let decisions = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "architecture-decisions.md")
        .expect("architecture decisions requirement");
    assert_eq!(
        decisions.required_sections,
        vec!["Summary", "Decisions", "Tradeoffs", "Consequences", "Unresolved Questions"]
    );
    assert_eq!(decisions.gates, vec![GateKind::Architecture, GateKind::Risk]);
}

#[test]
fn architecture_gate_blocks_when_required_decision_artifacts_are_missing() {
    let contract = contract_for_mode(Mode::Architecture);
    let artifacts = valid_artifacts(&contract)
        .into_iter()
        .filter(|(file_name, _)| file_name != "tradeoff-matrix.md")
        .collect::<Vec<_>>();

    let gates = evaluate_architecture_gates(
        &contract,
        &artifacts,
        ArchitectureGateContext {
            owner: "principal-architect",
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
        architecture.blockers.iter().any(|blocker| blocker.contains("tradeoff-matrix.md")),
        "architecture gate should cite the missing tradeoff matrix artifact"
    );
}

#[test]
fn systemic_architecture_run_requires_gate_approval_and_completes_after_approval() {
    let workspace = TempDir::new().expect("temp dir");
    let run_json = start_architecture_run(&workspace, "systemic-impact", "yellow");
    let run_id = run_json["run_id"].as_str().expect("run id");

    assert_eq!(run_json["state"], "AwaitingApproval");
    assert_eq!(run_json["blocking_classification"], "approval-gated");
    assert!(
        run_json["approval_targets"]
            .as_array()
            .is_some_and(|targets| targets.iter().any(|target| target == "gate:risk")),
        "systemic architecture runs should surface gate approval targets"
    );
    assert_eq!(run_json["invocations_pending_approval"], 0);

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            run_id,
            "--target",
            "gate:risk",
            "--by",
            "principal-engineer",
            "--decision",
            "approve",
            "--rationale",
            "Systemic architecture analysis may proceed with explicit ownership.",
        ])
        .assert()
        .success()
        .stdout(contains(run_id));

    let approval_record = workspace
        .path()
        .join(".canon")
        .join("runs")
        .join(run_id)
        .join("approvals")
        .join("approval-00.toml");
    assert!(approval_record.exists(), "approval record should be persisted");
    let approval_toml = fs::read_to_string(approval_record).expect("approval record");
    let approval_value: toml::Value = approval_toml.parse().expect("approval toml");
    assert_eq!(approval_value["gate"].as_str(), Some("Risk"));

    cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .stdout(contains("\"state\": \"Completed\""));
}

#[test]
fn red_zone_architecture_run_requires_gate_approval_even_when_bounded_impact() {
    let workspace = TempDir::new().expect("temp dir");
    let run_json = start_architecture_run(&workspace, "bounded-impact", "red");
    let run_id = parse_run_id(&serde_json::to_vec(&run_json).expect("serialize run json"));

    assert_eq!(run_json["state"], "AwaitingApproval");
    assert_eq!(run_json["blocking_classification"], "approval-gated");
    assert!(
        run_json["approval_targets"]
            .as_array()
            .is_some_and(|targets| targets.iter().any(|target| target == "gate:risk")),
        "red-zone architecture runs should still surface gate approval targets"
    );

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            &run_id,
            "--target",
            "gate:risk",
            "--by",
            "principal-engineer",
            "--decision",
            "approve",
            "--rationale",
            "Red-zone architecture analysis may proceed with explicit gate approval.",
        ])
        .assert()
        .success();

    let status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", &run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_json: serde_json::Value =
        serde_json::from_slice(&status_output).expect("status json");
    assert_eq!(status_json["state"], "Completed");
}
