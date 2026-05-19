use time::OffsetDateTime;

use super::{
    BacklogGateContext, ChangeGateContext, DiscoveryGateContext, DomainLanguageGateContext,
    DomainModelGateContext, ImplementationGateContext, IncidentGateContext, MigrationGateContext,
    PrReviewGateContext, RefactorGateContext, ReviewGateContext, SecurityAssessmentGateContext,
    SystemAssessmentGateContext, VerificationGateContext, evaluate_backlog_gates,
    evaluate_change_gates, evaluate_discovery_gates, evaluate_domain_language_gates,
    evaluate_domain_model_gates, evaluate_implementation_gates, evaluate_incident_gates,
    evaluate_migration_gates, evaluate_pr_review_gates, evaluate_refactor_gates,
    evaluate_requirements_gates, evaluate_review_gates, evaluate_security_assessment_gates,
    evaluate_system_assessment_gates, evaluate_verification_gates, rules,
};
use crate::artifacts::contract::contract_for_mode;
use crate::domain::approval::{ApprovalDecision, ApprovalRecord};
use crate::domain::artifact::{
    ArtifactContract, ArtifactFormat, ArtifactRequirement, artifact_slug,
};
use crate::domain::execution::DeniedInvocation;
use crate::domain::gate::{GateEvaluation, GateKind, GateStatus};
use crate::domain::mode::Mode;
use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::run::{
    ClosureAssessment, ClosureDecompositionScope, ClosureFinding, ClosureFindingSeverity,
    ClosureStatus, SystemContext,
};

fn valid_artifacts(contract: &ArtifactContract) -> Vec<(String, String)> {
    contract
        .artifact_requirements
        .iter()
        .map(|requirement| (requirement.file_name.clone(), render_artifact(requirement, None)))
        .collect()
}

fn render_artifact(requirement: &ArtifactRequirement, trailing: Option<&str>) -> String {
    let mut sections = requirement
        .required_sections
        .iter()
        .map(|section| format!("## {section}\n\nRecorded content for {section}."))
        .collect::<Vec<_>>();

    if let Some(trailing) = trailing {
        sections.push(trailing.to_string());
    }

    sections.join("\n\n")
}

fn gate(evaluations: &[GateEvaluation], kind: GateKind) -> &GateEvaluation {
    evaluations.iter().find(|evaluation| evaluation.gate == kind).expect("gate present")
}

fn artifact_contents_mut<'a>(artifacts: &'a mut [(String, String)], slug: &str) -> &'a mut String {
    &mut artifacts
        .iter_mut()
        .find(|(file_name, _)| artifact_slug(file_name) == slug)
        .expect("artifact present")
        .1
}

fn blocking_closure_assessment(scope: ClosureDecompositionScope) -> ClosureAssessment {
    ClosureAssessment {
        status: ClosureStatus::Blocked,
        findings: vec![ClosureFinding {
            category: "missing-evidence".to_string(),
            severity: ClosureFindingSeverity::Blocking,
            affected_scope: "delivery-slices.md".to_string(),
            recommended_followup: "complete the delivery slice evidence".to_string(),
        }],
        decomposition_scope: scope,
        notes: None,
    }
}

fn artifact_requirement(
    file_name: &str,
    sections: &[&str],
    gates: &[GateKind],
) -> ArtifactRequirement {
    ArtifactRequirement {
        file_name: file_name.to_string(),
        format: ArtifactFormat::Markdown,
        required_sections: sections.iter().map(ToString::to_string).collect(),
        gates: gates.to_vec(),
        required: true,
    }
}

fn optional_artifact_requirement(
    file_name: &str,
    sections: &[&str],
    gates: &[GateKind],
) -> ArtifactRequirement {
    ArtifactRequirement {
        file_name: file_name.to_string(),
        format: ArtifactFormat::Markdown,
        required_sections: sections.iter().map(ToString::to_string).collect(),
        gates: gates.to_vec(),
        required: false,
    }
}

#[test]
fn named_artifact_gate_ignores_missing_optional_artifacts() {
    let contract = ArtifactContract {
        version: 1,
        artifact_requirements: vec![optional_artifact_requirement(
            "dynamic-view.md",
            &["Summary"],
            &[GateKind::Architecture],
        )],
        required_verification_layers: Vec::new(),
    };

    let evaluation = rules::named_artifact_gate(
        GateKind::Architecture,
        &contract,
        &[],
        &["dynamic-view.md"],
        "dynamic view should exist when required",
    );

    assert_eq!(evaluation.status, GateStatus::Passed);
    assert!(evaluation.blockers.is_empty());
}

