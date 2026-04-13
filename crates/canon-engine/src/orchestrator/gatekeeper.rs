use time::OffsetDateTime;

use crate::artifacts::contract::validate_release_bundle;
use crate::domain::approval::ApprovalRecord;
use crate::domain::artifact::ArtifactContract;
use crate::domain::execution::DeniedInvocation;
use crate::domain::gate::{GateEvaluation, GateKind, GateStatus};
use crate::domain::policy::{RiskClass, UsageZone};

pub struct BrownfieldGateContext<'a> {
    pub owner: &'a str,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub approvals: &'a [ApprovalRecord],
    pub validation_independence_satisfied: bool,
    pub evidence_complete: bool,
}

pub struct PrReviewGateContext<'a> {
    pub owner: &'a str,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub approvals: &'a [ApprovalRecord],
    pub denied_invocations: &'a [DeniedInvocation],
    pub evidence_complete: bool,
}

pub fn evaluate_requirements_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    owner: &str,
    denied_invocations: &[DeniedInvocation],
    evidence_complete: bool,
) -> Vec<GateEvaluation> {
    vec![
        exploration_gate(artifacts),
        risk_gate(owner),
        requirements_release_readiness_gate(
            contract,
            artifacts,
            denied_invocations,
            evidence_complete,
        ),
    ]
}

pub fn evaluate_brownfield_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: BrownfieldGateContext<'_>,
) -> Vec<GateEvaluation> {
    vec![
        named_artifact_gate(
            GateKind::Exploration,
            contract,
            artifacts,
            &["system-slice.md"],
            "brownfield exploration requires a bounded system slice",
        ),
        named_artifact_gate(
            GateKind::BrownfieldPreservation,
            contract,
            artifacts,
            &["legacy-invariants.md", "change-surface.md"],
            "brownfield preservation requires preserved behavior and a named change surface",
        ),
        named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["implementation-plan.md", "decision-record.md"],
            "brownfield architecture review requires an implementation plan and decision record",
        ),
        brownfield_risk_gate(context.owner, context.risk, context.zone, context.approvals),
        brownfield_release_readiness_gate(
            contract,
            artifacts,
            context.validation_independence_satisfied,
            context.evidence_complete,
        ),
    ]
}

pub fn evaluate_pr_review_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: PrReviewGateContext<'_>,
) -> Vec<GateEvaluation> {
    let disposition_approved = context.approvals.iter().any(|approval| {
        approval.matches_gate(GateKind::ReviewDisposition) && approval.is_approved()
    });

    vec![
        approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone review work requires explicit approval before it can proceed",
        ),
        pr_review_architecture_gate(contract, artifacts),
        review_disposition_gate(contract, artifacts, disposition_approved),
        pr_review_release_readiness_gate(
            contract,
            artifacts,
            disposition_approved,
            context.denied_invocations,
            context.evidence_complete,
        ),
    ]
}

