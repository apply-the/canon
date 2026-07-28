//! Provider-neutral reviewer adapter contract for actionable pr-review.
//!
//! Canon accepts externally produced reviewer evidence but does not host or
//! execute an AI/LLM reviewer. The types below preserve existing review
//! projections while the external-evidence validator enforces the M2a
//! structural, binding, freshness, and lineage boundary.

use serde::{Deserialize, Serialize};

use canon_contracts::{ChallengeTier, Claim, EvidenceReference, ExternalVerificationEvidence};

const PROVIDER_LINEAGE_PREFIX: &str = "provider:";
const EXECUTOR_LINEAGE_SEGMENT: &str = "/executor:";
const INVOCATION_LINEAGE_SEGMENT: &str = "/invocation:";
const HUMAN_LINEAGE_PREFIX: &str = "human:";
const DETERMINISTIC_LINEAGE_MARKER: &str = "canon-deterministic";
const NON_GENERATIVE_LINEAGE_MARKER: &str = "non-generative";

/// Structured output from a reviewer adapter invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewerOutput {
    /// Status of the review execution.
    pub status: ReviewerStatus,
    /// The reviewer adapter kind that produced this output.
    pub reviewer: String,
    /// Actionable findings produced by the reviewer.
    pub findings: Vec<ReviewerFinding>,
    /// Review coverage metadata.
    pub coverage: ReviewerCoverage,
}

/// Whether the reviewer executed successfully, failed, or was not configured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewerStatus {
    /// Reviewer executed and returned valid output.
    Executed,
    /// Reviewer was invoked but returned invalid or no output.
    Failed,
    /// No reviewer adapter is configured.
    NotConfigured,
    /// The run explicitly performed governance-only inspection;
    /// no actionable code review was attempted.
    GovernanceOnly,
}

impl ReviewerStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Executed => "actionable_review_executed",
            Self::Failed => "actionable_review_failed",
            Self::NotConfigured => "actionable_review_not_configured",
            Self::GovernanceOnly => "governance_only",
        }
    }
}

/// A single finding from the reviewer, which may become a canonical comment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewerFinding {
    /// Stable finding ID (assigned by the reviewer or the runtime).
    pub id: String,
    /// File path, when the finding applies to a specific file.
    pub path: Option<String>,
    /// Line number, when the finding applies to a specific line.
    pub line: Option<u32>,
    /// Side (LEFT/RIGHT) when line is present.
    pub side: Option<String>,
    /// Diff hunk header when exact line is not determined.
    pub hunk_header: Option<String>,
    /// Severity: blocking, major, minor, question, nitpick.
    pub severity: ReviewerSeverity,
    /// Conventional comment type: issue, suggestion, question, nitpick, praise.
    pub kind: String,
    /// Brief summary of the finding.
    pub summary: String,
    /// Why this finding matters.
    pub why_it_matters: String,
    /// Suggested remediation.
    pub suggested_remediation: String,
    /// Optional suggested code change.
    pub suggested_change: Option<String>,
}

/// Severity levels for reviewer findings, ordered from most to least critical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReviewerSeverity {
    Blocking,
    Major,
    Minor,
    Question,
    Nitpick,
}

impl ReviewerSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::Major => "major",
            Self::Minor => "minor",
            Self::Question => "question",
            Self::Nitpick => "nitpick",
        }
    }

    /// Severity ordering for sorting: blocking > major > minor > question > nitpick.
    pub fn order(&self) -> u8 {
        match self {
            Self::Blocking => 0,
            Self::Major => 1,
            Self::Minor => 2,
            Self::Question => 3,
            Self::Nitpick => 4,
        }
    }
}

/// Coverage metadata from the reviewer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewerCoverage {
    /// Total changed files in the diff.
    pub changed_files_count: u32,
    /// Files inspected deeply by the reviewer.
    pub files_inspected_deeply: Vec<String>,
    /// Files skipped by the reviewer.
    pub files_skipped: Vec<String>,
    /// Whether the review was exhaustive.
    pub exhaustive: bool,
    /// Coverage limitations, if any.
    pub limitations: Vec<String>,
}

/// Input to a reviewer adapter.
#[derive(Debug, Clone)]
pub struct ReviewerInput {
    /// The raw diff patch.
    pub patch: String,
    /// List of changed file paths.
    pub changed_files: Vec<String>,
    /// Base ref for the review.
    pub base_ref: String,
    /// Head ref for the review.
    pub head_ref: String,
}

/// Freshness determined by the caller from the authoritative packet binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceFreshness {
    /// The supplied evidence still matches its admitted inputs.
    Fresh,
    /// A later mutation or binding change invalidated the evidence.
    Stale,
}

/// Whether the external producer completed a terminal evidence result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalEvidenceState {
    /// The external evidence result is complete and terminal.
    Terminal,
    /// The external producer has not produced a terminal result.
    Incomplete,
}