fn incident_contract() -> ArtifactContract {
    ArtifactContract {
        version: 1,
        artifact_requirements: vec![
            artifact_requirement(
                "incident-frame.md",
                &[
                    "Summary",
                    "Incident Scope",
                    "Trigger And Current State",
                    "Operational Constraints",
                ],
                &[GateKind::Risk, GateKind::IncidentContainment, GateKind::Architecture],
            ),
            artifact_requirement(
                "blast-radius-map.md",
                &["Summary", "Impacted Surfaces", "Propagation Paths", "Confidence And Unknowns"],
                &[GateKind::IncidentContainment],
            ),
            artifact_requirement(
                "containment-plan.md",
                &["Summary", "Immediate Actions", "Ordered Sequence", "Stop Conditions"],
                &[GateKind::IncidentContainment],
            ),
            artifact_requirement(
                "incident-decision-record.md",
                &["Summary", "Decision Points", "Approved Actions", "Deferred Actions"],
                &[GateKind::Architecture],
            ),
            artifact_requirement(
                "follow-up-verification.md",
                &["Summary", "Verification Checks", "Release Readiness", "Follow-Up Work"],
                &[GateKind::ReleaseReadiness],
            ),
        ],
        required_verification_layers: Vec::new(),
    }
}

fn migration_contract() -> ArtifactContract {
    ArtifactContract {
        version: 1,
        artifact_requirements: vec![
            artifact_requirement(
                "source-target-map.md",
                &["Summary", "Current State", "Target State", "Transition Boundaries"],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            artifact_requirement(
                "compatibility-matrix.md",
                &[
                    "Summary",
                    "Guaranteed Compatibility",
                    "Temporary Incompatibilities",
                    "Coexistence Rules",
                ],
                &[GateKind::Architecture, GateKind::MigrationSafety],
            ),
            artifact_requirement(
                "sequencing-plan.md",
                &["Summary", "Ordered Steps", "Parallelizable Work", "Cutover Criteria"],
                &[GateKind::MigrationSafety],
            ),
            artifact_requirement(
                "fallback-plan.md",
                &["Summary", "Rollback Triggers", "Fallback Paths", "Re-Entry Criteria"],
                &[GateKind::MigrationSafety],
            ),
            artifact_requirement(
                "migration-verification-report.md",
                &["Summary", "Verification Checks", "Residual Risks", "Release Readiness"],
                &[GateKind::MigrationSafety, GateKind::ReleaseReadiness],
            ),
            artifact_requirement(
                "decision-record.md",
                &["Summary", "Migration Decisions", "Deferred Decisions", "Approval Notes"],
                &[GateKind::Architecture],
            ),
        ],
        required_verification_layers: Vec::new(),
    }
}

#[test]
fn requirements_release_readiness_collects_evidence_and_runtime_disabled_blockers() {
    let contract = contract_for_mode(Mode::Requirements);
    let artifacts = valid_artifacts(&contract);
    let denied = vec![DeniedInvocation {
        request_id: "req-1".to_string(),
        rationale: "adapter disabled for runtime execution by policy".to_string(),
        policy_refs: Vec::new(),
        recorded_at: OffsetDateTime::UNIX_EPOCH,
    }];

    let evaluations = evaluate_requirements_gates(&contract, &artifacts, "Owner", &denied, false);
    let release = gate(&evaluations, GateKind::ReleaseReadiness);

    assert_eq!(release.status, GateStatus::Blocked);
    assert!(
        release
            .blockers
            .iter()
            .any(|blocker| blocker.contains("explicit generation and validation evidence"))
    );
    assert!(
        release
            .blockers
            .iter()
            .any(|blocker| blocker.contains("runtime-disabled invocation attempts"))
    );
}

#[test]
fn requirements_release_readiness_blocks_raw_input_replay_in_problem_statement() {
    let contract = contract_for_mode(Mode::Requirements);
    let mut artifacts = valid_artifacts(&contract);
    let problem_statement = artifacts
        .iter_mut()
        .find(|(file_name, _)| artifact_slug(file_name) == "problem-statement.md")
        .expect("problem statement artifact");
    problem_statement.1.push_str("\n\n## Input: canon-input/requirements/source.md\n\nRaw dump");

    let evaluations = evaluate_requirements_gates(&contract, &artifacts, "Owner", &[], true);
    let release = gate(&evaluations, GateKind::ReleaseReadiness);

    assert_eq!(release.status, GateStatus::Blocked);
    assert!(release.blockers.iter().any(|blocker| blocker.contains("replaying raw input labels")));
}

#[test]
fn requirements_exploration_and_risk_block_without_problem_statement_or_owner() {
    let contract = contract_for_mode(Mode::Requirements);
    let mut artifacts = valid_artifacts(&contract);
    artifacts.retain(|(file_name, _)| artifact_slug(file_name) != "problem-statement.md");

    let evaluations = evaluate_requirements_gates(&contract, &artifacts, "   ", &[], true);

    let exploration = gate(&evaluations, GateKind::Exploration);
    assert_eq!(exploration.status, GateStatus::Blocked);
    assert!(
        exploration
            .blockers
            .iter()
            .any(|blocker| blocker.contains("problem statement artifact is required"))
    );

    let risk = gate(&evaluations, GateKind::Risk);
    assert_eq!(risk.status, GateStatus::Blocked);
    assert!(risk.blockers.iter().any(|blocker| blocker.contains("human ownership is required")));
}

#[test]
fn change_risk_gate_requires_or_records_approval_for_systemic_red_runs() {
    let contract = contract_for_mode(Mode::Change);
    let artifacts = valid_artifacts(&contract);

    let gated = evaluate_change_gates(
        &contract,
        &artifacts,
        ChangeGateContext {
            owner: "Owner",
            risk: RiskClass::SystemicImpact,
            zone: UsageZone::Red,
            approvals: &[],
            system_context: Some(SystemContext::Existing),
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    assert_eq!(gate(&gated, GateKind::Risk).status, GateStatus::NeedsApproval);

    let approvals = vec![ApprovalRecord::for_gate(
        GateKind::Risk,
        "Approver <approver@example.com>".to_string(),
        ApprovalDecision::Approve,
        "approved for test".to_string(),
        OffsetDateTime::UNIX_EPOCH,
    )];
    let overridden = evaluate_change_gates(
        &contract,
        &artifacts,
        ChangeGateContext {
            owner: "Owner",
            risk: RiskClass::SystemicImpact,
            zone: UsageZone::Red,
            approvals: &approvals,
            system_context: Some(SystemContext::Existing),
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );

    assert_eq!(gate(&overridden, GateKind::Risk).status, GateStatus::Overridden);
}

#[test]
fn implementation_execution_gate_requires_explicit_approval_before_completion() {
    let contract = contract_for_mode(Mode::Implementation);
    let artifacts = valid_artifacts(&contract);

    let pending = evaluate_implementation_gates(
        &contract,
        &artifacts,
        ImplementationGateContext {
            owner: "Owner",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            approvals: &[],
            system_context: Some(SystemContext::Existing),
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    assert_eq!(gate(&pending, GateKind::Execution).status, GateStatus::NeedsApproval);

    let approvals = vec![ApprovalRecord::for_gate(
        GateKind::Execution,
        "Owner <owner@example.com>".to_string(),
        ApprovalDecision::Approve,
        "approved bounded execution".to_string(),
        OffsetDateTime::UNIX_EPOCH,
    )];
    let approved = evaluate_implementation_gates(
        &contract,
        &artifacts,
        ImplementationGateContext {
            owner: "Owner",
            risk: RiskClass::SystemicImpact,
            zone: UsageZone::Red,
            approvals: &approvals,
            system_context: Some(SystemContext::Existing),
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    assert_eq!(gate(&approved, GateKind::Execution).status, GateStatus::Overridden);
}

#[test]
fn incident_gates_require_containment_artifacts_and_risk_approval() {
    let contract = incident_contract();
    let artifacts = valid_artifacts(&contract);

    let gated = evaluate_incident_gates(
        &contract,
        &artifacts,
        IncidentGateContext {
            owner: "Owner",
            risk: RiskClass::SystemicImpact,
            zone: UsageZone::Red,
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );

    assert_eq!(gate(&gated, GateKind::Risk).status, GateStatus::NeedsApproval);
    assert_eq!(gate(&gated, GateKind::IncidentContainment).status, GateStatus::Passed);
    assert_eq!(gate(&gated, GateKind::Architecture).status, GateStatus::Passed);
    assert_eq!(gate(&gated, GateKind::ReleaseReadiness).status, GateStatus::Passed);
}

#[test]
fn incident_gates_block_when_operational_evidence_is_not_captured() {
    let contract = incident_contract();
    let mut artifacts = valid_artifacts(&contract);
    artifacts
        .iter_mut()
        .find(|(file_name, _)| artifact_slug(file_name) == "blast-radius-map.md")
        .expect("blast radius artifact")
        .1
        .push_str("\n\nNOT CAPTURED");

    let gated = evaluate_incident_gates(
        &contract,
        &artifacts,
        IncidentGateContext {
            owner: "Owner",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );

    assert_eq!(gate(&gated, GateKind::IncidentContainment).status, GateStatus::Blocked);
    assert_eq!(gate(&gated, GateKind::ReleaseReadiness).status, GateStatus::Blocked);
}

#[test]
fn migration_gates_require_migration_safety_packet_and_pass_when_present() {
    let contract = migration_contract();
    let artifacts = valid_artifacts(&contract);

    let evaluations = evaluate_migration_gates(
        &contract,
        &artifacts,
        MigrationGateContext {
            owner: "Owner",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );

    assert_eq!(gate(&evaluations, GateKind::Exploration).status, GateStatus::Passed);
    assert_eq!(gate(&evaluations, GateKind::Architecture).status, GateStatus::Passed);
    assert_eq!(gate(&evaluations, GateKind::MigrationSafety).status, GateStatus::Passed);
    assert_eq!(gate(&evaluations, GateKind::Risk).status, GateStatus::Passed);
    assert_eq!(gate(&evaluations, GateKind::ReleaseReadiness).status, GateStatus::Passed);
}

fn assert_analysis_mode_gates(evaluations: &[GateEvaluation], review_gate: GateKind) {
    assert_eq!(gate(evaluations, review_gate).status, GateStatus::Passed);
    assert_eq!(gate(evaluations, GateKind::Risk).status, GateStatus::Passed);
    assert_eq!(gate(evaluations, GateKind::ReleaseReadiness).status, GateStatus::Passed);
}

#[test]
fn discovery_gates_pass_when_analysis_context_is_complete() {
    let contract = contract_for_mode(Mode::Discovery);
    let artifacts = valid_artifacts(&contract);

    let evaluations = evaluate_discovery_gates(
        &contract,
        &artifacts,
        DiscoveryGateContext {
            owner: "Owner",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );

    assert_analysis_mode_gates(&evaluations, GateKind::Exploration);
}

#[test]
fn analysis_mode_gate_sets_pass_for_security_system_and_domain_profiles() {
    let security_contract = contract_for_mode(Mode::SecurityAssessment);
    let security_artifacts = valid_artifacts(&security_contract);
    let security = evaluate_security_assessment_gates(
        &security_contract,
        &security_artifacts,
        SecurityAssessmentGateContext {
            owner: "Owner",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    assert_analysis_mode_gates(&security, GateKind::Architecture);

    let system_contract = contract_for_mode(Mode::SystemAssessment);
    let system_artifacts = valid_artifacts(&system_contract);
    let system = evaluate_system_assessment_gates(
        &system_contract,
        &system_artifacts,
        SystemAssessmentGateContext {
            owner: "Owner",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    assert_analysis_mode_gates(&system, GateKind::Architecture);

    let language_contract = contract_for_mode(Mode::DomainLanguage);
    let language_artifacts = valid_artifacts(&language_contract);
    let language = evaluate_domain_language_gates(
        &language_contract,
        &language_artifacts,
        DomainLanguageGateContext {
            owner: "Owner",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    assert_analysis_mode_gates(&language, GateKind::Architecture);

    let model_contract = contract_for_mode(Mode::DomainModel);
    let model_artifacts = valid_artifacts(&model_contract);
    let model = evaluate_domain_model_gates(
        &model_contract,
        &model_artifacts,
        DomainModelGateContext {
            owner: "Owner",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    assert_analysis_mode_gates(&model, GateKind::Architecture);
}

#[test]
fn migration_gates_block_when_fallback_plan_is_not_captured() {
    let contract = migration_contract();
    let mut artifacts = valid_artifacts(&contract);
    artifacts
        .iter_mut()
        .find(|(file_name, _)| artifact_slug(file_name) == "fallback-plan.md")
        .expect("fallback plan artifact")
        .1
        .push_str("\n\nNOT CAPTURED");

    let evaluations = evaluate_migration_gates(
        &contract,
        &artifacts,
        MigrationGateContext {
            owner: "Owner",
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );

    assert_eq!(gate(&evaluations, GateKind::MigrationSafety).status, GateStatus::Blocked);
    assert_eq!(gate(&evaluations, GateKind::ReleaseReadiness).status, GateStatus::Blocked);
}

#[test]
fn refactor_execution_gate_requires_explicit_approval_before_completion() {
    let contract = contract_for_mode(Mode::Refactor);
    let artifacts = valid_artifacts(&contract);

    let pending = evaluate_refactor_gates(
        &contract,
        &artifacts,
        RefactorGateContext {
            owner: "Owner",
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            approvals: &[],
            system_context: Some(SystemContext::Existing),
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    assert_eq!(gate(&pending, GateKind::Execution).status, GateStatus::NeedsApproval);

    let approvals = vec![ApprovalRecord::for_gate(
        GateKind::Execution,
        "Owner <owner@example.com>".to_string(),
        ApprovalDecision::Approve,
        "approved bounded refactor execution".to_string(),
        OffsetDateTime::UNIX_EPOCH,
    )];
    let approved = evaluate_refactor_gates(
        &contract,
        &artifacts,
        RefactorGateContext {
            owner: "Owner",
            risk: RiskClass::SystemicImpact,
            zone: UsageZone::Red,
            approvals: &approvals,
            system_context: Some(SystemContext::Existing),
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    assert_eq!(gate(&approved, GateKind::Execution).status, GateStatus::Overridden);
}

#[test]
fn change_release_readiness_blocks_without_evidence_or_independent_validation() {
    let contract = contract_for_mode(Mode::Change);
    let artifacts = valid_artifacts(&contract);

    let evaluations = evaluate_change_gates(
        &contract,
        &artifacts,
        ChangeGateContext {
            owner: "Owner",
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            approvals: &[],
            system_context: Some(SystemContext::Existing),
            validation_independence_satisfied: false,
            evidence_complete: false,
        },
    );
    let release = gate(&evaluations, GateKind::ReleaseReadiness);

    assert_eq!(release.status, GateStatus::Blocked);
    assert!(
        release
            .blockers
            .iter()
            .any(|blocker| blocker.contains("explicit generation and validation evidence"))
    );
    assert!(
        release
            .blockers
            .iter()
            .any(|blocker| blocker.contains("independently recorded validation path"))
    );
}

#[test]
fn change_preservation_blocks_without_existing_system_context() {
    let contract = contract_for_mode(Mode::Change);
    let artifacts = valid_artifacts(&contract);

    let evaluations = evaluate_change_gates(
        &contract,
        &artifacts,
        ChangeGateContext {
            owner: "Owner",
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            approvals: &[],
            system_context: Some(SystemContext::New),
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    let preservation = gate(&evaluations, GateKind::ChangePreservation);

    assert_eq!(preservation.status, GateStatus::Blocked);
    assert!(
        preservation.blockers.iter().any(|blocker| blocker.contains("system_context = existing"))
    );
}

#[test]
fn pr_review_architecture_gate_blocks_boundary_and_contract_drift_statuses() {
    let contract = contract_for_mode(Mode::PrReview);
    let mut artifacts = valid_artifacts(&contract);

    let boundary_check = artifacts
        .iter_mut()
        .find(|(file_name, _)| artifact_slug(file_name) == "boundary-check.md")
        .expect("boundary-check artifact present");
    boundary_check.1.push_str("\n\nStatus: missing-boundary-review");

    let contract_drift = artifacts
        .iter_mut()
        .find(|(file_name, _)| artifact_slug(file_name) == "contract-drift.md")
        .expect("contract-drift artifact present");
    contract_drift.1.push_str("\n\nStatus: unsupported-contract-drift");

    let evaluations = evaluate_pr_review_gates(
        &contract,
        &artifacts,
        PrReviewGateContext {
            owner: "Reviewer",
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            approvals: &[],
            denied_invocations: &[],
            evidence_complete: true,
        },
    );
    let architecture = gate(&evaluations, GateKind::Architecture);

    assert_eq!(architecture.status, GateStatus::Blocked);
    assert!(
        architecture
            .blockers
            .iter()
            .any(|blocker| blocker.contains("boundary review is incomplete"))
    );
    assert!(
        architecture.blockers.iter().any(|blocker| blocker.contains("unsupported contract drift"))
    );
}

#[test]
fn pr_review_disposition_gate_respects_explicit_review_disposition_approval() {
    let contract = contract_for_mode(Mode::PrReview);
    let mut artifacts = valid_artifacts(&contract);
    let review_summary = artifacts
        .iter_mut()
        .find(|(file_name, _)| artifact_slug(file_name) == "review-summary.md")
        .expect("review-summary artifact present");
    review_summary.1.push_str("\n\nStatus: awaiting-disposition");

    let gated = evaluate_pr_review_gates(
        &contract,
        &artifacts,
        PrReviewGateContext {
            owner: "Reviewer",
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            approvals: &[],
            denied_invocations: &[],
            evidence_complete: true,
        },
    );
    assert_eq!(gate(&gated, GateKind::ReviewDisposition).status, GateStatus::NeedsApproval);

    let approvals = vec![ApprovalRecord::for_gate(
        GateKind::ReviewDisposition,
        "Reviewer <reviewer@example.com>".to_string(),
        ApprovalDecision::Approve,
        "resolved must-fix findings".to_string(),
        OffsetDateTime::UNIX_EPOCH,
    )];
    let approved = evaluate_pr_review_gates(
        &contract,
        &artifacts,
        PrReviewGateContext {
            owner: "Reviewer",
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            approvals: &approvals,
            denied_invocations: &[],
            evidence_complete: true,
        },
    );

    assert_eq!(gate(&approved, GateKind::ReviewDisposition).status, GateStatus::Overridden);
}

#[test]
fn pr_review_release_readiness_collects_evidence_disposition_and_runtime_blockers() {
    let contract = contract_for_mode(Mode::PrReview);
    let mut artifacts = valid_artifacts(&contract);
    let review_summary = artifacts
        .iter_mut()
        .find(|(file_name, _)| artifact_slug(file_name) == "review-summary.md")
        .expect("review-summary artifact present");
    review_summary.1.push_str("\n\nStatus: awaiting-disposition");

    let denied = vec![DeniedInvocation {
        request_id: "req-pr-1".to_string(),
        rationale: "adapter disabled for runtime execution by policy".to_string(),
        policy_refs: Vec::new(),
        recorded_at: OffsetDateTime::UNIX_EPOCH,
    }];

    let evaluations = evaluate_pr_review_gates(
        &contract,
        &artifacts,
        PrReviewGateContext {
            owner: "Reviewer",
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            approvals: &[],
            denied_invocations: &denied,
            evidence_complete: false,
        },
    );
    let release = gate(&evaluations, GateKind::ReleaseReadiness);

    assert_eq!(release.status, GateStatus::Blocked);
    assert!(
        release
            .blockers
            .iter()
            .any(|blocker| blocker.contains("persisted diff inspection and critique evidence"))
    );
    assert!(
        release.blockers.iter().any(|blocker| {
            blocker.contains("unresolved must-fix findings without disposition")
        })
    );
    assert!(
        release
            .blockers
            .iter()
            .any(|blocker| blocker.contains("runtime-disabled pr-review invocation attempts"))
    );
}

#[test]
fn change_preservation_gate_ignores_missing_optional_artifact() {
    let contract = ArtifactContract {
        version: 1,
        artifact_requirements: vec![
            artifact_requirement(
                "legacy-invariants.md",
                &["Legacy Invariants"],
                &[GateKind::ChangePreservation],
            ),
            optional_artifact_requirement(
                "change-surface.md",
                &[],
                &[GateKind::ChangePreservation],
            ),
        ],
        required_verification_layers: Vec::new(),
    };
    let artifacts =
        vec![("legacy-invariants.md".to_string(), "## Legacy Invariants\n\nBounded.".to_string())];

    let evaluations = evaluate_change_gates(
        &contract,
        &artifacts,
        ChangeGateContext {
            owner: "owner",
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            approvals: &[],
            system_context: Some(SystemContext::Existing),
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    let preservation = gate(&evaluations, GateKind::ChangePreservation);

    assert_eq!(preservation.status, GateStatus::Passed);
    assert!(
        !preservation.blockers.iter().any(|blocker| blocker.contains("change-surface.md")),
        "optional artifact absence must not produce a blocker"
    );
}

#[test]
fn implementation_readiness_gate_ignores_missing_optional_artifact() {
    let contract = ArtifactContract {
        version: 1,
        artifact_requirements: vec![
            artifact_requirement(
                "task-mapping.md",
                &["Task Mapping"],
                &[GateKind::ImplementationReadiness],
            ),
            optional_artifact_requirement(
                "mutation-bounds.md",
                &[],
                &[GateKind::ImplementationReadiness],
            ),
        ],
        required_verification_layers: Vec::new(),
    };
    let artifacts =
        vec![("task-mapping.md".to_string(), "## Task Mapping\n\nTask list.".to_string())];

    let evaluations = evaluate_implementation_gates(
        &contract,
        &artifacts,
        ImplementationGateContext {
            owner: "owner",
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            approvals: &[],
            system_context: Some(SystemContext::Existing),
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    let readiness = gate(&evaluations, GateKind::ImplementationReadiness);

    assert_eq!(readiness.status, GateStatus::Passed);
    assert!(
        !readiness.blockers.iter().any(|blocker| blocker.contains("mutation-bounds.md")),
        "optional artifact absence must not produce a blocker"
    );
}

#[test]
fn refactor_preservation_gate_ignores_missing_optional_artifact() {
    let contract = ArtifactContract {
        version: 1,
        artifact_requirements: vec![
            artifact_requirement(
                "preserved-behavior.md",
                &["Preserved Behavior"],
                &[GateKind::ChangePreservation],
            ),
            optional_artifact_requirement(
                "refactor-scope.md",
                &[],
                &[GateKind::ChangePreservation],
            ),
        ],
        required_verification_layers: Vec::new(),
    };
    let artifacts = vec![(
        "preserved-behavior.md".to_string(),
        "## Preserved Behavior\n\nBounded refactor.".to_string(),
    )];

    let evaluations = evaluate_refactor_gates(
        &contract,
        &artifacts,
        RefactorGateContext {
            owner: "owner",
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            approvals: &[],
            system_context: Some(SystemContext::Existing),
            validation_independence_satisfied: true,
            evidence_complete: true,
        },
    );
    let preservation = gate(&evaluations, GateKind::ChangePreservation);

    assert_eq!(preservation.status, GateStatus::Passed);
    assert!(
        !preservation.blockers.iter().any(|blocker| blocker.contains("refactor-scope.md")),
        "optional artifact absence must not produce a blocker"
    );
}

#[test]
fn backlog_gates_surface_closure_findings_and_allow_risk_only_scope() {
    let contract = contract_for_mode(Mode::Backlog);
    let artifacts = valid_artifacts(&contract);

    let blocked = evaluate_backlog_gates(
        &contract,
        &artifacts,
        BacklogGateContext {
            owner: "Owner",
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            system_context: Some(SystemContext::Existing),
            approvals: &[],
            validation_independence_satisfied: true,
            evidence_complete: true,
            closure_assessment: &blocking_closure_assessment(ClosureDecompositionScope::FullPacket),
        },
    );
    let architecture = gate(&blocked, GateKind::Architecture);

    assert_eq!(architecture.status, GateStatus::Blocked);
    assert!(architecture.blockers.iter().any(|blocker| {
        blocker.contains("closure finding `missing-evidence` on delivery-slices.md")
    }));

    let risk_only = rules::backlog_architecture_gate(
        &contract,
        &[],
        Some(SystemContext::Existing),
        &ClosureAssessment {
            status: ClosureStatus::Sufficient,
            findings: Vec::new(),
            decomposition_scope: ClosureDecompositionScope::RiskOnlyPacket,
            notes: None,
        },
    );

    assert_eq!(risk_only.status, GateStatus::Passed);
    assert!(risk_only.blockers.is_empty());

    let missing_existing_context = rules::backlog_architecture_gate(
        &contract,
        &[],
        Some(SystemContext::New),
        &ClosureAssessment {
            status: ClosureStatus::Sufficient,
            findings: Vec::new(),
            decomposition_scope: ClosureDecompositionScope::RiskOnlyPacket,
            notes: None,
        },
    );

    assert_eq!(missing_existing_context.status, GateStatus::Blocked);
    assert!(missing_existing_context.blockers.iter().any(|blocker| {
        blocker.contains("backlog planning requires `system_context = existing`")
    }));
}

#[test]
fn review_gates_block_on_pending_disposition_and_open_missing_evidence() {
    let contract = contract_for_mode(Mode::Review);
    let mut artifacts = valid_artifacts(&contract);
    artifact_contents_mut(&mut artifacts, "review-disposition.md")
        .push_str("\n\nStatus: awaiting-disposition");
    artifact_contents_mut(&mut artifacts, "missing-evidence.md")
        .push_str("\n\nStatus: missing-evidence-open");

    let evaluations = evaluate_review_gates(
        &contract,
        &artifacts,
        ReviewGateContext {
            owner: "Reviewer",
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            approvals: &[],
            evidence_complete: true,
        },
    );

    assert_eq!(gate(&evaluations, GateKind::ReviewDisposition).status, GateStatus::NeedsApproval);
    let release = gate(&evaluations, GateKind::ReleaseReadiness);
    assert_eq!(release.status, GateStatus::Blocked);
    assert!(release.blockers.iter().any(|blocker| {
        blocker.contains("review-disposition.md still records unresolved disposition work")
    }));
    assert!(
        release
            .blockers
            .iter()
            .any(|blocker| { blocker.contains("review packet still records open evidence gaps") })
    );
}

#[test]
fn verification_gates_block_on_unresolved_findings_and_unsupported_verdict() {
    let contract = contract_for_mode(Mode::Verification);
    let mut artifacts = valid_artifacts(&contract);
    artifact_contents_mut(&mut artifacts, "unresolved-findings.md")
        .push_str("\n\nStatus: unresolved-findings-open");
    artifact_contents_mut(&mut artifacts, "verification-report.md")
        .push_str("\n\nStatus: unsupported");

    let evaluations = evaluate_verification_gates(
        &contract,
        &artifacts,
        VerificationGateContext {
            owner: "Verifier",
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            approvals: &[],
            validation_independence_satisfied: false,
            evidence_complete: false,
        },
    );
    let release = gate(&evaluations, GateKind::ReleaseReadiness);

    assert_eq!(release.status, GateStatus::Blocked);
    assert!(
        release
            .blockers
            .iter()
            .any(|blocker| { blocker.contains("persisted challenge and validation evidence") })
    );
    assert!(
        release
            .blockers
            .iter()
            .any(|blocker| { blocker.contains("independently recorded validation path") })
    );
    assert!(
        release.blockers.iter().any(|blocker| {
            blocker.contains("unresolved findings that block release readiness")
        })
    );
    assert!(release.blockers.iter().any(|blocker| {
        blocker.contains("verification-report.md still records an unsupported verdict")
    }));
}

#[test]
fn named_artifact_gate_blocks_on_insufficient_evidence_markers() {
    let contract = ArtifactContract {
        version: 1,
        artifact_requirements: vec![artifact_requirement(
            "dynamic-view.md",
            &[],
            &[GateKind::Architecture],
        )],
        required_verification_layers: Vec::new(),
    };

    let evaluation = rules::named_artifact_gate(
        GateKind::Architecture,
        &contract,
        &[(
            "dynamic-view.md".to_string(),
            "Insufficient evidence: still gathering proof.".to_string(),
        )],
        &["dynamic-view.md"],
        "dynamic view should exist when required",
    );

    assert_eq!(evaluation.status, GateStatus::Blocked);
    assert!(
        evaluation
            .blockers
            .iter()
            .any(|blocker| { blocker.contains("dynamic-view.md lacks sufficient evidence") })
    );
}

#[test]
fn direct_risk_and_execution_helpers_cover_owner_and_invocation_paths() {
    let owner_missing = rules::approval_aware_risk_gate(
        "   ",
        RiskClass::LowImpact,
        UsageZone::Green,
        &[],
        "unused",
    );
    assert_eq!(owner_missing.status, GateStatus::Blocked);
    assert!(owner_missing.blockers.iter().any(|blocker| {
        blocker.contains("human ownership is required before risk classification can pass")
    }));

    let invocation_approval = [ApprovalRecord::for_invocation(
        "req-7".to_string(),
        "Approver <approver@example.com>".to_string(),
        ApprovalDecision::Approve,
        "approved by invocation".to_string(),
        OffsetDateTime::UNIX_EPOCH,
    )];
    let invocation_overridden = rules::change_risk_gate(
        "Owner",
        RiskClass::SystemicImpact,
        UsageZone::Red,
        &invocation_approval,
    );
    assert_eq!(invocation_overridden.status, GateStatus::Overridden);

    let execution_blocked = rules::execution_gate("", &[], "approval required");
    assert_eq!(execution_blocked.status, GateStatus::Blocked);
    assert!(execution_blocked.blockers.iter().any(|blocker| {
        blocker.contains("human ownership is required before execution approval can be requested")
    }));
}

#[test]
fn preservation_helpers_block_on_missing_required_artifacts_and_wrong_context() {
    let change_contract = ArtifactContract {
        version: 1,
        artifact_requirements: vec![artifact_requirement(
            "legacy-invariants.md",
            &["Legacy Invariants"],
            &[GateKind::ChangePreservation],
        )],
        required_verification_layers: Vec::new(),
    };
    let change = rules::change_preservation_gate(&change_contract, &[], Some(SystemContext::New));
    assert_eq!(change.status, GateStatus::Blocked);
    assert!(change.blockers.iter().any(|blocker| blocker.contains("legacy-invariants.md")));
    assert!(change.blockers.iter().any(|blocker| blocker.contains("system_context = existing")));

    let implementation_contract = ArtifactContract {
        version: 1,
        artifact_requirements: vec![artifact_requirement(
            "task-mapping.md",
            &["Task Mapping"],
            &[GateKind::ImplementationReadiness],
        )],
        required_verification_layers: Vec::new(),
    };
    let implementation = rules::implementation_readiness_gate(
        &implementation_contract,
        &[],
        Some(SystemContext::New),
    );
    assert_eq!(implementation.status, GateStatus::Blocked);
    assert!(implementation.blockers.iter().any(|blocker| blocker.contains("task-mapping.md")));
    assert!(implementation.blockers.iter().any(|blocker| {
        blocker.contains("implementation planning requires `system_context = existing`")
    }));

    let refactor_contract = ArtifactContract {
        version: 1,
        artifact_requirements: vec![artifact_requirement(
            "preserved-behavior.md",
            &["Preserved Behavior"],
            &[GateKind::ChangePreservation],
        )],
        required_verification_layers: Vec::new(),
    };
    let refactor =
        rules::refactor_preservation_gate(&refactor_contract, &[], Some(SystemContext::New));
    assert_eq!(refactor.status, GateStatus::Blocked);
    assert!(refactor.blockers.iter().any(|blocker| blocker.contains("preserved-behavior.md")));
    assert!(refactor.blockers.iter().any(|blocker| {
        blocker.contains("refactor preservation requires `system_context = existing`")
    }));
}

#[test]
fn readiness_helpers_surface_missing_evidence_and_validation_requirements() {
    let contract = ArtifactContract {
        version: 1,
        artifact_requirements: Vec::new(),
        required_verification_layers: Vec::new(),
    };

    let analysis = rules::analysis_release_readiness_gate(
        GateKind::ReleaseReadiness,
        &contract,
        &[],
        false,
        false,
        "analysis evidence missing",
    );
    assert_eq!(analysis.status, GateStatus::Blocked);
    assert!(analysis.blockers.iter().any(|blocker| blocker.contains("analysis evidence missing")));
    assert!(
        analysis.blockers.iter().any(|blocker| {
            blocker.contains("independently recorded repository validation path")
        })
    );

    let implementation = rules::implementation_release_readiness_gate(&contract, &[], false, false);
    assert_eq!(implementation.status, GateStatus::Blocked);
    assert!(implementation.blockers.iter().any(|blocker| {
        blocker
            .contains("implementation readiness needs explicit generation and validation evidence")
    }));
    assert!(implementation.blockers.iter().any(|blocker| {
        blocker
            .contains("implementation readiness requires an independently recorded validation path")
    }));

    let refactor = rules::refactor_release_readiness_gate(&contract, &[], false, false);
    assert_eq!(refactor.status, GateStatus::Blocked);
    assert!(refactor.blockers.iter().any(|blocker| {
        blocker.contains("refactor readiness needs explicit generation and validation evidence")
    }));
    assert!(refactor.blockers.iter().any(|blocker| {
        blocker.contains("refactor readiness requires an independently recorded validation path")
    }));
}

#[test]
fn review_and_pr_helpers_cover_missing_artifact_and_approved_paths() {
    let risk_approved = rules::approval_aware_risk_gate(
        "Owner",
        RiskClass::SystemicImpact,
        UsageZone::Red,
        &[ApprovalRecord::for_gate(
            GateKind::Risk,
            "Owner <owner@example.com>".to_string(),
            ApprovalDecision::Approve,
            "approved high-risk review".to_string(),
            OffsetDateTime::UNIX_EPOCH,
        )],
        "unused",
    );
    assert_eq!(risk_approved.status, GateStatus::Overridden);

    let pr_review_contract = contract_for_mode(Mode::PrReview);
    let pr_architecture = rules::pr_review_architecture_gate(&pr_review_contract, &[]);
    assert_eq!(pr_architecture.status, GateStatus::Blocked);
    assert!(
        pr_architecture
            .blockers
            .iter()
            .any(|blocker| blocker.contains("missing required artifact"))
    );

    let review_contract = contract_for_mode(Mode::Review);
    let disposition = rules::review_disposition_gate_for_file(
        &review_contract,
        &[],
        "review-disposition.md",
        false,
    );
    assert_eq!(disposition.status, GateStatus::Blocked);
    assert!(
        disposition
            .blockers
            .iter()
            .any(|blocker| blocker.contains("missing required artifact `review-disposition.md`"))
    );

    let review_release = rules::review_release_readiness_gate(
        &review_contract,
        &valid_artifacts(&review_contract),
        false,
        false,
    );
    assert_eq!(review_release.status, GateStatus::Blocked);
    assert!(review_release.blockers.iter().any(|blocker| {
        blocker.contains(
            "review readiness requires persisted context, critique, and validation evidence",
        )
    }));
}
