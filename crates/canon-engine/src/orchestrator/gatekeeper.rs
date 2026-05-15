use time::OffsetDateTime;

use crate::artifacts::contract::validate_release_bundle;
use crate::domain::approval::ApprovalRecord;
use crate::domain::artifact::{ArtifactContract, REVIEW_SUMMARY_ARTIFACT_SLUG, artifact_slug};
use crate::domain::execution::DeniedInvocation;
use crate::domain::gate::{GateEvaluation, GateKind, GateStatus};
use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::run::{ClosureAssessment, ClosureDecompositionScope, SystemContext};

/// Evaluation context for Discovery mode gate checks.
pub struct DiscoveryGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether validation and generation are performed by independent actors.
    pub validation_independence_satisfied: bool,
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluation context for System Shaping mode gate checks.
pub struct SystemShapingGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluation context for Architecture mode gate checks.
pub struct ArchitectureGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluation context for Backlog mode gate checks.
pub struct BacklogGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Whether the run targets a new or existing system.
    pub system_context: Option<SystemContext>,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether validation and generation are performed by independent actors.
    pub validation_independence_satisfied: bool,
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
    /// Closure assessment for the backlog packet.
    pub closure_assessment: &'a ClosureAssessment,
}

/// Evaluation context for Change mode gate checks.
pub struct ChangeGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether the run targets a new or existing system.
    pub system_context: Option<SystemContext>,
    /// Whether validation and generation are performed by independent actors.
    pub validation_independence_satisfied: bool,
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluation context for Implementation mode gate checks.
pub struct ImplementationGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether the run targets a new or existing system.
    pub system_context: Option<SystemContext>,
    /// Whether validation and generation are performed by independent actors.
    pub validation_independence_satisfied: bool,
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluation context for Incident mode gate checks.
pub struct IncidentGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether validation and generation are performed by independent actors.
    pub validation_independence_satisfied: bool,
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluation context for Migration mode gate checks.
pub struct MigrationGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether validation and generation are performed by independent actors.
    pub validation_independence_satisfied: bool,
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluation context for Security Assessment mode gate checks.
pub struct SecurityAssessmentGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether validation and generation are performed by independent actors.
    pub validation_independence_satisfied: bool,
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluation context for System Assessment mode gate checks.
pub struct SystemAssessmentGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether validation and generation are performed by independent actors.
    pub validation_independence_satisfied: bool,
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluation context for Supply Chain Analysis mode gate checks.
pub struct SupplyChainAnalysisGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether validation and generation are performed by independent actors.
    pub validation_independence_satisfied: bool,
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluation context for Domain Language mode gate checks.
pub struct DomainLanguageGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether validation and generation are performed by independent actors.
    pub validation_independence_satisfied: bool,
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluation context for Domain Model mode gate checks.
pub struct DomainModelGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether validation and generation are performed by independent actors.
    pub validation_independence_satisfied: bool,
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluation context for Refactor mode gate checks.
pub struct RefactorGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether the run targets a new or existing system.
    pub system_context: Option<SystemContext>,
    /// Whether validation and generation are performed by independent actors.
    pub validation_independence_satisfied: bool,
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluation context for Review mode gate checks.
pub struct ReviewGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluation context for Verification mode gate checks.
pub struct VerificationGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Whether validation and generation are performed by independent actors.
    pub validation_independence_satisfied: bool,
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluation context for PR Review mode gate checks.
pub struct PrReviewGateContext<'a> {
    /// Named human owner required for high-risk/red-zone work.
    pub owner: &'a str,
    /// Assigned risk class for the run.
    pub risk: RiskClass,
    /// Assigned usage zone for the run.
    pub zone: UsageZone,
    /// Approval records recorded against this run.
    pub approvals: &'a [ApprovalRecord],
    /// Invocations that were denied during the run.
    pub denied_invocations: &'a [DeniedInvocation],
    /// Whether all required evidence has been captured.
    pub evidence_complete: bool,
}

