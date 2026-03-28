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
