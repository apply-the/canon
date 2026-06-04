use std::fs;

use assert_cmd::Command;
use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::artifact::{ArtifactContract, ArtifactRequirement};
use canon_engine::domain::gate::{GateKind, GateStatus};
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::orchestrator::gatekeeper::{
    BrainstormingGateContext, evaluate_brainstorming_gates,
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

fn run_brainstorming_flow(workspace: &TempDir) -> String {
    fs::write(
        workspace.path().join("brainstorming.md"),
        "# Brainstorming Brief\n\nIntent: define options and spikes.\nConstraint: do not converge.\n\n## Summary\nSummary content.\n\n## Context\nSome context.\n\n## Options\nOption A.\nOption B.\nOption C.\n\n## Tradeoffs\nTradeoffs.\n\n## Open Questions\nQuestions.\n\n## Spikes\nSpike 1.\n",
    )
    .expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "brainstorming",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "architect",
            "--input",
            "brainstorming.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id").to_string();

    let resume_output = cli_command()
        .current_dir(workspace.path())
        .args(["resume", "--run", &run_id])
        .output()
        .expect("resume brainstorming run");
    let resume_code = resume_output.status.code().expect("resume exit code");
    if !matches!(resume_code, 0 | 2) {
        let stdout = String::from_utf8_lossy(&resume_output.stdout);
        let stderr = String::from_utf8_lossy(&resume_output.stderr);
        panic!(
            "expected brainstorming resume to materialize follow-up artifacts, got exit code {resume_code}\nstdout:\n{stdout}\nstderr:\n{stderr}"
        );
    }

    run_id
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
fn brainstorming_contract_matches_spec_artifact_names_sections_and_gates() {
    let contract = contract_for_mode(Mode::Brainstorming);

    assert_eq!(contract.artifact_requirements.len(), 6);

    let names = contract
        .artifact_requirements
        .iter()
        .map(|requirement| requirement.slug())
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        vec![
            "context.md",
            "options.md",
            "tradeoffs.md",
            "open-questions.md",
            "spikes.md",
            "packet-metadata.json",
        ]
    );

    let sections_and_gates = contract
        .artifact_requirements
        .iter()
        .map(|requirement| {
            (
                requirement.slug(),
                requirement.required_sections.iter().map(String::as_str).collect::<Vec<_>>(),
                requirement.gates.clone(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        sections_and_gates,
        vec![
            ("context.md", vec!["Summary", "Context"], vec![GateKind::Exploration],),
            ("options.md", vec!["Summary", "Options"], vec![GateKind::Exploration],),
            (
                "tradeoffs.md",
                vec!["Summary", "Tradeoffs"],
                vec![GateKind::Exploration, GateKind::Risk],
            ),
            ("open-questions.md", vec!["Summary", "Open Questions"], vec![GateKind::Risk],),
            (
                "spikes.md",
                vec!["Summary", "Spikes"],
                vec![GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            ("packet-metadata.json", vec![], vec![GateKind::ReleaseReadiness],),
        ]
    );
}

#[test]
fn brainstorming_exploration_gate_blocks_when_required_artifacts_are_missing() {
    let contract = contract_for_mode(Mode::Brainstorming);
    let artifacts = valid_artifacts(&contract)
        .into_iter()
        .filter(|(file_name, _)| {
            canon_engine::domain::artifact::artifact_slug(file_name) != "options.md"
        })
        .collect::<Vec<_>>();

    let gates = evaluate_brainstorming_gates(
        &contract,
        &artifacts,
        BrainstormingGateContext {
            owner: "architect",
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
        exploration.blockers.iter().any(|blocker| blocker.contains("options.md")),
        "exploration gate should cite the missing options artifact"
    );
}

#[test]
fn brainstorming_contract_exposes_artifacts_invocations_and_evidence() {
    let workspace = TempDir::new().expect("temp dir");
    let run_id = run_brainstorming_flow(&workspace);

    cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "artifacts", "--run", &run_id])
        .assert()
        .success()
        .stdout(contains("No artifacts recorded."));

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
    assert!(entries.is_empty(), "brainstorming should not persist invocations before generation");

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
    let evidence_entries = evidence_json["entries"].as_array().expect("evidence entries");
    assert!(
        evidence_entries.is_empty(),
        "brainstorming should not persist evidence bundles before generation"
    );

    let contract_path = canon_engine::persistence::layout::ProjectLayout::new(workspace.path())
        .run_dir(&run_id)
        .join("artifact-contract.toml");
    let contract_toml = fs::read_to_string(contract_path).expect("artifact contract");
    assert!(contract_toml.contains("context.md"));
    assert!(contract_toml.contains("options.md"));
    assert!(contract_toml.contains("spikes.md"));
}