/// Evaluates the gate set for a Requirements mode run.
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

/// Evaluates the gate set for a Discovery mode run.
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

/// Evaluates the gate set for a System Shaping mode run.
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
            &["system-shape.md", "domain-model.md", "architecture-outline.md", "capability-map.md"],
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

/// Evaluates the gate set for an Architecture mode run.
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
            &["architecture-decisions.md", "invariants.md", "tradeoff-matrix.md", "context-map.md"],
            "architecture review requires decisions, invariants, and tradeoff analysis",
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

/// Evaluates the gate set for a Change mode run.
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

/// Evaluates the gate set for a Backlog mode run.
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

/// Evaluates the gate set for an Implementation mode run.
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

/// Evaluates the gate set for an Incident mode run.
pub fn evaluate_incident_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: IncidentGateContext<'_>,
) -> Vec<GateEvaluation> {
    vec![
        approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone incident work requires explicit approval before it can proceed",
        ),
        operational_capture_gate(
            named_artifact_gate(
                GateKind::IncidentContainment,
                contract,
                artifacts,
                &["incident-frame.md", "blast-radius-map.md", "containment-plan.md"],
                "incident containment requires an explicit incident frame, blast radius map, and containment plan",
            ),
            artifacts,
            &["incident-frame.md", "blast-radius-map.md", "containment-plan.md"],
            GateKind::IncidentContainment,
        ),
        named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["incident-frame.md", "incident-decision-record.md"],
            "incident review requires an incident frame and decision record before architecture can pass",
        ),
        operational_capture_gate(
            analysis_release_readiness_gate(
                GateKind::ReleaseReadiness,
                contract,
                artifacts,
                context.validation_independence_satisfied,
                context.evidence_complete,
                "incident readiness requires persisted context, critique, and follow-up verification evidence",
            ),
            artifacts,
            &[
                "incident-frame.md",
                "blast-radius-map.md",
                "containment-plan.md",
                "incident-decision-record.md",
                "follow-up-verification.md",
            ],
            GateKind::ReleaseReadiness,
        ),
    ]
}

/// Evaluates the gate set for a Migration mode run.
pub fn evaluate_migration_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: MigrationGateContext<'_>,
) -> Vec<GateEvaluation> {
    vec![
        named_artifact_gate(
            GateKind::Exploration,
            contract,
            artifacts,
            &["source-target-map.md"],
            "migration exploration requires a bounded source-target map",
        ),
        named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["source-target-map.md", "compatibility-matrix.md", "decision-record.md"],
            "migration architecture review requires source-target mapping, compatibility posture, and decision capture",
        ),
        operational_capture_gate(
            named_artifact_gate(
                GateKind::MigrationSafety,
                contract,
                artifacts,
                &[
                    "compatibility-matrix.md",
                    "sequencing-plan.md",
                    "fallback-plan.md",
                    "migration-verification-report.md",
                ],
                "migration safety requires compatibility, sequencing, fallback, and verification artifacts",
            ),
            artifacts,
            &[
                "compatibility-matrix.md",
                "sequencing-plan.md",
                "fallback-plan.md",
                "migration-verification-report.md",
            ],
            GateKind::MigrationSafety,
        ),
        approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone migration work requires explicit approval before it can proceed",
        ),
        operational_capture_gate(
            analysis_release_readiness_gate(
                GateKind::ReleaseReadiness,
                contract,
                artifacts,
                context.validation_independence_satisfied,
                context.evidence_complete,
                "migration readiness requires persisted context, critique, and verification evidence",
            ),
            artifacts,
            &[
                "source-target-map.md",
                "compatibility-matrix.md",
                "sequencing-plan.md",
                "fallback-plan.md",
                "migration-verification-report.md",
                "decision-record.md",
            ],
            GateKind::ReleaseReadiness,
        ),
    ]
}

