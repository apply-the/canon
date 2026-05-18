use time::OffsetDateTime;

use crate::artifacts::contract::validate_release_bundle;
use crate::domain::approval::ApprovalRecord;
use crate::domain::artifact::{
    ArtifactContract, ArtifactRequirement, REVIEW_SUMMARY_ARTIFACT_SLUG, artifact_slug,
};
use crate::domain::execution::DeniedInvocation;
use crate::domain::gate::{GateEvaluation, GateKind, GateStatus};
use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::run::{
    ClosureAssessment, ClosureDecompositionScope, ClosureFindingSeverity, ClosureStatus,
    SystemContext,
};

pub(super) fn operational_capture_gate(
    mut evaluation: GateEvaluation,
    artifacts: &[(String, String)],
    names: &[&str],
    gate: GateKind,
) -> GateEvaluation {
    evaluation.blockers.extend(
        names
            .iter()
            .filter_map(|name| {
                artifacts.iter().find(|(file_name, _)| artifact_slug(file_name) == *name).and_then(
                    |(file_name, contents)| {
                        if contents.contains("NOT CAPTURED") {
                            Some(format!(
                                "{file_name} still contains uncaptured operational evidence and blocks {} review",
                                gate.as_str()
                            ))
                        } else {
                            None
                        }
                    },
                )
            })
            .collect::<Vec<_>>(),
    );
    evaluation.status = gate_status_from_blockers(&evaluation.blockers);
    evaluation
}

fn requirement_blockers(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    predicate: impl Fn(&ArtifactRequirement) -> bool,
) -> Vec<String> {
    contract
        .artifact_requirements
        .iter()
        .filter(|requirement| predicate(requirement))
        .flat_map(|requirement| validate_requirement_blockers(requirement, artifacts))
        .collect::<Vec<_>>()
}

fn validate_requirement_blockers(
    requirement: &ArtifactRequirement,
    artifacts: &[(String, String)],
) -> Vec<String> {
    artifacts
        .iter()
        .find(|(file_name, _)| file_name == &requirement.file_name)
        .map(|(_, contents)| crate::artifacts::contract::validate_artifact(requirement, contents))
        .unwrap_or_else(|| {
            if requirement.required {
                vec![format!("missing required artifact `{}`", requirement.file_name)]
            } else {
                Vec::new()
            }
        })
}

