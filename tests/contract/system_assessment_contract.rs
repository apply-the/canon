use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::approval::{ApprovalDecision, ApprovalRecord};
use canon_engine::domain::artifact::{ArtifactContract, ArtifactRequirement};
use canon_engine::domain::gate::{GateKind, GateStatus};
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::orchestrator::gatekeeper::{
    SystemAssessmentGateContext, evaluate_system_assessment_gates,
};
use time::OffsetDateTime;

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
fn system_assessment_mode_uses_a_distinct_assessment_packet_bundle() {
    let contract = contract_for_mode(Mode::SystemAssessment);

    let files = contract
        .artifact_requirements
        .iter()
        .map(|requirement| requirement.file_name.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        files,
        vec![
            "assessment-overview.md",
            "coverage-map.md",
            "asset-inventory.md",
            "functional-view.md",
            "component-view.md",
            "deployment-view.md",
            "technology-view.md",
            "integration-view.md",
            "risk-register.md",
            "assessment-evidence.md",
        ]
    );
}

#[test]
fn system_assessment_artifacts_require_assessment_specific_sections() {
    let contract = contract_for_mode(Mode::SystemAssessment);

    let sections = contract
        .artifact_requirements
        .iter()
        .map(|requirement| {
            (
                requirement.file_name.as_str(),
                requirement.required_sections.iter().map(String::as_str).collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        sections,
        vec![
            (
                "assessment-overview.md",
                vec![
                    "Summary",
                    "Assessment Objective",
                    "Stakeholders",
                    "Primary Concerns",
                    "Assessment Posture",
                ],
            ),
            (
                "coverage-map.md",
                vec![
                    "Summary",
                    "Stakeholder Concerns",
                    "Assessed Views",
                    "Partial Or Skipped Coverage",
                    "Confidence By Surface",
                ],
            ),
            (
                "asset-inventory.md",
                vec![
                    "Summary",
                    "Assessed Assets",
                    "Critical Dependencies",
                    "Boundary Notes",
                    "Ownership Signals",
                ],
            ),
            (
                "functional-view.md",
                vec![
                    "Summary",
                    "Responsibilities",
                    "Primary Flows",
                    "Observed Boundaries",
                    "Confidence Notes",
                ],
            ),
            (
                "component-view.md",
                vec!["Summary", "Components", "Responsibilities", "Interfaces", "Confidence Notes",],
            ),
            (
                "deployment-view.md",
                vec![
                    "Summary",
                    "Execution Environments",
                    "Network And Runtime Boundaries",
                    "Deployment Signals",
                    "Coverage Gaps",
                ],
            ),
            (
                "technology-view.md",
                vec![
                    "Summary",
                    "Technology Stack",
                    "Platform Dependencies",
                    "Version Or Lifecycle Signals",
                    "Evidence Gaps",
                ],
            ),
            (
                "integration-view.md",
                vec![
                    "Summary",
                    "Integrations",
                    "Data Exchanges",
                    "Trust And Failure Boundaries",
                    "Inference Notes",
                ],
            ),
            (
                "risk-register.md",
                vec![
                    "Summary",
                    "Observed Risks",
                    "Risk Triggers",
                    "Impact Notes",
                    "Likely Follow-On Modes",
                ],
            ),
            (
                "assessment-evidence.md",
                vec![
                    "Summary",
                    "Observed Findings",
                    "Inferred Findings",
                    "Assessment Gaps",
                    "Evidence Sources",
                ],
            ),
        ]
    );
}

#[test]
fn system_assessment_gate_blocks_when_coverage_map_is_missing() {
    let contract = contract_for_mode(Mode::SystemAssessment);
    let artifacts = valid_artifacts(&contract)
        .into_iter()
        .filter(|(file_name, _)| file_name != "coverage-map.md")
        .collect::<Vec<_>>();

    let gates = evaluate_system_assessment_gates(
        &contract,
        &artifacts,
        SystemAssessmentGateContext {
            owner: "architecture-lead",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    let architecture =
        gates.iter().find(|gate| gate.gate == GateKind::Architecture).expect("architecture gate");

    assert_eq!(architecture.status, GateStatus::Blocked);
    assert!(
        architecture.blockers.iter().any(|blocker| blocker.contains("coverage-map.md")),
        "architecture gate should cite the missing coverage-map artifact"
    );
}

#[test]
fn system_assessment_gate_requires_approval_for_systemic_red_runs() {
    let contract = contract_for_mode(Mode::SystemAssessment);
    let artifacts = valid_artifacts(&contract);

    let gates = evaluate_system_assessment_gates(
        &contract,
        &artifacts,
        SystemAssessmentGateContext {
            owner: "architecture-lead",
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
        "principal-architect".to_string(),
        ApprovalDecision::Approve,
        "accepted bounded system assessment packet".to_string(),
        OffsetDateTime::UNIX_EPOCH,
    )];
    let gates = evaluate_system_assessment_gates(
        &contract,
        &artifacts,
        SystemAssessmentGateContext {
            owner: "architecture-lead",
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
fn system_assessment_release_readiness_requires_independent_validation_and_evidence() {
    let contract = contract_for_mode(Mode::SystemAssessment);
    let artifacts = valid_artifacts(&contract);

    let gates = evaluate_system_assessment_gates(
        &contract,
        &artifacts,
        SystemAssessmentGateContext {
            owner: "architecture-lead",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            approvals: &[],
            validation_independence_satisfied: false,
            evidence_complete: false,
        },
    );
    let release =
        gates.iter().find(|gate| gate.gate == GateKind::ReleaseReadiness).expect("release gate");

    assert_eq!(release.status, GateStatus::Blocked);
    assert!(release.blockers.iter().any(|blocker| {
        blocker == "system-assessment readiness requires persisted context, critique, and verification evidence"
    }));
    assert!(release.blockers.iter().any(|blocker| {
        blocker
            == "analysis readiness requires an independently recorded repository validation path"
    }));
}