/// Evaluates the gate set for a Security Assessment mode run.
pub fn evaluate_security_assessment_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: SecurityAssessmentGateContext<'_>,
) -> Vec<GateEvaluation> {
    vec![
        named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["assessment-overview.md", "threat-model.md", "risk-register.md", "mitigations.md"],
            "security assessment review requires scope, threats, risks, and mitigation guidance",
        ),
        approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone security-assessment work requires explicit approval before it can proceed",
        ),
        analysis_release_readiness_gate(
            GateKind::ReleaseReadiness,
            contract,
            artifacts,
            context.validation_independence_satisfied,
            context.evidence_complete,
            "security-assessment readiness requires persisted context, critique, and verification evidence",
        ),
    ]
}

/// Evaluates the gate set for a System Assessment mode run.
pub fn evaluate_system_assessment_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: SystemAssessmentGateContext<'_>,
) -> Vec<GateEvaluation> {
    vec![
        named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &[
                "assessment-overview.md",
                "coverage-map.md",
                "component-view.md",
                "integration-view.md",
            ],
            "system assessment review requires scope, coverage, component mapping, and integration evidence",
        ),
        approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone system-assessment work requires explicit approval before it can proceed",
        ),
        analysis_release_readiness_gate(
            GateKind::ReleaseReadiness,
            contract,
            artifacts,
            context.validation_independence_satisfied,
            context.evidence_complete,
            "system-assessment readiness requires persisted context, critique, and verification evidence",
        ),
    ]
}

/// Evaluates the gate set for a Supply Chain Analysis mode run.
pub fn evaluate_supply_chain_analysis_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: SupplyChainAnalysisGateContext<'_>,
) -> Vec<GateEvaluation> {
    let artifact_risk = operational_capture_gate(
        named_artifact_gate(
            GateKind::Risk,
            contract,
            artifacts,
            &[
                "analysis-overview.md",
                "sbom-bundle.md",
                "vulnerability-triage.md",
                "license-compliance.md",
                "legacy-posture.md",
            ],
            "supply-chain analysis requires scope, SBOM, vulnerability, license, and legacy posture evidence",
        ),
        artifacts,
        &[
            "analysis-overview.md",
            "sbom-bundle.md",
            "vulnerability-triage.md",
            "license-compliance.md",
            "legacy-posture.md",
        ],
        GateKind::Risk,
    );
    let mut risk = approval_aware_risk_gate(
        context.owner,
        context.risk,
        context.zone,
        context.approvals,
        "systemic-impact or red-zone supply-chain-analysis work requires explicit approval before it can proceed",
    );
    if !artifact_risk.blockers.is_empty() {
        risk.status = GateStatus::Blocked;
        risk.blockers.extend(artifact_risk.blockers);
    }

    vec![
        risk,
        operational_capture_gate(
            analysis_release_readiness_gate(
                GateKind::ReleaseReadiness,
                contract,
                artifacts,
                context.validation_independence_satisfied,
                context.evidence_complete,
                "supply-chain-analysis readiness requires persisted context, critique, and verification evidence",
            ),
            artifacts,
            &["analysis-evidence.md", "policy-decisions.md"],
            GateKind::ReleaseReadiness,
        ),
    ]
}

