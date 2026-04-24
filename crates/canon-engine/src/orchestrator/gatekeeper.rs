use time::OffsetDateTime;

use crate::artifacts::contract::validate_release_bundle;
use crate::domain::approval::ApprovalRecord;
use crate::domain::artifact::ArtifactContract;
use crate::domain::execution::DeniedInvocation;
use crate::domain::gate::{GateEvaluation, GateKind, GateStatus};
use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::run::{ClosureAssessment, ClosureDecompositionScope, SystemContext};

pub struct DiscoveryGateContext<'a> {
    pub owner: &'a str,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub approvals: &'a [ApprovalRecord],
    pub validation_independence_satisfied: bool,
    pub evidence_complete: bool,
}

pub struct SystemShapingGateContext<'a> {
    pub owner: &'a str,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub approvals: &'a [ApprovalRecord],
    pub evidence_complete: bool,
}

pub struct ArchitectureGateContext<'a> {
    pub owner: &'a str,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub approvals: &'a [ApprovalRecord],
    pub evidence_complete: bool,
}

pub struct BacklogGateContext<'a> {
    pub owner: &'a str,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub system_context: Option<SystemContext>,
    pub approvals: &'a [ApprovalRecord],
    pub validation_independence_satisfied: bool,
    pub evidence_complete: bool,
    pub closure_assessment: &'a ClosureAssessment,
}

pub struct ChangeGateContext<'a> {
    pub owner: &'a str,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub approvals: &'a [ApprovalRecord],
    pub system_context: Option<SystemContext>,
    pub validation_independence_satisfied: bool,
    pub evidence_complete: bool,
}

pub struct ImplementationGateContext<'a> {
    pub owner: &'a str,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub approvals: &'a [ApprovalRecord],
    pub system_context: Option<SystemContext>,
    pub validation_independence_satisfied: bool,
    pub evidence_complete: bool,
}

pub struct RefactorGateContext<'a> {
    pub owner: &'a str,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub approvals: &'a [ApprovalRecord],
    pub system_context: Option<SystemContext>,
    pub validation_independence_satisfied: bool,
    pub evidence_complete: bool,
}

pub struct ReviewGateContext<'a> {
    pub owner: &'a str,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub approvals: &'a [ApprovalRecord],
    pub evidence_complete: bool,
}

pub struct VerificationGateContext<'a> {
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

pub fn evaluate_discovery_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: DiscoveryGateContext<'_>,
) -> Vec<GateEvaluation> {
    vec![
        named_artifact_gate(
            GateKind::Exploration,
            contract,
            artifacts,
            &["problem-map.md", "context-boundary.md"],
            "discovery requires a bounded problem domain and explicit context boundary",
        ),
        approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone discovery work requires explicit approval before it can proceed",
        ),
        analysis_release_readiness_gate(
            GateKind::ReleaseReadiness,
            contract,
            artifacts,
            context.validation_independence_satisfied,
            context.evidence_complete,
            "discovery readiness requires persisted context, critique, and repository validation evidence",
        ),
    ]
}

pub fn evaluate_system_shaping_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: SystemShapingGateContext<'_>,
) -> Vec<GateEvaluation> {
    vec![
        named_artifact_gate(
            GateKind::Exploration,
            contract,
            artifacts,
            &["system-shape.md", "capability-map.md"],
            "system-shaping requires a bounded system shape and capability map",
        ),
        named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["system-shape.md", "architecture-outline.md", "capability-map.md"],
            "system-shaping architecture review requires bounded structure, capabilities, and rationale",
        ),
        approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone system-shaping work requires explicit approval before it can proceed",
        ),
        analysis_release_readiness_gate(
            GateKind::ReleaseReadiness,
            contract,
            artifacts,
            true,
            context.evidence_complete,
            "system-shaping readiness requires persisted context, generation, and critique evidence",
        ),
    ]
}

pub fn evaluate_architecture_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: ArchitectureGateContext<'_>,
) -> Vec<GateEvaluation> {
    vec![
        named_artifact_gate(
            GateKind::Exploration,
            contract,
            artifacts,
            &["boundary-map.md"],
            "architecture exploration requires an explicit boundary map",
        ),
        named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["architecture-decisions.md", "invariants.md", "tradeoff-matrix.md"],
            "architecture review requires decisions, invariants, and tradeoff scoring",
        ),
        approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone architecture work requires explicit approval before it can proceed",
        ),
        analysis_release_readiness_gate(
            GateKind::ReleaseReadiness,
            contract,
            artifacts,
            true,
            context.evidence_complete,
            "architecture readiness requires persisted context, generation, and critique evidence",
        ),
    ]
}

pub fn evaluate_change_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: ChangeGateContext<'_>,
) -> Vec<GateEvaluation> {
    vec![
        named_artifact_gate(
            GateKind::Exploration,
            contract,
            artifacts,
            &["system-slice.md"],
            "change exploration requires a bounded system slice",
        ),
        change_preservation_gate(contract, artifacts, context.system_context),
        named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["implementation-plan.md", "decision-record.md"],
            "change architecture review requires an implementation plan and decision record",
        ),
        change_risk_gate(context.owner, context.risk, context.zone, context.approvals),
        change_release_readiness_gate(
            contract,
            artifacts,
            context.validation_independence_satisfied,
            context.evidence_complete,
        ),
    ]
}

