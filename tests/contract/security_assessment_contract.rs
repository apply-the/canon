use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::approval::{ApprovalDecision, ApprovalRecord};
use canon_engine::domain::artifact::{ArtifactContract, ArtifactRequirement};
use canon_engine::domain::gate::{GateKind, GateStatus};
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::orchestrator::gatekeeper::{
    SecurityAssessmentGateContext, evaluate_security_assessment_gates,
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
fn security_assessment_mode_uses_a_distinct_security_packet_bundle() {
    let contract = contract_for_mode(Mode::SecurityAssessment);

    let files = contract
        .artifact_requirements
        .iter()
        .map(|requirement| requirement.file_name.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        files,
        vec![
            "assessment-overview.md",
            "threat-model.md",
            "risk-register.md",
            "mitigations.md",
            "assumptions-and-gaps.md",
            "compliance-anchors.md",
            "assessment-evidence.md",
        ]
    );
}

#[test]
fn security_assessment_artifacts_require_security_specific_sections() {
    let contract = contract_for_mode(Mode::SecurityAssessment);

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
                    "Assessment Scope",
                    "In-Scope Assets",
                    "Trust Boundaries",
                    "Out Of Scope",
                ],
            ),
            (
                "threat-model.md",
                vec!["Summary", "Threat Inventory", "Attacker Goals", "Boundary Threats",],
            ),
            (
                "risk-register.md",
                vec!["Summary", "Risk Findings", "Likelihood And Impact", "Proposed Owners",],
            ),
            (
                "mitigations.md",
                vec!["Summary", "Recommended Controls", "Tradeoffs", "Sequencing Notes",],
            ),
            (
                "assumptions-and-gaps.md",
                vec!["Summary", "Assumptions", "Evidence Gaps", "Unobservable Surfaces",],
            ),
            (
                "compliance-anchors.md",
                vec!["Summary", "Applicable Standards", "Control Families", "Scope Limits",],
            ),
            (
                "assessment-evidence.md",
                vec!["Summary", "Source Inputs", "Independent Checks", "Deferred Verification",],
            ),
        ]
    );
}

#[test]
fn security_assessment_gate_blocks_when_core_review_artifacts_are_missing() {
    let contract = contract_for_mode(Mode::SecurityAssessment);
    let artifacts = valid_artifacts(&contract)
        .into_iter()
        .filter(|(file_name, _)| file_name != "mitigations.md")
        .collect::<Vec<_>>();

    let gates = evaluate_security_assessment_gates(
        &contract,
        &artifacts,
        SecurityAssessmentGateContext {
            owner: "security-lead",
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
        architecture.blockers.iter().any(|blocker| blocker.contains("mitigations.md")),
        "architecture gate should cite the missing mitigation artifact"
    );
}

#[test]
fn security_assessment_gate_requires_approval_for_systemic_red_runs() {
    let contract = contract_for_mode(Mode::SecurityAssessment);
    let artifacts = valid_artifacts(&contract);

    let gates = evaluate_security_assessment_gates(
        &contract,
        &artifacts,
        SecurityAssessmentGateContext {
            owner: "security-lead",
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
        "principal-security".to_string(),
        ApprovalDecision::Approve,
        "accepted bounded security packet".to_string(),
        OffsetDateTime::UNIX_EPOCH,
    )];
    let gates = evaluate_security_assessment_gates(
        &contract,
        &artifacts,
        SecurityAssessmentGateContext {
            owner: "security-lead",
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
fn security_assessment_release_readiness_requires_independent_validation_and_evidence() {
    let contract = contract_for_mode(Mode::SecurityAssessment);
    let artifacts = valid_artifacts(&contract);

    let gates = evaluate_security_assessment_gates(
        &contract,
        &artifacts,
        SecurityAssessmentGateContext {
            owner: "security-lead",
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
        blocker == "security-assessment readiness requires persisted context, critique, and verification evidence"
    }));
    assert!(release.blockers.iter().any(|blocker| {
        blocker
            == "analysis readiness requires an independently recorded repository validation path"
    }));
}