/// Runtime expectations used to validate a published external-evidence DTO.
///
/// Freshness and terminality remain caller-supplied because
/// `canon-contracts 0.90.0` intentionally has no timestamp, digest, or
/// terminal-status field on `ExternalVerificationEvidence`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalEvidenceValidationContext {
    /// Identity of Canon's deterministic validator.
    pub canon_validator_identity: String,
    /// Identity of the Canon invocation performing structural validation.
    pub current_invocation_identity: String,
    /// Exact claims required by the active governance policy.
    pub required_claims: Vec<Claim>,
    /// Immutable references admitted for this validation.
    pub accepted_evidence_references: Vec<EvidenceReference>,
    /// Minimum independent challenge tier required by policy.
    pub minimum_challenge_tier: ChallengeTier,
    /// Authoritative freshness result supplied by the binding layer.
    pub freshness: EvidenceFreshness,
    /// Authoritative completion result supplied by the producer boundary.
    pub terminal_state: TerminalEvidenceState,
    /// Lineages that share the implementer's context or are otherwise disallowed.
    pub forbidden_lineages: Vec<String>,
}

/// Successful deterministic inspection of externally produced evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalEvidenceAcceptance {
    /// The published fields and caller-supplied bindings satisfy policy.
    pub structure_valid: bool,
    /// Always false: Canon does not attest the external semantic judgment.
    pub semantic_judgment_asserted: bool,
}

/// Stable internal reasons for rejecting external semantic evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalEvidenceRejection {
    /// Producer identity was absent or identified Canon itself.
    InvalidProducerIdentity,
    /// Lineage was absent, malformed, internal, deterministic, or forbidden.
    InvalidLineage,
    /// Independent context was absent or belonged to the current invocation.
    InvalidIndependentContext,
    /// Required claims were absent, duplicated, or did not match.
    ClaimMismatch,
    /// Evidence references were absent, duplicated, or did not match.
    EvidenceReferenceMismatch,
    /// The evidence binding is stale.
    StaleEvidence,
    /// The producer result is not terminal.
    IncompleteEvidence,
    /// The declared challenge tier does not satisfy policy.
    InsufficientChallengeTier,
    /// A named override was present but blank.
    InvalidNamedOverride,
}

impl std::fmt::Display for ExternalEvidenceRejection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reason = match self {
            Self::InvalidProducerIdentity => "external producer identity is invalid",
            Self::InvalidLineage => "external reviewer lineage is invalid",
            Self::InvalidIndependentContext => "independent review context is invalid",
            Self::ClaimMismatch => "external evidence claims do not match policy",
            Self::EvidenceReferenceMismatch => {
                "external evidence references do not match the admitted binding"
            }
            Self::StaleEvidence => "external evidence is stale",
            Self::IncompleteEvidence => "external evidence is not terminal",
            Self::InsufficientChallengeTier => {
                "external evidence does not satisfy the required challenge tier"
            }
            Self::InvalidNamedOverride => "named override must not be blank",
        };
        formatter.write_str(reason)
    }
}

impl std::error::Error for ExternalEvidenceRejection {}

/// Validates supplied external semantic evidence without executing a reviewer.
pub fn validate_external_evidence(
    evidence: &ExternalVerificationEvidence,
    context: &ExternalEvidenceValidationContext,
) -> Result<ExternalEvidenceAcceptance, ExternalEvidenceRejection> {
    validate_producer(evidence, context)?;
    validate_lineage(evidence, context)?;
    validate_claims(evidence, context)?;
    validate_references(evidence, context)?;
    validate_runtime_binding(evidence, context)?;

    Ok(ExternalEvidenceAcceptance { structure_valid: true, semantic_judgment_asserted: false })
}

fn validate_producer(
    evidence: &ExternalVerificationEvidence,
    context: &ExternalEvidenceValidationContext,
) -> Result<(), ExternalEvidenceRejection> {
    if evidence.reviewer_identity.trim().is_empty()
        || evidence.reviewer_identity == context.canon_validator_identity
        || evidence.reviewer_identity == context.current_invocation_identity
    {
        return Err(ExternalEvidenceRejection::InvalidProducerIdentity);
    }
    if evidence.independent_context_identity.trim().is_empty()
        || evidence.independent_context_identity == context.current_invocation_identity
    {
        return Err(ExternalEvidenceRejection::InvalidIndependentContext);
    }
    Ok(())
}