pub fn evaluate_backlog_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: BacklogGateContext<'_>,
) -> Vec<GateEvaluation> {
    let effective_contract = crate::artifacts::contract::backlog_contract_for_closure(
        contract,
        context.closure_assessment,
    );
    let exploration_names: &[&str] = if matches!(
        context.closure_assessment.decomposition_scope,
        ClosureDecompositionScope::RiskOnlyPacket
    ) {
        &["backlog-overview.md"]
    } else {
        &["backlog-overview.md", "capability-to-epic-map.md"]
    };

    vec![
        named_artifact_gate(
            GateKind::Exploration,
            &effective_contract,
            artifacts,
            exploration_names,
            "backlog requires an overview and capability mapping before decomposition can proceed",
        ),
        backlog_architecture_gate(
            &effective_contract,
            artifacts,
            context.system_context,
            context.closure_assessment,
        ),
        approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone backlog work requires explicit approval before it can proceed",
        ),
        analysis_release_readiness_gate(
            GateKind::ReleaseReadiness,
            &effective_contract,
            artifacts,
            context.validation_independence_satisfied,
            context.evidence_complete,
            "backlog readiness requires persisted context, critique, and repository validation evidence",
        ),
    ]
}

pub fn evaluate_implementation_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: ImplementationGateContext<'_>,
) -> Vec<GateEvaluation> {
    let readiness = implementation_readiness_gate(contract, artifacts, context.system_context);
    let release = implementation_release_readiness_gate(
        contract,
        artifacts,
        context.validation_independence_satisfied,
        context.evidence_complete,
    );
    let mut gates = vec![readiness];
    if !gates.iter().any(|gate| matches!(gate.status, GateStatus::Blocked))
        && !matches!(release.status, GateStatus::Blocked)
    {
        gates.push(implementation_execution_gate(context.owner, context.approvals));
    }
    gates.push(release);
    gates
}

pub fn evaluate_refactor_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: RefactorGateContext<'_>,
) -> Vec<GateEvaluation> {
    let preservation = refactor_preservation_gate(contract, artifacts, context.system_context);
    let architecture = named_artifact_gate(
        GateKind::Architecture,
        contract,
        artifacts,
        &["structural-rationale.md", "contract-drift-check.md"],
        "refactor architecture review requires structural rationale and contract drift review",
    );
    let release = refactor_release_readiness_gate(
        contract,
        artifacts,
        context.validation_independence_satisfied,
        context.evidence_complete,
    );
    let mut gates = vec![preservation, architecture];
    if !gates.iter().any(|gate| matches!(gate.status, GateStatus::Blocked))
        && !matches!(release.status, GateStatus::Blocked)
    {
        gates.push(refactor_execution_gate(context.owner, context.approvals));
    }
    gates.push(release);
    gates
}