fn exploration_gate(artifacts: &[(String, String)]) -> GateEvaluation {
    let has_problem = artifacts.iter().any(|(file_name, _)| file_name == "problem-statement.md");
    let blockers = if has_problem {
        Vec::new()
    } else {
        vec!["problem statement artifact is required before exploration can pass".to_string()]
    };

    GateEvaluation {
        gate: GateKind::Exploration,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

fn risk_gate(owner: &str) -> GateEvaluation {
    let blockers = if owner.trim().is_empty() {
        vec!["human ownership is required before risk classification can pass".to_string()]
    } else {
        Vec::new()
    };

    GateEvaluation {
        gate: GateKind::Risk,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

fn requirements_release_readiness_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    denied_invocations: &[DeniedInvocation],
    evidence_complete: bool,
) -> GateEvaluation {
    let mut blockers = validate_release_bundle(contract, artifacts);

    if !evidence_complete {
        blockers.push(
            "requirements release readiness needs explicit generation and validation evidence"
                .to_string(),
        );
    }

    if denied_invocations
        .iter()
        .any(|denied| denied.rationale.contains("disabled for runtime execution"))
    {
        blockers.push(
            "runtime-disabled invocation attempts must be resolved before release readiness can pass"
                .to_string(),
        );
    }

    GateEvaluation {
        gate: GateKind::ReleaseReadiness,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

fn brownfield_risk_gate(
    owner: &str,
    risk: RiskClass,
    zone: UsageZone,
    approvals: &[ApprovalRecord],
) -> GateEvaluation {
    let owner_blockers = if owner.trim().is_empty() {
        vec!["human ownership is required before risk classification can pass".to_string()]
    } else {
        Vec::new()
    };

    if !owner_blockers.is_empty() {
        return GateEvaluation {
            gate: GateKind::Risk,
            status: GateStatus::Blocked,
            blockers: owner_blockers,
            evaluated_at: OffsetDateTime::now_utc(),
        };
    }

    let approval_required =
        matches!(risk, RiskClass::SystemicImpact) || matches!(zone, UsageZone::Red);
    let gate_approved = approvals
        .iter()
        .any(|approval| approval.matches_gate(GateKind::Risk) && approval.is_approved());
    let invocation_approved = approvals
        .iter()
        .any(|approval| approval.invocation_request_id.is_some() && approval.is_approved());

    let (status, blockers) = if approval_required && !(gate_approved || invocation_approved) {
        (
            GateStatus::NeedsApproval,
            vec![
                "systemic-impact or red-zone brownfield work requires explicit approval before it can proceed"
                    .to_string(),
            ],
        )
    } else if gate_approved || invocation_approved {
        (GateStatus::Overridden, Vec::new())
    } else {
        (GateStatus::Passed, Vec::new())
    };

    GateEvaluation {
        gate: GateKind::Risk,
        status,
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

fn brownfield_release_readiness_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    validation_independence_satisfied: bool,
    evidence_complete: bool,
) -> GateEvaluation {
    let mut blockers = validate_release_bundle(contract, artifacts);

    if !evidence_complete {
        blockers.push(
            "brownfield readiness needs explicit generation and validation evidence".to_string(),
        );
    }

    if !validation_independence_satisfied {
        blockers.push(
            "brownfield readiness requires an independently recorded validation path".to_string(),
        );
    }

    GateEvaluation {
        gate: GateKind::ReleaseReadiness,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

fn approval_aware_risk_gate(
    owner: &str,
    risk: RiskClass,
    zone: UsageZone,
    approvals: &[ApprovalRecord],
    approval_message: &str,
) -> GateEvaluation {
    let owner_blockers = if owner.trim().is_empty() {
        vec!["human ownership is required before risk classification can pass".to_string()]
    } else {
        Vec::new()
    };

    if !owner_blockers.is_empty() {
        return GateEvaluation {
            gate: GateKind::Risk,
            status: GateStatus::Blocked,
            blockers: owner_blockers,
            evaluated_at: OffsetDateTime::now_utc(),
        };
    }

    let approval_required =
        matches!(risk, RiskClass::SystemicImpact) || matches!(zone, UsageZone::Red);
    let approved = approvals
        .iter()
        .any(|approval| approval.matches_gate(GateKind::Risk) && approval.is_approved());

    let (status, blockers) = if approval_required && !approved {
        (GateStatus::NeedsApproval, vec![approval_message.to_string()])
    } else if approved {
        (GateStatus::Overridden, Vec::new())
    } else {
        (GateStatus::Passed, Vec::new())
    };

    GateEvaluation {
        gate: GateKind::Risk,
        status,
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

fn pr_review_architecture_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
) -> GateEvaluation {
    let mut blockers = contract
        .artifact_requirements
        .iter()
        .filter(|requirement| {
            matches!(requirement.file_name.as_str(), "boundary-check.md" | "contract-drift.md")
        })
        .flat_map(|requirement| {
            artifacts
                .iter()
                .find(|(file_name, _)| file_name == &requirement.file_name)
                .map(|(_, contents)| {
                    crate::artifacts::contract::validate_artifact(requirement, contents)
                })
                .unwrap_or_else(|| {
                    vec![format!("missing required artifact `{}`", requirement.file_name)]
                })
        })
        .collect::<Vec<_>>();

    for (file_name, contents) in artifacts {
        if file_name == "boundary-check.md" && contents.contains("Status: missing-boundary-review")
        {
            blockers.push(
                "boundary review is incomplete and cannot satisfy the architecture gate"
                    .to_string(),
            );
        }
        if file_name == "contract-drift.md"
            && contents.contains("Status: unsupported-contract-drift")
        {
            blockers.push("unsupported contract drift blocks the architecture gate".to_string());
        }
    }

    GateEvaluation {
        gate: GateKind::Architecture,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

fn review_disposition_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    disposition_approved: bool,
) -> GateEvaluation {
    let mut blockers = contract
        .artifact_requirements
        .iter()
        .filter(|requirement| requirement.file_name == "review-summary.md")
        .flat_map(|requirement| {
            artifacts
                .iter()
                .find(|(file_name, _)| file_name == &requirement.file_name)
                .map(|(_, contents)| {
                    crate::artifacts::contract::validate_artifact(requirement, contents)
                })
                .unwrap_or_else(|| {
                    vec!["missing required artifact `review-summary.md`".to_string()]
                })
        })
        .collect::<Vec<_>>();

    let summary = artifacts
        .iter()
        .find(|(file_name, _)| file_name == "review-summary.md")
        .map(|(_, contents)| contents.as_str())
        .unwrap_or_default();

    let status = if blockers.is_empty() && summary.contains("Status: awaiting-disposition") {
        if disposition_approved {
            GateStatus::Overridden
        } else {
            blockers.push(
                "must-fix review findings require explicit disposition before readiness can pass"
                    .to_string(),
            );
            GateStatus::NeedsApproval
        }
    } else {
        gate_status_from_blockers(&blockers)
    };

    GateEvaluation {
        gate: GateKind::ReviewDisposition,
        status,
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

fn pr_review_release_readiness_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    disposition_approved: bool,
    denied_invocations: &[DeniedInvocation],
    evidence_complete: bool,
) -> GateEvaluation {
    let mut blockers = validate_release_bundle(contract, artifacts);

    if !evidence_complete {
        blockers.push(
            "pr-review readiness requires persisted diff inspection and critique evidence"
                .to_string(),
        );
    }

    let summary = artifacts
        .iter()
        .find(|(file_name, _)| file_name == "review-summary.md")
        .map(|(_, contents)| contents.as_str())
        .unwrap_or_default();

    if summary.contains("Status: awaiting-disposition") && !disposition_approved {
        blockers.push(
            "review-summary.md still records unresolved must-fix findings without disposition"
                .to_string(),
        );
    }

    if denied_invocations
        .iter()
        .any(|denied| denied.rationale.contains("disabled for runtime execution"))
    {
        blockers.push(
            "runtime-disabled pr-review invocation attempts must be resolved before readiness can pass"
                .to_string(),
        );
    }

    GateEvaluation {
        gate: GateKind::ReleaseReadiness,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

fn gate_status_from_blockers(blockers: &[String]) -> GateStatus {
    if blockers.is_empty() { GateStatus::Passed } else { GateStatus::Blocked }
}

fn named_artifact_gate(
    gate: GateKind,
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    names: &[&str],
    fallback_blocker: &str,
) -> GateEvaluation {
    let mut blockers = contract
        .artifact_requirements
        .iter()
        .filter(|requirement| names.iter().any(|name| requirement.file_name == *name))
        .flat_map(|requirement| {
            artifacts
                .iter()
                .find(|(file_name, _)| file_name == &requirement.file_name)
                .map(|(_, contents)| {
                    crate::artifacts::contract::validate_artifact(requirement, contents)
                })
                .unwrap_or_else(|| {
                    vec![format!("missing required artifact `{}`", requirement.file_name)]
                })
        })
        .collect::<Vec<_>>();

    if blockers.is_empty()
        && names.iter().any(|name| !artifacts.iter().any(|(file_name, _)| file_name == name))
    {
        blockers.push(fallback_blocker.to_string());
    }

    GateEvaluation {
        gate,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

#[cfg(test)]
mod tests {
    use time::OffsetDateTime;

    use super::{
        BrownfieldGateContext, PrReviewGateContext, evaluate_brownfield_gates,
        evaluate_pr_review_gates, evaluate_requirements_gates,
    };
    use crate::artifacts::contract::contract_for_mode;
    use crate::domain::approval::{ApprovalDecision, ApprovalRecord};
    use crate::domain::artifact::{ArtifactContract, ArtifactRequirement};
    use crate::domain::execution::DeniedInvocation;
    use crate::domain::gate::{GateEvaluation, GateKind, GateStatus};
    use crate::domain::mode::Mode;
    use crate::domain::policy::{RiskClass, UsageZone};

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

        let evaluations =
            evaluate_requirements_gates(&contract, &artifacts, "Owner", &denied, false);
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
    fn requirements_exploration_and_risk_block_without_problem_statement_or_owner() {
        let contract = contract_for_mode(Mode::Requirements);
        let mut artifacts = valid_artifacts(&contract);
        artifacts.retain(|(file_name, _)| file_name != "problem-statement.md");

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
        assert!(
            risk.blockers.iter().any(|blocker| blocker.contains("human ownership is required"))
        );
    }

    #[test]
    fn brownfield_risk_gate_requires_or_records_approval_for_systemic_red_runs() {
        let contract = contract_for_mode(Mode::BrownfieldChange);
        let artifacts = valid_artifacts(&contract);

        let gated = evaluate_brownfield_gates(
            &contract,
            &artifacts,
            BrownfieldGateContext {
                owner: "Owner",
                risk: RiskClass::SystemicImpact,
                zone: UsageZone::Red,
                approvals: &[],
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
        let overridden = evaluate_brownfield_gates(
            &contract,
            &artifacts,
            BrownfieldGateContext {
                owner: "Owner",
                risk: RiskClass::SystemicImpact,
                zone: UsageZone::Red,
                approvals: &approvals,
                validation_independence_satisfied: true,
                evidence_complete: true,
            },
        );

        assert_eq!(gate(&overridden, GateKind::Risk).status, GateStatus::Overridden);
    }

    #[test]
    fn brownfield_release_readiness_blocks_without_evidence_or_independent_validation() {
        let contract = contract_for_mode(Mode::BrownfieldChange);
        let artifacts = valid_artifacts(&contract);

        let evaluations = evaluate_brownfield_gates(
            &contract,
            &artifacts,
            BrownfieldGateContext {
                owner: "Owner",
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
    fn pr_review_architecture_gate_blocks_boundary_and_contract_drift_statuses() {
        let contract = contract_for_mode(Mode::PrReview);
        let mut artifacts = valid_artifacts(&contract);

        let boundary_check = artifacts
            .iter_mut()
            .find(|(file_name, _)| file_name == "boundary-check.md")
            .expect("boundary-check artifact present");
        boundary_check.1.push_str("\n\nStatus: missing-boundary-review");

        let contract_drift = artifacts
            .iter_mut()
            .find(|(file_name, _)| file_name == "contract-drift.md")
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
            architecture
                .blockers
                .iter()
                .any(|blocker| blocker.contains("unsupported contract drift"))
        );
    }

    #[test]
    fn pr_review_disposition_gate_respects_explicit_review_disposition_approval() {
        let contract = contract_for_mode(Mode::PrReview);
        let mut artifacts = valid_artifacts(&contract);
        let review_summary = artifacts
            .iter_mut()
            .find(|(file_name, _)| file_name == "review-summary.md")
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
            .find(|(file_name, _)| file_name == "review-summary.md")
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
        assert!(release
            .blockers
            .iter()
            .any(|blocker| blocker.contains("unresolved must-fix findings without disposition")));
        assert!(
            release
                .blockers
                .iter()
                .any(|blocker| blocker.contains("runtime-disabled pr-review invocation attempts"))
        );
    }
}