fn validate_lineage(
    evidence: &ExternalVerificationEvidence,
    context: &ExternalEvidenceValidationContext,
) -> Result<(), ExternalEvidenceRejection> {
    let lineage = evidence.lineage.trim();
    let provider_lineage = lineage.starts_with(PROVIDER_LINEAGE_PREFIX)
        && lineage.contains(EXECUTOR_LINEAGE_SEGMENT)
        && lineage.contains(INVOCATION_LINEAGE_SEGMENT);
    let human_lineage =
        lineage.starts_with(HUMAN_LINEAGE_PREFIX) && lineage.len() > HUMAN_LINEAGE_PREFIX.len();
    let forbidden = context
        .forbidden_lineages
        .iter()
        .any(|forbidden| !forbidden.is_empty() && lineage.contains(forbidden));

    if (!provider_lineage && !human_lineage)
        || lineage.contains(&context.current_invocation_identity)
        || lineage.contains(DETERMINISTIC_LINEAGE_MARKER)
        || lineage.contains(NON_GENERATIVE_LINEAGE_MARKER)
        || forbidden
    {
        return Err(ExternalEvidenceRejection::InvalidLineage);
    }
    Ok(())
}

fn validate_claims(
    evidence: &ExternalVerificationEvidence,
    context: &ExternalEvidenceValidationContext,
) -> Result<(), ExternalEvidenceRejection> {
    if evidence.claims.is_empty()
        || has_duplicates(&evidence.claims)
        || !context.required_claims.iter().all(|claim| evidence.claims.contains(claim))
        || evidence.claims.iter().any(|claim| !context.required_claims.contains(claim))
    {
        return Err(ExternalEvidenceRejection::ClaimMismatch);
    }
    Ok(())
}

fn validate_references(
    evidence: &ExternalVerificationEvidence,
    context: &ExternalEvidenceValidationContext,
) -> Result<(), ExternalEvidenceRejection> {
    if evidence.evidence_references.is_empty()
        || has_duplicates(&evidence.evidence_references)
        || has_duplicates(&context.accepted_evidence_references)
        || evidence
            .evidence_references
            .iter()
            .any(|reference| !context.accepted_evidence_references.contains(reference))
        || context
            .accepted_evidence_references
            .iter()
            .any(|reference| !evidence.evidence_references.contains(reference))
    {
        return Err(ExternalEvidenceRejection::EvidenceReferenceMismatch);
    }
    Ok(())
}

fn validate_runtime_binding(
    evidence: &ExternalVerificationEvidence,
    context: &ExternalEvidenceValidationContext,
) -> Result<(), ExternalEvidenceRejection> {
    if context.freshness == EvidenceFreshness::Stale {
        return Err(ExternalEvidenceRejection::StaleEvidence);
    }
    if context.terminal_state == TerminalEvidenceState::Incomplete {
        return Err(ExternalEvidenceRejection::IncompleteEvidence);
    }
    if challenge_tier_rank(evidence.challenge_tier)
        < challenge_tier_rank(context.minimum_challenge_tier)
    {
        return Err(ExternalEvidenceRejection::InsufficientChallengeTier);
    }
    if evidence.named_override.as_ref().is_some_and(|value| value.trim().is_empty()) {
        return Err(ExternalEvidenceRejection::InvalidNamedOverride);
    }
    Ok(())
}

fn has_duplicates<T: Ord + Clone>(values: &[T]) -> bool {
    let mut ordered = values.to_vec();
    ordered.sort();
    ordered.windows(2).any(|pair| pair[0] == pair[1])
}

const fn challenge_tier_rank(tier: ChallengeTier) -> u8 {
    match tier {
        ChallengeTier::Tier0 => 0,
        ChallengeTier::Tier1 => 1,
        ChallengeTier::Tier2 => 2,
        ChallengeTier::Tier3 => 3,
    }
}

/// Result status for a request that asks Canon to execute semantic review.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticReviewExecutionStatus {
    /// Canon does not own or host external semantic reviewers.
    Unsupported,
}

/// Auditable unsupported result for semantic-review execution requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticReviewExecutionOutcome {
    /// Explicit non-success capability status.
    pub status: SemanticReviewExecutionStatus,
    /// Reviewer child-process invocations performed by Canon.
    pub process_invocations: u64,
    /// Reviewer network invocations performed by Canon.
    pub network_invocations: u64,
    /// Provider credential reads performed by Canon.
    pub provider_credential_reads: u64,
    /// Semantic evidence records manufactured by Canon.
    pub created_evidence: u64,
}

/// Fail-closed semantic reviewer execution boundary.
#[derive(Debug, Clone, Copy, Default)]
pub struct SemanticReviewerExecutor;

