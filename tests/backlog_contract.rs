use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::artifact::{ArtifactContract, ArtifactRequirement};
use canon_engine::domain::gate::{GateKind, GateStatus};
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{
    ClosureAssessment, ClosureDecompositionScope, ClosureFinding, ClosureFindingSeverity,
    ClosureStatus, SystemContext,
};
use canon_engine::orchestrator::gatekeeper::{BacklogGateContext, evaluate_backlog_gates};

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
fn backlog_contract_matches_spec_artifact_names_sections_and_gates() {
    let contract = contract_for_mode(Mode::Backlog);

    assert_eq!(contract.artifact_requirements.len(), 8);

    let names = contract
        .artifact_requirements
        .iter()
        .map(|requirement| requirement.file_name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        vec![
            "backlog-overview.md",
            "epic-tree.md",
            "capability-to-epic-map.md",
            "dependency-map.md",
            "delivery-slices.md",
            "sequencing-plan.md",
            "acceptance-anchors.md",
            "planning-risks.md",
        ]
    );

    let overview = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "backlog-overview.md")
        .expect("backlog overview requirement");
    assert_eq!(
        overview.required_sections,
        vec![
            "Summary",
            "Scope",
            "Planning Horizon",
            "Source Inputs",
            "Delivery Intent",
            "Decomposition Posture",
        ]
    );
    assert_eq!(overview.gates, vec![GateKind::Exploration, GateKind::Risk]);
}

#[test]
fn backlog_gates_pass_for_a_full_packet_with_sufficient_closure() {
    let contract = contract_for_mode(Mode::Backlog);
    let artifacts = valid_artifacts(&contract);

    let gates = evaluate_backlog_gates(
        &contract,
        &artifacts,
        BacklogGateContext {
            owner: "planner",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(SystemContext::Existing),
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
            closure_assessment: &ClosureAssessment::sufficient(),
        },
    );

    assert!(
        gates.iter().all(|gate| gate.status == GateStatus::Passed),
        "expected all backlog gates to pass, got: {gates:?}"
    );
}

#[test]
fn backlog_closure_limited_packet_only_requires_overview_and_risks() {
    let contract = contract_for_mode(Mode::Backlog);
    let overview = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "backlog-overview.md")
        .expect("backlog overview requirement");
    let risks = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "planning-risks.md")
        .expect("planning risks requirement");
    let artifacts = vec![
        (overview.file_name.clone(), render_artifact(overview)),
        (risks.file_name.clone(), render_artifact(risks)),
    ];

    let gates = evaluate_backlog_gates(
        &contract,
        &artifacts,
        BacklogGateContext {
            owner: "planner",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(SystemContext::Existing),
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
            closure_assessment: &ClosureAssessment {
                status: ClosureStatus::Blocked,
                findings: vec![ClosureFinding {
                    category: "missing-capability-boundary".to_string(),
                    severity: ClosureFindingSeverity::Blocking,
                    affected_scope: "whole-run".to_string(),
                    recommended_followup: "Return to architecture before decomposition."
                        .to_string(),
                }],
                decomposition_scope: ClosureDecompositionScope::RiskOnlyPacket,
                notes: Some("Source architecture is not sufficiently closed.".to_string()),
            },
        },
    );

    let exploration =
        gates.iter().find(|gate| gate.gate == GateKind::Exploration).expect("exploration gate");
    assert_eq!(exploration.status, GateStatus::Passed);

    let architecture =
        gates.iter().find(|gate| gate.gate == GateKind::Architecture).expect("architecture gate");
    assert_eq!(architecture.status, GateStatus::Blocked);
    assert!(
        architecture.blockers.iter().any(|blocker| blocker.contains("missing-capability-boundary")),
        "expected closure finding blockers, got: {:?}",
        architecture.blockers
    );
    assert!(
        !architecture
            .blockers
            .iter()
            .any(|blocker| blocker.contains("epic-tree.md")
                || blocker.contains("delivery-slices.md")),
        "risk-only backlog packets should not require full decomposition artifacts, got: {:?}",
        architecture.blockers
    );

    let release = gates
        .iter()
        .find(|gate| gate.gate == GateKind::ReleaseReadiness)
        .expect("release readiness gate");
    assert_eq!(release.status, GateStatus::Passed);
}
