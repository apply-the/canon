use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::approval::{ApprovalDecision, ApprovalRecord};
use canon_engine::domain::artifact::{ArtifactContract, ArtifactRequirement};
use canon_engine::domain::gate::{GateKind, GateStatus};
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::orchestrator::gatekeeper::{
    SupplyChainAnalysisGateContext, evaluate_supply_chain_analysis_gates,
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
fn supply_chain_analysis_mode_uses_a_distinct_operational_packet_bundle() {
    let contract = contract_for_mode(Mode::SupplyChainAnalysis);

    let files = contract
        .artifact_requirements
        .iter()
        .map(|requirement| requirement.file_name.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        files,
        vec![
            "analysis-overview.md",
            "sbom-bundle.md",
            "vulnerability-triage.md",
            "license-compliance.md",
            "legacy-posture.md",
            "policy-decisions.md",
            "analysis-evidence.md",
        ]
    );
}

#[test]
fn supply_chain_analysis_artifacts_require_supply_chain_specific_sections() {
    let contract = contract_for_mode(Mode::SupplyChainAnalysis);

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
                "analysis-overview.md",
                vec![
                    "Summary",
                    "Declared Scope",
                    "Licensing Posture",
                    "Distribution Model",
                    "Ecosystems In Scope",
                    "Out Of Scope Components",
                ],
            ),
            (
                "sbom-bundle.md",
                vec![
                    "Summary",
                    "Scanner Selection Rationale",
                    "SBOM Outputs",
                    "Scanner Decisions",
                    "Coverage Gaps",
                ],
            ),
            (
                "vulnerability-triage.md",
                vec!["Summary", "Findings By Severity", "Exploitability Notes", "Triage Decisions",],
            ),
            (
                "license-compliance.md",
                vec![
                    "Summary",
                    "Compatibility Classes",
                    "Flagged Incompatibilities",
                    "Obligations",
                ],
            ),
            (
                "legacy-posture.md",
                vec![
                    "Summary",
                    "Outdated Dependencies",
                    "End Of Life Signals",
                    "Abandonment Signals",
                    "Modernization Slices",
                ],
            ),
            (
                "policy-decisions.md",
                vec!["Summary", "Scanner Decisions", "Coverage Gaps", "Deferred Verification",],
            ),
            (
                "analysis-evidence.md",
                vec!["Summary", "Source Inputs", "Independent Checks", "Deferred Verification",],
            ),
        ]
    );
}

#[test]
fn supply_chain_analysis_gate_blocks_when_core_review_artifacts_are_missing() {
    let contract = contract_for_mode(Mode::SupplyChainAnalysis);
    let artifacts = valid_artifacts(&contract)
        .into_iter()
        .filter(|(file_name, _)| file_name != "license-compliance.md")
        .collect::<Vec<_>>();

    let gates = evaluate_supply_chain_analysis_gates(
        &contract,
        &artifacts,
        SupplyChainAnalysisGateContext {
            owner: "release-engineer",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    let risk = gates.iter().find(|gate| gate.gate == GateKind::Risk).expect("risk gate");

    assert_eq!(risk.status, GateStatus::Blocked);
    assert!(
        risk.blockers.iter().any(|blocker| blocker.contains("license-compliance.md")),
        "risk gate should cite the missing license artifact"
    );
}

#[test]
fn supply_chain_analysis_gate_requires_approval_for_systemic_red_runs() {
    let contract = contract_for_mode(Mode::SupplyChainAnalysis);
    let artifacts = valid_artifacts(&contract);

    let gates = evaluate_supply_chain_analysis_gates(
        &contract,
        &artifacts,
        SupplyChainAnalysisGateContext {
            owner: "release-engineer",
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
        "principal-release".to_string(),
        ApprovalDecision::Approve,
        "accepted bounded supply-chain packet".to_string(),
        OffsetDateTime::UNIX_EPOCH,
    )];
    let gates = evaluate_supply_chain_analysis_gates(
        &contract,
        &artifacts,
        SupplyChainAnalysisGateContext {
            owner: "release-engineer",
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
fn supply_chain_analysis_release_readiness_requires_independent_validation_and_evidence() {
    let contract = contract_for_mode(Mode::SupplyChainAnalysis);
    let artifacts = valid_artifacts(&contract);

    let gates = evaluate_supply_chain_analysis_gates(
        &contract,
        &artifacts,
        SupplyChainAnalysisGateContext {
            owner: "release-engineer",
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
        blocker
            == "supply-chain-analysis readiness requires persisted context, critique, and verification evidence"
    }));
    assert!(release.blockers.iter().any(|blocker| {
        blocker
            == "analysis readiness requires an independently recorded repository validation path"
    }));
}
