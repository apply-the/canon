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
    "# Architecture Brief\n\nDecision focus: identify boundary ownership and tradeoffs for analysis-mode expansion.\nConstraint: preserve Canon runtime contracts, approvals, and evidence persistence.\n\n## Decision\nMake bounded contexts and context relationships first-class in architecture packets.\n\n## Options\n- Add a dedicated context map while keeping the existing C4 artifacts.\n- Force all domain boundary detail into `boundary-map.md`.\n\n## Constraints\n- Preserve approval semantics and publish destinations.\n- Keep non-target modes unchanged.\n\n## Candidate Boundaries\n- Runtime Governance\n- Artifact Authoring\n\n## Invariants\n- Run identity remains unchanged.\n- Evidence lineage remains reviewable.\n\n## Evaluation Criteria\n- Boundary clarity\n- Coupling visibility\n\n## Decision Drivers\n- Reviewers need the selected option and rationale without consulting chat history.\n- The packet must remain critique-first when authored context is weak.\n\n## Options Considered\n- Keep generic architecture summaries and accept the loss of rejected alternatives.\n- Preserve authored decision and option-analysis sections directly in the existing artifacts.\n\n## Pros\n- Reviewers can inspect the chosen and rejected options directly.\n- The packet remains reusable outside the originating conversation.\n\n## Cons\n- Authors must provide richer decision content up front.\n\n## Recommendation\nPreserve authored decision and option-analysis sections in the existing architecture decision artifacts.\n\n## Why Not The Others\n- The generic summary path hides rejected alternatives.\n- A brand new artifact family would widen scope and churn.\n\n## Risks\n- Shared helpers may hide ownership boundaries.\n- Context crossings may look cleaner than they are.\n\n## Bounded Contexts\n- Runtime Governance: owns approvals, gate state, and evidence linkage.\n- Artifact Authoring: owns packet composition and authored-body fidelity.\n\n## Context Relationships\n- Artifact Authoring depends on Runtime Governance for gate outcomes and persisted artifact identity.\n\n## Integration Seams\n- Orchestrator service boundaries separate artifact generation from gate evaluation.\n\n## Anti-Corruption Candidates\n- A narrow renderer-facing contract should shield authored packet structure from orchestration internals.\n\n## Ownership Boundaries\n- Runtime Governance is owned by the execution and policy layer.\n- Artifact Authoring is owned by the markdown rendering and authored-body extraction layer.\n\n## Shared Invariants\n- Published artifacts remain traceable to one run id.\n- Approval-gated work cannot silently skip risk review.\n\n## System Context\n- System: `canon-engine` governs AI-assisted analysis packets for bounded engineering work.\n- External actors:\n  - architect-reviewer: inspects architecture packets and risk posture.\n  - copilot-cli-adapter: generates and critiques bounded packet content.\n\n## Containers\n- `canon-cli` (Rust CLI): starts runs and exposes inspect/approve/status flows.\n- `canon-engine` (Rust library): owns orchestration, gating, and artifact rendering.\n- `.canon/` (local filesystem runtime store): persists manifests, artifacts, and evidence.\n\n## Components\n- `mode_shaping`: drives `system-shaping` and `architecture` execution paths.\n- `gatekeeper`: evaluates gate readiness from artifact contracts and evidence.\n- `markdown renderer`: materializes reviewable markdown artifacts from authored inputs and AI outputs.\n"
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

    assert_eq!(contract.artifact_requirements.len(), 9);

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
            "context-map.md",
            "readiness-assessment.md",
            "system-context.md",
            "container-view.md",
            "component-view.md",
        ]
    );

    let context_map = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "context-map.md")
        .expect("context map requirement");
    assert_eq!(
        context_map.required_sections,
        vec![
            "Summary",
            "Bounded Contexts",
            "Context Relationships",
            "Integration Seams",
            "Anti-Corruption Candidates",
            "Ownership Boundaries",
            "Shared Invariants",
        ]
    );
    assert_eq!(context_map.gates, vec![GateKind::Architecture, GateKind::Risk]);

    let decisions = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "architecture-decisions.md")
        .expect("architecture decisions requirement");
    assert_eq!(
        decisions.required_sections,
        vec![
            "Summary",
            "Decision",
            "Constraints",
            "Decision Drivers",
            "Recommendation",
            "Consequences",
        ]
    );
    assert_eq!(decisions.gates, vec![GateKind::Architecture, GateKind::Risk]);

    let tradeoff_matrix = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "tradeoff-matrix.md")
        .expect("tradeoff matrix requirement");
    assert_eq!(
        tradeoff_matrix.required_sections,
        vec![
            "Summary",
            "Options Considered",
            "Evaluation Criteria",
            "Pros",
            "Cons",
            "Why Not The Others",
        ]
    );
    assert_eq!(tradeoff_matrix.gates, vec![GateKind::Architecture, GateKind::Risk]);
}

#[test]
fn architecture_gate_blocks_when_required_decision_artifacts_are_missing() {
    let contract = contract_for_mode(Mode::Architecture);
    let artifacts = valid_artifacts(&contract)
        .into_iter()
        .filter(|(file_name, _)| file_name != "context-map.md")
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
        architecture.blockers.iter().any(|blocker| blocker.contains("context-map.md")),
        "architecture gate should cite the missing context-map artifact"
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

    let approval_record = canon_engine::persistence::layout::ProjectLayout::new(workspace.path())
        .run_dir(run_id)
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
