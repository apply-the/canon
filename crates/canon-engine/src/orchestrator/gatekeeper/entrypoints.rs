use super::{rules, *};
use crate::domain::approval::ApprovalRecord;
use crate::domain::artifact::ArtifactContract;
use crate::domain::execution::DeniedInvocation;
use crate::domain::gate::{GateEvaluation, GateKind, GateStatus};
use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::run::ClosureDecompositionScope;

#[derive(Clone, Copy)]
struct AnalysisModeSpec<'a> {
    review_gate: GateKind,
    review_artifacts: &'a [&'a str],
    review_message: &'a str,
    approval_message: &'a str,
    readiness_message: &'a str,
}

trait AnalysisModeGateContextView<'a> {
    fn owner(&self) -> &'a str;
    fn risk(&self) -> RiskClass;
    fn zone(&self) -> UsageZone;
    fn approvals(&self) -> &'a [ApprovalRecord];
    fn validation_independence_satisfied(&self) -> bool;
    fn evidence_complete(&self) -> bool;
}

impl<'a> AnalysisModeSpec<'a> {
    const fn new(
        review_gate: GateKind,
        review_artifacts: &'a [&'a str],
        review_message: &'a str,
        approval_message: &'a str,
        readiness_message: &'a str,
    ) -> Self {
        Self { review_gate, review_artifacts, review_message, approval_message, readiness_message }
    }
}

macro_rules! impl_analysis_mode_gate_context_view {
    ($($context:ty),+ $(,)?) => {
        $(
            impl<'a> AnalysisModeGateContextView<'a> for $context {
                fn owner(&self) -> &'a str { self.owner }

                fn risk(&self) -> RiskClass { self.risk }

                fn zone(&self) -> UsageZone { self.zone }

                fn approvals(&self) -> &'a [ApprovalRecord] { self.approvals }

                fn validation_independence_satisfied(&self) -> bool {
                    self.validation_independence_satisfied
                }

                fn evidence_complete(&self) -> bool { self.evidence_complete }
            }
        )+
    };
}

impl_analysis_mode_gate_context_view!(
    DiscoveryGateContext<'a>,
    SecurityAssessmentGateContext<'a>,
    SystemAssessmentGateContext<'a>,
    DomainLanguageGateContext<'a>,
    DomainModelGateContext<'a>,
    BrainstormingGateContext<'a>,
);

const DISCOVERY_ANALYSIS_SPEC: AnalysisModeSpec<'static> = AnalysisModeSpec::new(
    GateKind::Exploration,
    &["problem-map.md", "context-boundary.md"],
    "discovery requires a bounded problem domain and explicit context boundary",
    "systemic-impact or red-zone discovery work requires explicit approval before it can proceed",
    "discovery readiness requires persisted context, critique, and repository validation evidence",
);

const BRAINSTORMING_ANALYSIS_SPEC: AnalysisModeSpec<'static> = AnalysisModeSpec::new(
    GateKind::Exploration,
    &["context.md", "options.md", "tradeoffs.md", "spikes.md", "open-questions.md"],
    "brainstorming requires context, options, tradeoffs, spikes, and open questions",
    "systemic-impact or red-zone brainstorming work requires explicit approval before it can proceed",
    "brainstorming readiness requires persisted context, critique, and verification evidence",
);

const SECURITY_ASSESSMENT_ANALYSIS_SPEC: AnalysisModeSpec<'static> = AnalysisModeSpec::new(
    GateKind::Architecture,
    &["assessment-overview.md", "threat-model.md", "risk-register.md", "mitigations.md"],
    "security assessment review requires scope, threats, risks, and mitigation guidance",
    "systemic-impact or red-zone security-assessment work requires explicit approval before it can proceed",
    "security-assessment readiness requires persisted context, critique, and verification evidence",
);

const SYSTEM_ASSESSMENT_ANALYSIS_SPEC: AnalysisModeSpec<'static> = AnalysisModeSpec::new(
    GateKind::Architecture,
    &["assessment-overview.md", "coverage-map.md", "component-view.md", "integration-view.md"],
    "system assessment review requires scope, coverage, component mapping, and integration evidence",
    "systemic-impact or red-zone system-assessment work requires explicit approval before it can proceed",
    "system-assessment readiness requires persisted context, critique, and verification evidence",
);

const DOMAIN_LANGUAGE_ANALYSIS_SPEC: AnalysisModeSpec<'static> = AnalysisModeSpec::new(
    GateKind::Architecture,
    &["language-overview.md", "domain-glossary.md", "preferred-language.md"],
    "domain-language review requires scope, glossary, and preferred language evidence",
    "systemic-impact or red-zone domain-language work requires explicit approval before it can proceed",
    "domain-language readiness requires persisted context, critique, and verification evidence",
);

const DOMAIN_MODEL_ANALYSIS_SPEC: AnalysisModeSpec<'static> = AnalysisModeSpec::new(
    GateKind::Architecture,
    &["model-overview.md", "concept-catalog.md", "relationship-map.md", "bounded-context-map.md"],
    "domain-model review requires scope, concepts, relationships, and bounded context evidence",
    "systemic-impact or red-zone domain-model work requires explicit approval before it can proceed",
    "domain-model readiness requires persisted context, critique, and verification evidence",
);