fn change_preservation_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    system_context: Option<SystemContext>,
) -> GateEvaluation {
    let mut blockers = contract
        .artifact_requirements
        .iter()
        .filter(|requirement| {
            matches!(requirement.file_name.as_str(), "legacy-invariants.md" | "change-surface.md")
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

    // Change keeps the preserved-behavior gate, while system_context keeps the target-system fact explicit.
    if !matches!(system_context, Some(SystemContext::Existing)) {
        blockers.push(
            "change preservation requires `system_context = existing` so gating stays bound to an existing system"
                .to_string(),
        );
    }

    GateEvaluation {
        gate: GateKind::ChangePreservation,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

pub fn evaluate_review_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: ReviewGateContext<'_>,
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
        named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["boundary-assessment.md", "decision-impact.md"],
            "review requires explicit boundary assessment and decision impact before disposition",
        ),
        review_disposition_gate_for_file(
            contract,
            artifacts,
            "review-disposition.md",
            disposition_approved,
        ),
        review_release_readiness_gate(
            contract,
            artifacts,
            disposition_approved,
            context.evidence_complete,
        ),
    ]
}

pub fn evaluate_verification_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: VerificationGateContext<'_>,
) -> Vec<GateEvaluation> {
    vec![
        approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone verification work requires explicit approval before it can proceed",
        ),
        verification_release_readiness_gate(
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
        review_disposition_gate_for_file(
            contract,
            artifacts,
            "review-summary.md",
            disposition_approved,
        ),
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

    if artifacts.iter().any(|(file_name, contents)| {
        file_name == "problem-statement.md" && contents.contains("## Input:")
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

fn analysis_release_readiness_gate(
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

fn implementation_readiness_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    system_context: Option<SystemContext>,
) -> GateEvaluation {
    let mut blockers = contract
        .artifact_requirements
        .iter()
        .filter(|requirement| {
            matches!(
                requirement.file_name.as_str(),
                "task-mapping.md"
                    | "mutation-bounds.md"
                    | "validation-hooks.md"
                    | "rollback-notes.md"
            )
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

    if !matches!(system_context, Some(SystemContext::Existing)) {
        blockers.push(
            "implementation planning requires `system_context = existing` so mutation bounds stay attached to an existing system"
                .to_string(),
        );
    }

    GateEvaluation {
        gate: GateKind::ImplementationReadiness,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

fn refactor_preservation_gate(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    system_context: Option<SystemContext>,
) -> GateEvaluation {
    let mut blockers = contract
        .artifact_requirements
        .iter()
        .filter(|requirement| {
            matches!(
                requirement.file_name.as_str(),
                "preserved-behavior.md" | "refactor-scope.md" | "no-feature-addition.md"
            )
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

    if !matches!(system_context, Some(SystemContext::Existing)) {
        blockers.push(
            "refactor preservation requires `system_context = existing` so structural work stays attached to an existing system"
                .to_string(),
        );
    }

    GateEvaluation {
        gate: GateKind::ChangePreservation,
        status: gate_status_from_blockers(&blockers),
        blockers,
        evaluated_at: OffsetDateTime::now_utc(),
    }
}

fn change_risk_gate(
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

fn implementation_execution_gate(owner: &str, approvals: &[ApprovalRecord]) -> GateEvaluation {
    execution_gate(
        owner,
        approvals,
        "implementation execution mutates the workspace and requires explicit approval before it can proceed",
    )
}

fn backlog_architecture_gate(
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

    if matches!(closure_assessment.status, crate::domain::run::ClosureStatus::Blocked) {
        blockers.extend(
                closure_assessment
                    .findings
                    .iter()
                    .filter(|finding| {
                        matches!(finding.severity, crate::domain::run::ClosureFindingSeverity::Blocking)
                    })
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

fn refactor_execution_gate(owner: &str, approvals: &[ApprovalRecord]) -> GateEvaluation {
    execution_gate(
        owner,
        approvals,
        "refactor execution mutates the workspace and requires explicit approval before it can proceed",
    )
}

fn execution_gate(
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

fn change_release_readiness_gate(
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

fn implementation_release_readiness_gate(
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

fn refactor_release_readiness_gate(
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

fn review_disposition_gate_for_file(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    file_name: &str,
    disposition_approved: bool,
) -> GateEvaluation {
    let mut blockers = contract
        .artifact_requirements
        .iter()
        .filter(|requirement| requirement.file_name == file_name)
        .flat_map(|requirement| {
            artifacts
                .iter()
                .find(|(file_name, _)| file_name == &requirement.file_name)
                .map(|(_, contents)| {
                    crate::artifacts::contract::validate_artifact(requirement, contents)
                })
                .unwrap_or_else(|| vec![format!("missing required artifact `{file_name}`")])
        })
        .collect::<Vec<_>>();

    let summary = artifacts
        .iter()
        .find(|(artifact_file_name, _)| artifact_file_name == file_name)
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

fn review_release_readiness_gate(
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
        .find(|(file_name, _)| file_name == "review-disposition.md")
        .map(|(_, contents)| contents.as_str())
        .unwrap_or_default();
    let missing_evidence = artifacts
        .iter()
        .find(|(file_name, _)| file_name == "missing-evidence.md")
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

fn verification_release_readiness_gate(
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
        .find(|(file_name, _)| file_name == "unresolved-findings.md")
        .map(|(_, contents)| contents.as_str())
        .unwrap_or_default();
    let verdict = artifacts
        .iter()
        .find(|(file_name, _)| file_name == "verification-report.md")
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
        ChangeGateContext, ImplementationGateContext, PrReviewGateContext, RefactorGateContext,
        evaluate_change_gates, evaluate_implementation_gates, evaluate_pr_review_gates,
        evaluate_refactor_gates, evaluate_requirements_gates,
    };
    use crate::artifacts::contract::contract_for_mode;
    use crate::domain::approval::{ApprovalDecision, ApprovalRecord};
    use crate::domain::artifact::{ArtifactContract, ArtifactRequirement};
    use crate::domain::execution::DeniedInvocation;
    use crate::domain::gate::{GateEvaluation, GateKind, GateStatus};
    use crate::domain::mode::Mode;
    use crate::domain::policy::{RiskClass, UsageZone};
    use crate::domain::run::SystemContext;

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
    fn requirements_release_readiness_blocks_raw_input_replay_in_problem_statement() {
        let contract = contract_for_mode(Mode::Requirements);
        let mut artifacts = valid_artifacts(&contract);
        let problem_statement = artifacts
            .iter_mut()
            .find(|(file_name, _)| file_name == "problem-statement.md")
            .expect("problem statement artifact");
        problem_statement
            .1
            .push_str("\n\n## Input: canon-input/requirements/source.md\n\nRaw dump");

        let evaluations = evaluate_requirements_gates(&contract, &artifacts, "Owner", &[], true);
        let release = gate(&evaluations, GateKind::ReleaseReadiness);

        assert_eq!(release.status, GateStatus::Blocked);
        assert!(
            release.blockers.iter().any(|blocker| blocker.contains("replaying raw input labels"))
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
            preservation
                .blockers
                .iter()
                .any(|blocker| { blocker.contains("system_context = existing") })
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