fn operational_capture_gate(
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

/// Evaluates the gate set for a Refactor mode run.
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
            matches!(requirement.slug(), "legacy-invariants.md" | "change-surface.md")
        })
        .flat_map(|requirement| {
            artifacts
                .iter()
                .find(|(file_name, _)| file_name == &requirement.file_name)
                .map(|(_, contents)| {
                    crate::artifacts::contract::validate_artifact(requirement, contents)
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

/// Evaluates the gate set for a Review mode run.
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

/// Evaluates the gate set for a Verification mode run.
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

/// Evaluates the gate set for a PR Review mode run.
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
                requirement.slug(),
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
                    if requirement.required {
                        vec![format!("missing required artifact `{}`", requirement.file_name)]
                    } else {
                        Vec::new()
                    }
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
                requirement.slug(),
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
                    if requirement.required {
                        vec![format!("missing required artifact `{}`", requirement.file_name)]
                    } else {
                        Vec::new()
                    }
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
            matches!(requirement.slug(), "boundary-check.md" | "contract-drift.md")
        })
        .flat_map(|requirement| {
            artifacts
                .iter()
                .find(|(file_name, _)| file_name == &requirement.file_name)
                .map(|(_, contents)| {
                    crate::artifacts::contract::validate_artifact(requirement, contents)
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

fn review_disposition_gate_for_file(
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
                .find(|(file_name, _)| file_name == &requirement.file_name)
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

/// Evaluates the gate set for a Domain Language mode run.
pub fn evaluate_domain_language_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: DomainLanguageGateContext<'_>,
) -> Vec<GateEvaluation> {
    vec![
        named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["language-overview.md", "domain-glossary.md", "preferred-language.md"],
            "domain-language review requires scope, glossary, and preferred language evidence",
        ),
        approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone domain-language work requires explicit approval before it can proceed",
        ),
        analysis_release_readiness_gate(
            GateKind::ReleaseReadiness,
            contract,
            artifacts,
            context.validation_independence_satisfied,
            context.evidence_complete,
            "domain-language readiness requires persisted context, critique, and verification evidence",
        ),
    ]
}

/// Evaluates the gate set for a Domain Model mode run.
pub fn evaluate_domain_model_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: DomainModelGateContext<'_>,
) -> Vec<GateEvaluation> {
    vec![
        named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &[
                "model-overview.md",
                "concept-catalog.md",
                "relationship-map.md",
                "bounded-context-map.md",
            ],
            "domain-model review requires scope, concepts, relationships, and bounded context evidence",
        ),
        approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone domain-model work requires explicit approval before it can proceed",
        ),
        analysis_release_readiness_gate(
            GateKind::ReleaseReadiness,
            contract,
            artifacts,
            context.validation_independence_satisfied,
            context.evidence_complete,
            "domain-model readiness requires persisted context, critique, and verification evidence",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use time::OffsetDateTime;

    use super::{
        ChangeGateContext, ImplementationGateContext, IncidentGateContext, MigrationGateContext,
        PrReviewGateContext, RefactorGateContext, evaluate_change_gates,
        evaluate_implementation_gates, evaluate_incident_gates, evaluate_migration_gates,
        evaluate_pr_review_gates, evaluate_refactor_gates, evaluate_requirements_gates,
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

        let evaluation = super::named_artifact_gate(
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
                    &[
                        "Summary",
                        "Impacted Surfaces",
                        "Propagation Paths",
                        "Confidence And Unknowns",
                    ],
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
            .find(|(file_name, _)| artifact_slug(file_name) == "problem-statement.md")
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

    #[test]
    fn change_preservation_gate_ignores_missing_optional_artifact() {
        // A contract where "change-surface.md" is optional and not supplied.
        // The gate should pass because the missing artifact is not required.
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
        let artifacts = vec![(
            "legacy-invariants.md".to_string(),
            "## Legacy Invariants\n\nBounded.".to_string(),
        )];

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
            !preservation.blockers.iter().any(|b| b.contains("change-surface.md")),
            "optional artifact absence must not produce a blocker"
        );
    }

    #[test]
    fn implementation_readiness_gate_ignores_missing_optional_artifact() {
        // A contract where "mutation-bounds.md" is optional and not supplied.
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
            !readiness.blockers.iter().any(|b| b.contains("mutation-bounds.md")),
            "optional artifact absence must not produce a blocker"
        );
    }

    #[test]
    fn refactor_preservation_gate_ignores_missing_optional_artifact() {
        // A contract where "refactor-scope.md" is optional and not supplied.
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
            !preservation.blockers.iter().any(|b| b.contains("refactor-scope.md")),
            "optional artifact absence must not produce a blocker"
        );
    }
}