fn analysis_mode_gate_set<'a>(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    spec: AnalysisModeSpec<'a>,
    context: impl AnalysisModeGateContextView<'a>,
) -> Vec<GateEvaluation> {
    vec![
        rules::named_artifact_gate(
            spec.review_gate,
            contract,
            artifacts,
            spec.review_artifacts,
            spec.review_message,
        ),
        rules::approval_aware_risk_gate(
            context.owner(),
            context.risk(),
            context.zone(),
            context.approvals(),
            spec.approval_message,
        ),
        rules::analysis_release_readiness_gate(
            GateKind::ReleaseReadiness,
            contract,
            artifacts,
            context.validation_independence_satisfied(),
            context.evidence_complete(),
            spec.readiness_message,
        ),
    ]
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
        rules::exploration_gate(artifacts),
        rules::risk_gate(owner),
        rules::requirements_release_readiness_gate(
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
    analysis_mode_gate_set(contract, artifacts, DISCOVERY_ANALYSIS_SPEC, context)
}

/// Evaluates the gate set for a Brainstorming mode run.
pub fn evaluate_brainstorming_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: BrainstormingGateContext<'_>,
) -> Vec<GateEvaluation> {
    analysis_mode_gate_set(contract, artifacts, BRAINSTORMING_ANALYSIS_SPEC, context)
}