impl SemanticReviewerExecutor {
    /// Rejects semantic-review execution without acquiring any external capability.
    pub const fn request_execution(self) -> SemanticReviewExecutionOutcome {
        SemanticReviewExecutionOutcome {
            status: SemanticReviewExecutionStatus::Unsupported,
            process_invocations: 0,
            network_invocations: 0,
            provider_credential_reads: 0,
            created_evidence: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reviewer_status_as_str() {
        assert_eq!(ReviewerStatus::Executed.as_str(), "actionable_review_executed");
        assert_eq!(ReviewerStatus::Failed.as_str(), "actionable_review_failed");
        assert_eq!(ReviewerStatus::NotConfigured.as_str(), "actionable_review_not_configured");
        assert_eq!(ReviewerStatus::GovernanceOnly.as_str(), "governance_only");
    }

    #[test]
    fn test_reviewer_severity_as_str() {
        assert_eq!(ReviewerSeverity::Blocking.as_str(), "blocking");
        assert_eq!(ReviewerSeverity::Major.as_str(), "major");
        assert_eq!(ReviewerSeverity::Minor.as_str(), "minor");
        assert_eq!(ReviewerSeverity::Question.as_str(), "question");
        assert_eq!(ReviewerSeverity::Nitpick.as_str(), "nitpick");
    }

    #[test]
    fn test_reviewer_severity_order() {
        assert_eq!(ReviewerSeverity::Blocking.order(), 0);
        assert_eq!(ReviewerSeverity::Major.order(), 1);
        assert_eq!(ReviewerSeverity::Minor.order(), 2);
        assert_eq!(ReviewerSeverity::Question.order(), 3);
        assert_eq!(ReviewerSeverity::Nitpick.order(), 4);
    }

    #[test]
    fn test_reviewer_severity_ordering() {
        let mut v = vec![
            ReviewerSeverity::Nitpick,
            ReviewerSeverity::Major,
            ReviewerSeverity::Blocking,
            ReviewerSeverity::Question,
            ReviewerSeverity::Minor,
        ];
        v.sort();
        assert_eq!(
            v,
            vec![
                ReviewerSeverity::Blocking,
                ReviewerSeverity::Major,
                ReviewerSeverity::Minor,
                ReviewerSeverity::Question,
                ReviewerSeverity::Nitpick,
            ]
        );
    }

    #[test]
    fn test_reviewer_output_serialization() {
        let output = ReviewerOutput {
            status: ReviewerStatus::Executed,
            reviewer: "test".to_string(),
            findings: vec![],
            coverage: ReviewerCoverage {
                changed_files_count: 1,
                files_inspected_deeply: vec![],
                files_skipped: vec![],
                exhaustive: true,
                limitations: vec![],
            },
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("\"status\":\"executed\""));
        let parsed: ReviewerOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.status, ReviewerStatus::Executed);
    }

    #[test]
    fn test_reviewer_finding_creation() {
        let finding = ReviewerFinding {
            id: "F001".to_string(),
            path: Some("src/main.rs".to_string()),
            line: Some(42),
            side: Some("RIGHT".to_string()),
            hunk_header: None,
            severity: ReviewerSeverity::Blocking,
            kind: "issue".to_string(),
            summary: "Critical bug".to_string(),
            why_it_matters: "Causes crash".to_string(),
            suggested_remediation: "Add null check".to_string(),
            suggested_change: Some("if x.is_none() { return; }".to_string()),
        };
        assert_eq!(finding.id, "F001");
        assert_eq!(finding.path, Some("src/main.rs".to_string()));
        assert_eq!(finding.severity, ReviewerSeverity::Blocking);
    }

    #[test]
    fn external_evidence_rejections_have_deterministic_diagnostics() {
        let cases = [
            (
                ExternalEvidenceRejection::InvalidProducerIdentity,
                "external producer identity is invalid",
            ),
            (ExternalEvidenceRejection::InvalidLineage, "external reviewer lineage is invalid"),
            (
                ExternalEvidenceRejection::InvalidIndependentContext,
                "independent review context is invalid",
            ),
            (
                ExternalEvidenceRejection::ClaimMismatch,
                "external evidence claims do not match policy",
            ),
            (
                ExternalEvidenceRejection::EvidenceReferenceMismatch,
                "external evidence references do not match the admitted binding",
            ),
            (ExternalEvidenceRejection::StaleEvidence, "external evidence is stale"),
            (ExternalEvidenceRejection::IncompleteEvidence, "external evidence is not terminal"),
            (
                ExternalEvidenceRejection::InsufficientChallengeTier,
                "external evidence does not satisfy the required challenge tier",
            ),
            (ExternalEvidenceRejection::InvalidNamedOverride, "named override must not be blank"),
        ];

        for (reason, expected) in cases {
            assert_eq!(reason.to_string(), expected);
        }
    }

    #[test]
    fn all_challenge_tiers_have_a_strict_deterministic_rank() {
        assert_eq!(challenge_tier_rank(ChallengeTier::Tier0), 0);
        assert_eq!(challenge_tier_rank(ChallengeTier::Tier1), 1);
        assert_eq!(challenge_tier_rank(ChallengeTier::Tier2), 2);
        assert_eq!(challenge_tier_rank(ChallengeTier::Tier3), 3);
    }
}