fn existing_system_requirement_gate(
    gate: GateKind,
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    predicate: impl Fn(&ArtifactRequirement) -> bool,
    system_context: Option<SystemContext>,
    missing_context_message: &str,
) -> GateEvaluation {
    let mut blockers = requirement_blockers(contract, artifacts, predicate);

    if !matches!(system_context, Some(SystemContext::Existing)) {
        blockers.push(missing_context_message.to_string());
    }

    GateEvaluation {
        gate,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

fn blocked_risk_gate_without_owner(owner: &str) -> Option<GateEvaluation> {
    if owner.trim().is_empty() {
        Some(GateEvaluation {
            gate: GateKind::Risk,
            status: GateStatus::Blocked,
            blockers: vec![
                "human ownership is required before risk classification can pass".to_string(),
            ],
            evaluated_at: OffsetDateTime::now_utc(),
        })
    } else {
        None
    }
}

pub(super) fn change_preservation_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    system_context: Option<SystemContext>,
) -> GateEvaluation {
    existing_system_requirement_gate(
        GateKind::ChangePreservation,
        contract,
        artifacts,
        |requirement| matches!(requirement.slug(), "legacy-invariants.md" | "change-surface.md"),
        system_context,
        "change preservation requires `system_context = existing` so gating stays bound to an existing system",
    )
}

pub(super) fn exploration_gate(artifacts: &[(String, String)]) -> GateEvaluation {
    let has_problem =
        artifacts.iter().any(|(file_name, _)| artifact_slug(file_name) == "problem-statement.md");
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

pub(super) fn risk_gate(owner: &str) -> GateEvaluation {
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

pub(super) fn requirements_release_readiness_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    denied_invocations: &[DeniedInvocation],
    evidence_complete: bool,
) -> GateEvaluation {
    let mut blockers = validate_release_bundle(contract, artifacts);

    if artifacts.iter().any(|(file_name, contents)| {
        artifact_slug(file_name) == "problem-statement.md" && contents.contains("## Input:")
    }) {
        blockers.push(
            "requirements problem statement must synthesize the bounded need instead of replaying raw input labels"
                .to_string(),
        );
    }

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

pub(super) fn analysis_release_readiness_gate(
    gate: GateKind,
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    validation_independence_satisfied: bool,
    evidence_complete: bool,
    missing_evidence_message: &str,
) -> GateEvaluation {
    let mut blockers = validate_release_bundle(contract, artifacts);

    if !evidence_complete {
        blockers.push(missing_evidence_message.to_string());
    }

    if !validation_independence_satisfied {
        blockers.push(
            "analysis readiness requires an independently recorded repository validation path"
                .to_string(),
        );
    }

    GateEvaluation {
        gate,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

pub(super) fn implementation_readiness_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    system_context: Option<SystemContext>,
) -> GateEvaluation {
    existing_system_requirement_gate(
        GateKind::ImplementationReadiness,
        contract,
        artifacts,
        |requirement| {
            matches!(
                requirement.slug(),
                "task-mapping.md"
                    | "mutation-bounds.md"
                    | "validation-hooks.md"
                    | "rollback-notes.md"
            )
        },
        system_context,
        "implementation planning requires `system_context = existing` so mutation bounds stay attached to an existing system",
    )
}

pub(super) fn refactor_preservation_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    system_context: Option<SystemContext>,
) -> GateEvaluation {
    existing_system_requirement_gate(
        GateKind::ChangePreservation,
        contract,
        artifacts,
        |requirement| {
            matches!(
                requirement.slug(),
                "preserved-behavior.md" | "refactor-scope.md" | "no-feature-addition.md"
            )
        },
        system_context,
        "refactor preservation requires `system_context = existing` so structural work stays attached to an existing system",
    )
}

pub(super) fn change_risk_gate(
    owner: &str,
    risk: RiskClass,
    zone: UsageZone,
    approvals: &[ApprovalRecord],
) -> GateEvaluation {
    if let Some(evaluation) = blocked_risk_gate_without_owner(owner) {
        return evaluation;
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
                "systemic-impact or red-zone change work requires explicit approval before it can proceed"
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

pub(super) fn implementation_execution_gate(
    owner: &str,
    approvals: &[ApprovalRecord],
) -> GateEvaluation {
    execution_gate(
        owner,
        approvals,
        "implementation execution mutates the workspace and requires explicit approval before it can proceed",
    )
}

pub(super) fn backlog_architecture_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    system_context: Option<SystemContext>,
    closure_assessment: &ClosureAssessment,
) -> GateEvaluation {
    let mut blockers = if matches!(
        closure_assessment.decomposition_scope,
        ClosureDecompositionScope::RiskOnlyPacket
    ) {
        Vec::new()
    } else {
        named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["epic-tree.md", "dependency-map.md", "delivery-slices.md"],
            "backlog architecture review requires explicit epics, dependencies, and delivery slices",
        )
        .blockers
    };

    if !matches!(system_context, Some(SystemContext::Existing)) {
        blockers.push(
            "backlog planning requires `system_context = existing` so decomposition stays attached to an existing system"
                .to_string(),
        );
    }

    if matches!(closure_assessment.status, ClosureStatus::Blocked) {
        blockers.extend(
            closure_assessment
                .findings
                .iter()
                .filter(|finding| matches!(finding.severity, ClosureFindingSeverity::Blocking))
                .map(|finding| {
                    format!(
                        "closure finding `{}` on {} requires follow-up before architecture review can pass",
                        finding.category, finding.affected_scope
                    )
                }),
        );
    }

    GateEvaluation {
        gate: GateKind::Architecture,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

pub(super) fn refactor_execution_gate(owner: &str, approvals: &[ApprovalRecord]) -> GateEvaluation {
    execution_gate(
        owner,
        approvals,
        "refactor execution mutates the workspace and requires explicit approval before it can proceed",
    )
}

pub(super) fn execution_gate(
    owner: &str,
    approvals: &[ApprovalRecord],
    blocker_message: &str,
) -> GateEvaluation {
    if owner.trim().is_empty() {
        return GateEvaluation {
            gate: GateKind::Execution,
            status: GateStatus::Blocked,
            blockers: vec![
                "human ownership is required before execution approval can be requested"
                    .to_string(),
            ],
            evaluated_at: OffsetDateTime::now_utc(),
        };
    }

    let gate_approved = approvals
        .iter()
        .any(|approval| approval.matches_gate(GateKind::Execution) && approval.is_approved());

    let (status, blockers) = if gate_approved {
        (GateStatus::Overridden, Vec::new())
    } else {
        (GateStatus::NeedsApproval, vec![blocker_message.to_string()])
    };

    GateEvaluation {
        gate: GateKind::Execution,
        status,
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

pub(super) fn change_release_readiness_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    validation_independence_satisfied: bool,
    evidence_complete: bool,
) -> GateEvaluation {
    let mut blockers = validate_release_bundle(contract, artifacts);

    if !evidence_complete {
        blockers
            .push("change readiness needs explicit generation and validation evidence".to_string());
    }

    if !validation_independence_satisfied {
        blockers.push(
            "change readiness requires an independently recorded validation path".to_string(),
        );
    }

    GateEvaluation {
        gate: GateKind::ReleaseReadiness,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

pub(super) fn implementation_release_readiness_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    validation_independence_satisfied: bool,
    evidence_complete: bool,
) -> GateEvaluation {
    let mut blockers = validate_release_bundle(contract, artifacts);

    if !evidence_complete {
        blockers.push(
            "implementation readiness needs explicit generation and validation evidence"
                .to_string(),
        );
    }

    if !validation_independence_satisfied {
        blockers.push(
            "implementation readiness requires an independently recorded validation path"
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

pub(super) fn refactor_release_readiness_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    validation_independence_satisfied: bool,
    evidence_complete: bool,
) -> GateEvaluation {
    let mut blockers = validate_release_bundle(contract, artifacts);

    if !evidence_complete {
        blockers.push(
            "refactor readiness needs explicit generation and validation evidence".to_string(),
        );
    }

    if !validation_independence_satisfied {
        blockers.push(
            "refactor readiness requires an independently recorded validation path".to_string(),
        );
    }

    GateEvaluation {
        gate: GateKind::ReleaseReadiness,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

pub(super) fn approval_aware_risk_gate(
    owner: &str,
    risk: RiskClass,
    zone: UsageZone,
    approvals: &[ApprovalRecord],
    approval_message: &str,
) -> GateEvaluation {
    if let Some(evaluation) = blocked_risk_gate_without_owner(owner) {
        return evaluation;
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

pub(super) fn pr_review_architecture_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
) -> GateEvaluation {
    let mut blockers = requirement_blockers(contract, artifacts, |requirement| {
        matches!(requirement.slug(), "boundary-check.md" | "contract-drift.md")
    });

    for (file_name, contents) in artifacts {
        if artifact_slug(file_name) == "boundary-check.md"
            && contents.contains("Status: missing-boundary-review")
        {
            blockers.push(
                "boundary review is incomplete and cannot satisfy the architecture gate"
                    .to_string(),
            );
        }
        if artifact_slug(file_name) == "contract-drift.md"
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

pub(super) fn review_disposition_gate_for_file(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    file_name: &str,
    disposition_approved: bool,
) -> GateEvaluation {
    let mut blockers = contract
        .artifact_requirements
        .iter()
        .filter(|requirement| artifact_slug(&requirement.file_name) == file_name)
        .flat_map(|requirement| {
            artifacts
                .iter()
                .find(|(artifact_file_name, _)| artifact_file_name == &requirement.file_name)
                .map(|(_, contents)| {
                    crate::artifacts::contract::validate_artifact(requirement, contents)
                })
                .unwrap_or_else(|| {
                    if requirement.required {
                        vec![format!("missing required artifact `{file_name}`")]
                    } else {
                        Vec::new()
                    }
                })
        })
        .collect::<Vec<_>>();

    let summary = artifacts
        .iter()
        .find(|(artifact_file_name, _)| artifact_slug(artifact_file_name) == file_name)
        .map(|(_, contents)| contents.as_str())
        .unwrap_or_default();

    let status = if blockers.is_empty() && summary.contains("Status: awaiting-disposition") {
        if disposition_approved {
            GateStatus::Overridden
        } else {
            blockers.push(
                "review findings require explicit disposition before readiness can pass"
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

pub(super) fn review_release_readiness_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    disposition_approved: bool,
    evidence_complete: bool,
) -> GateEvaluation {
    let mut blockers = validate_release_bundle(contract, artifacts);

    if !evidence_complete {
        blockers.push(
            "review readiness requires persisted context, critique, and validation evidence"
                .to_string(),
        );
    }

    let disposition = artifacts
        .iter()
        .find(|(file_name, _)| artifact_slug(file_name) == "review-disposition.md")
        .map(|(_, contents)| contents.as_str())
        .unwrap_or_default();
    let missing_evidence = artifacts
        .iter()
        .find(|(file_name, _)| artifact_slug(file_name) == "missing-evidence.md")
        .map(|(_, contents)| contents.as_str())
        .unwrap_or_default();

    if disposition.contains("Status: awaiting-disposition") && !disposition_approved {
        blockers.push(
            "review-disposition.md still records unresolved disposition work without approval"
                .to_string(),
        );
    }

    if missing_evidence.contains("Status: missing-evidence-open") {
        blockers.push(
            "review packet still records open evidence gaps that block release readiness"
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

pub(super) fn pr_review_release_readiness_gate(
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
        .find(|(file_name, _)| artifact_slug(file_name) == REVIEW_SUMMARY_ARTIFACT_SLUG)
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

pub(super) fn verification_release_readiness_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    validation_independence_satisfied: bool,
    evidence_complete: bool,
) -> GateEvaluation {
    let mut blockers = validate_release_bundle(contract, artifacts);

    if !evidence_complete {
        blockers.push(
            "verification readiness requires persisted challenge and validation evidence"
                .to_string(),
        );
    }

    if !validation_independence_satisfied {
        blockers.push(
            "verification readiness requires an independently recorded validation path".to_string(),
        );
    }

    let unresolved_findings = artifacts
        .iter()
        .find(|(file_name, _)| artifact_slug(file_name) == "unresolved-findings.md")
        .map(|(_, contents)| contents.as_str())
        .unwrap_or_default();
    let verdict = artifacts
        .iter()
        .find(|(file_name, _)| artifact_slug(file_name) == "verification-report.md")
        .map(|(_, contents)| contents.as_str())
        .unwrap_or_default();

    if unresolved_findings.contains("Status: unresolved-findings-open") {
        blockers.push(
            "verification packet still records unresolved findings that block release readiness"
                .to_string(),
        );
    }

    if verdict.contains("Status: unsupported") {
        blockers.push("verification-report.md still records an unsupported verdict".to_string());
    }

    GateEvaluation {
        gate: GateKind::ReleaseReadiness,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

pub(super) fn gate_status_from_blockers(blockers: &[String]) -> GateStatus {
    if blockers.is_empty() { GateStatus::Passed } else { GateStatus::Blocked }
}

pub(super) fn named_artifact_gate(
    gate: GateKind,
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    names: &[&str],
    fallback_blocker: &str,
) -> GateEvaluation {
    let mut blockers = contract
        .artifact_requirements
        .iter()
        .filter(|requirement| {
            names.iter().any(|name| artifact_slug(&requirement.file_name) == *name)
        })
        .flat_map(|requirement| {
            artifacts
                .iter()
                .find(|(file_name, _)| file_name == &requirement.file_name)
                .map(|(_, contents)| {
                    let mut blockers =
                        crate::artifacts::contract::validate_artifact(requirement, contents);
                    if contents.contains("Insufficient evidence:") {
                        blockers.push(format!(
                            "{} lacks sufficient evidence for `{}` gate review",
                            requirement.file_name,
                            gate.as_str()
                        ));
                    }
                    blockers
                })
                .unwrap_or_else(|| {
                    if requirement.required {
                        vec![format!("missing required artifact `{}`", requirement.file_name)]
                    } else {
                        Vec::new()
                    }
                })
        })
        .collect::<Vec<_>>();

    if blockers.is_empty()
        && contract
            .artifact_requirements
            .iter()
            .filter(|requirement| requirement.required)
            .filter(|requirement| {
                names.iter().any(|name| artifact_slug(&requirement.file_name) == *name)
            })
            .any(|requirement| {
                !artifacts.iter().any(|(file_name, _)| file_name == &requirement.file_name)
            })
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