/// Evaluates the gate set for a System Shaping mode run.
pub fn evaluate_system_shaping_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: SystemShapingGateContext<'_>,
) -> Vec<GateEvaluation> {
    vec![
        rules::named_artifact_gate(
            GateKind::Exploration,
            contract,
            artifacts,
            &["system-shape.md", "capability-map.md"],
            "system-shaping requires a bounded system shape and capability map",
        ),
        rules::named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["system-shape.md", "domain-model.md", "architecture-outline.md", "capability-map.md"],
            "system-shaping architecture review requires bounded structure, capabilities, and rationale",
        ),
        rules::approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone system-shaping work requires explicit approval before it can proceed",
        ),
        rules::analysis_release_readiness_gate(
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
        rules::named_artifact_gate(
            GateKind::Exploration,
            contract,
            artifacts,
            &["boundary-map.md"],
            "architecture exploration requires an explicit boundary map",
        ),
        rules::named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["architecture-decisions.md", "invariants.md", "tradeoff-matrix.md", "context-map.md"],
            "architecture review requires decisions, invariants, and tradeoff analysis",
        ),
        rules::approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone architecture work requires explicit approval before it can proceed",
        ),
        rules::analysis_release_readiness_gate(
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
        rules::named_artifact_gate(
            GateKind::Exploration,
            contract,
            artifacts,
            &["system-slice.md"],
            "change exploration requires a bounded system slice",
        ),
        rules::change_preservation_gate(contract, artifacts, context.system_context),
        rules::named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["implementation-plan.md", "decision-record.md"],
            "change architecture review requires an implementation plan and decision record",
        ),
        rules::change_risk_gate(context.owner, context.risk, context.zone, context.approvals),
        rules::change_release_readiness_gate(
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
        rules::named_artifact_gate(
            GateKind::Exploration,
            &effective_contract,
            artifacts,
            exploration_names,
            "backlog requires an overview and capability mapping before decomposition can proceed",
        ),
        rules::backlog_architecture_gate(
            &effective_contract,
            artifacts,
            context.system_context,
            context.closure_assessment,
        ),
        rules::approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone backlog work requires explicit approval before it can proceed",
        ),
        rules::analysis_release_readiness_gate(
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
    let readiness =
        rules::implementation_readiness_gate(contract, artifacts, context.system_context);
    let release = rules::implementation_release_readiness_gate(
        contract,
        artifacts,
        context.validation_independence_satisfied,
        context.evidence_complete,
    );
    let mut gates = vec![readiness];
    if !gates.iter().any(|gate| matches!(gate.status, GateStatus::Blocked))
        && !matches!(release.status, GateStatus::Blocked)
    {
        gates.push(rules::implementation_execution_gate(context.owner, context.approvals));
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
        rules::approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone incident work requires explicit approval before it can proceed",
        ),
        rules::operational_capture_gate(
            rules::named_artifact_gate(
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
        rules::named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["incident-frame.md", "incident-decision-record.md"],
            "incident review requires an incident frame and decision record before architecture can pass",
        ),
        rules::operational_capture_gate(
            rules::analysis_release_readiness_gate(
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
        rules::named_artifact_gate(
            GateKind::Exploration,
            contract,
            artifacts,
            &["source-target-map.md"],
            "migration exploration requires a bounded source-target map",
        ),
        rules::named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["source-target-map.md", "compatibility-matrix.md", "decision-record.md"],
            "migration architecture review requires source-target mapping, compatibility posture, and decision capture",
        ),
        rules::operational_capture_gate(
            rules::named_artifact_gate(
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
        rules::approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone migration work requires explicit approval before it can proceed",
        ),
        rules::operational_capture_gate(
            rules::analysis_release_readiness_gate(
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
    analysis_mode_gate_set(contract, artifacts, SECURITY_ASSESSMENT_ANALYSIS_SPEC, context)
}

/// Evaluates the gate set for a System Assessment mode run.
pub fn evaluate_system_assessment_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: SystemAssessmentGateContext<'_>,
) -> Vec<GateEvaluation> {
    analysis_mode_gate_set(contract, artifacts, SYSTEM_ASSESSMENT_ANALYSIS_SPEC, context)
}

/// Evaluates the gate set for a Supply Chain Analysis mode run.
pub fn evaluate_supply_chain_analysis_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: SupplyChainAnalysisGateContext<'_>,
) -> Vec<GateEvaluation> {
    let artifact_risk = rules::operational_capture_gate(
        rules::named_artifact_gate(
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
    let mut risk = rules::approval_aware_risk_gate(
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
        rules::operational_capture_gate(
            rules::analysis_release_readiness_gate(
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

/// Evaluates the gate set for a Refactor mode run.
pub fn evaluate_refactor_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: RefactorGateContext<'_>,
) -> Vec<GateEvaluation> {
    let preservation =
        rules::refactor_preservation_gate(contract, artifacts, context.system_context);
    let architecture = rules::named_artifact_gate(
        GateKind::Architecture,
        contract,
        artifacts,
        &["structural-rationale.md", "contract-drift-check.md"],
        "refactor architecture review requires structural rationale and contract drift review",
    );
    let release = rules::refactor_release_readiness_gate(
        contract,
        artifacts,
        context.validation_independence_satisfied,
        context.evidence_complete,
    );
    let mut gates = vec![preservation, architecture];
    if !gates.iter().any(|gate| matches!(gate.status, GateStatus::Blocked))
        && !matches!(release.status, GateStatus::Blocked)
    {
        gates.push(rules::refactor_execution_gate(context.owner, context.approvals));
    }
    gates.push(release);
    gates
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
        rules::approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone review work requires explicit approval before it can proceed",
        ),
        rules::named_artifact_gate(
            GateKind::Architecture,
            contract,
            artifacts,
            &["boundary-assessment.md", "decision-impact.md"],
            "review requires explicit boundary assessment and decision impact before disposition",
        ),
        rules::review_disposition_gate_for_file(
            contract,
            artifacts,
            "review-disposition.md",
            disposition_approved,
        ),
        rules::review_release_readiness_gate(
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
        rules::approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone verification work requires explicit approval before it can proceed",
        ),
        rules::verification_release_readiness_gate(
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
        rules::approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone review work requires explicit approval before it can proceed",
        ),
        rules::pr_review_architecture_gate(contract, artifacts),
        rules::review_disposition_gate_for_file(
            contract,
            artifacts,
            "review-summary.md",
            disposition_approved,
        ),
        rules::pr_review_release_readiness_gate(
            contract,
            artifacts,
            disposition_approved,
            context.denied_invocations,
            context.evidence_complete,
        ),
    ]
}

/// Evaluates the gate set for a Domain Language mode run.
pub fn evaluate_domain_language_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: DomainLanguageGateContext<'_>,
) -> Vec<GateEvaluation> {
    analysis_mode_gate_set(contract, artifacts, DOMAIN_LANGUAGE_ANALYSIS_SPEC, context)
}

/// Evaluates the gate set for a Domain Model mode run.
pub fn evaluate_domain_model_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: DomainModelGateContext<'_>,
) -> Vec<GateEvaluation> {
    analysis_mode_gate_set(contract, artifacts, DOMAIN_MODEL_ANALYSIS_SPEC, context)
}

/// Evaluates the gate set for a Debugging mode run.
pub fn evaluate_debugging_gates(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
    context: DebuggingGateContext<'_>,
) -> Vec<GateEvaluation> {
    vec![
        rules::named_artifact_gate(
            GateKind::Exploration,
            contract,
            artifacts,
            &["context-map.md"],
            "debugging exploration requires a bounded context map",
        ),
        rules::named_artifact_gate(
            GateKind::Reproduction,
            contract,
            artifacts,
            &["reproduction-harness.md"],
            "debugging requires an explicit reproduction harness",
        ),
        rules::named_artifact_gate(
            GateKind::RootCause,
            contract,
            artifacts,
            &["root-cause-isolation.md", "fix-application.md"],
            "debugging requires root cause isolation and fix application evidence",
        ),
        rules::approval_aware_risk_gate(
            context.owner,
            context.risk,
            context.zone,
            context.approvals,
            "systemic-impact or red-zone debugging work requires explicit approval before it can proceed",
        ),
        rules::analysis_release_readiness_gate(
            GateKind::ReleaseReadiness,
            contract,
            artifacts,
            context.validation_independence_satisfied,
            context.evidence_complete,
            "debugging readiness requires persisted context, critique, and verification evidence",
        ),
    ]
}
