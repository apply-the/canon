use super::*;

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
