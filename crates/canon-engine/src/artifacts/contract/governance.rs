/// Artifact requirements for governance-class modes: Architecture, Review, PrReview, Verification.
use crate::domain::artifact::{ArtifactFormat, ArtifactRequirement};
use crate::domain::gate::GateKind;

use super::sections::*;
use super::{
    optional_requirement, optional_requirement_with_format, requirement, requirement_with_format,
};

// ── Artifact file-name constants ──────────────────────────────────────────────

// Architecture
const ARCHITECTURE_OVERVIEW_MD: &str = "architecture-overview.md";
const ARCHITECTURE_DECISIONS_MD: &str = "architecture-decisions.md";
const INVARIANTS_MD: &str = "invariants.md";
const TRADEOFF_MATRIX_MD: &str = "tradeoff-matrix.md";
const BOUNDARY_MAP_MD: &str = "boundary-map.md";
const CONTEXT_MAP_MD: &str = "context-map.md";
const READINESS_ASSESSMENT_MD: &str = "readiness-assessment.md";
const SYSTEM_CONTEXT_MD: &str = "system-context.md";
const SYSTEM_CONTEXT_MMD: &str = "system-context.mmd";
const CONTAINER_VIEW_MD: &str = "container-view.md";
const CONTAINER_VIEW_MMD: &str = "container-view.mmd";
const DEPLOYMENT_VIEW_MD: &str = "deployment-view.md";
const DEPLOYMENT_VIEW_MMD: &str = "deployment-view.mmd";
const COMPONENT_VIEW_MD: &str = "component-view.md";
const COMPONENT_VIEW_MMD: &str = "component-view.mmd";
const DYNAMIC_VIEW_MD: &str = "dynamic-view.md";
const DYNAMIC_VIEW_MMD: &str = "dynamic-view.mmd";
const VIEW_MANIFEST_JSON: &str = "view-manifest.json";

// Review
const REVIEW_BRIEF_MD: &str = "review-brief.md";
const BOUNDARY_ASSESSMENT_MD: &str = "boundary-assessment.md";
const MISSING_EVIDENCE_MD: &str = "missing-evidence.md";
const DECISION_IMPACT_MD: &str = "decision-impact.md";
const REVIEW_DISPOSITION_MD: &str = "review-disposition.md";

// PrReview — primary actionable review artifacts
const REVIEW_SUMMARY_MD: &str = "review-summary.md";
const CONVENTIONAL_COMMENTS_MD: &str = "conventional-comments.md";
const GITHUB_COMMENTS_JSON: &str = "github-comments.json";
const REVIEW_FINDINGS_JSON: &str = "review-findings.json";
const MISSING_TESTS_MD: &str = "missing-tests.md";
// PrReview — secondary governance artifacts
const PR_ANALYSIS_MD: &str = "pr-analysis.md";
const BOUNDARY_CHECK_MD: &str = "boundary-check.md";
const DUPLICATION_CHECK_MD: &str = "duplication-check.md";
const CONTRACT_DRIFT_MD: &str = "contract-drift.md";
const PR_DECISION_IMPACT_MD: &str = "decision-impact.md";

// Verification
const INVARIANTS_CHECKLIST_MD: &str = "invariants-checklist.md";
const CONTRACT_MATRIX_MD: &str = "contract-matrix.md";
const ADVERSARIAL_REVIEW_MD: &str = "adversarial-review.md";
const VERIFICATION_REPORT_MD: &str = "verification-report.md";
const UNRESOLVED_FINDINGS_MD: &str = "unresolved-findings.md";

// ── Mode contracts ────────────────────────────────────────────────────────────

/// Returns the artifact requirements for the [`Architecture`](crate::domain::mode::Mode::Architecture) mode.
pub(super) fn architecture() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            ARCHITECTURE_OVERVIEW_MD,
            &[
                SUMMARY,
                "Primary Decision",
                "Key Constraints",
                "Included Views",
                "Omitted Views",
                "Review Guidance",
            ],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            ARCHITECTURE_DECISIONS_MD,
            &[SUMMARY, DECISION, "Constraints", DECISION_DRIVERS, RECOMMENDATION, CONSEQUENCES],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            INVARIANTS_MD,
            &[SUMMARY, "Invariants", RATIONALE, "Verification Hooks"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            TRADEOFF_MATRIX_MD,
            &[
                SUMMARY,
                "Options Considered",
                "Evaluation Criteria",
                "Pros",
                "Cons",
                WHY_NOT_THE_OTHERS,
            ],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            BOUNDARY_MAP_MD,
            &[SUMMARY, BOUNDARIES, OWNERSHIP, "Crossing Rules"],
            &[GateKind::Exploration, GateKind::Architecture],
        ),
        requirement(
            CONTEXT_MAP_MD,
            &[
                SUMMARY,
                "Bounded Contexts",
                "Context Relationships",
                "Integration Seams",
                "Anti-Corruption Candidates",
                "Ownership Boundaries",
                "Shared Invariants",
            ],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            READINESS_ASSESSMENT_MD,
            &[
                SUMMARY,
                "Readiness Status",
                "Working Assumptions",
                UNRESOLVED_QUESTIONS,
                "Blockers",
                ACCEPTED_RISKS,
                "Recommended Next Mode",
            ],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            SYSTEM_CONTEXT_MD,
            &["System Context"],
            &[GateKind::Architecture, GateKind::Exploration],
        ),
        requirement_with_format(
            SYSTEM_CONTEXT_MMD,
            ArtifactFormat::Markdown,
            &[],
            &[GateKind::Architecture, GateKind::Exploration],
        ),
        requirement(CONTAINER_VIEW_MD, &["Containers"], &[GateKind::Architecture]),
        requirement_with_format(
            CONTAINER_VIEW_MMD,
            ArtifactFormat::Markdown,
            &[],
            &[GateKind::Architecture],
        ),
        requirement(
            DEPLOYMENT_VIEW_MD,
            &[DEPLOYMENT],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement_with_format(
            DEPLOYMENT_VIEW_MMD,
            ArtifactFormat::Markdown,
            &[],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        optional_requirement(
            COMPONENT_VIEW_MD,
            &[COMPONENTS],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        optional_requirement_with_format(
            COMPONENT_VIEW_MMD,
            ArtifactFormat::Markdown,
            &[],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        optional_requirement(
            DYNAMIC_VIEW_MD,
            &["Dynamic View"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        optional_requirement_with_format(
            DYNAMIC_VIEW_MMD,
            ArtifactFormat::Markdown,
            &[],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement_with_format(
            VIEW_MANIFEST_JSON,
            ArtifactFormat::Json,
            &[],
            &[GateKind::ReleaseReadiness],
        ),
    ]
}

/// Returns the artifact requirements for the [`Review`](crate::domain::mode::Mode::Review) mode.
pub(super) fn review() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            REVIEW_BRIEF_MD,
            &[SUMMARY, "Review Target", "Evidence Basis"],
            &[GateKind::Risk, GateKind::Architecture],
        ),
        requirement(
            BOUNDARY_ASSESSMENT_MD,
            &[SUMMARY, "Boundary Findings", "Ownership Notes"],
            &[GateKind::Architecture, GateKind::ReviewDisposition],
        ),
        requirement(
            MISSING_EVIDENCE_MD,
            &[SUMMARY, MISSING_EVIDENCE, "Collection Priorities"],
            &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
        ),
        requirement(
            DECISION_IMPACT_MD,
            &[SUMMARY, "Decision Impact", "Reversibility Concerns"],
            &[GateKind::Architecture, GateKind::ReviewDisposition],
        ),
        requirement(
            REVIEW_DISPOSITION_MD,
            &[SUMMARY, FINAL_DISPOSITION, ACCEPTED_RISKS],
            &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
        ),
    ]
}

/// Returns the artifact requirements for the [`PrReview`](crate::domain::mode::Mode::PrReview) mode.
pub(super) fn pr_review() -> Vec<ArtifactRequirement> {
    vec![
        // ── Primary actionable review artifacts ─────────────────────────────
        requirement(
            REVIEW_SUMMARY_MD,
            &[
                SUMMARY,
                DECISION,
                "Executive Summary",
                "Must-Fix Findings",
                "Accepted Risks",
                "Missing Tests",
                "GitHub-Ready Comments",
                "General Findings",
                "Governance Notes",
                "Severity",
                FINAL_DISPOSITION,
            ],
            &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
        ),
        requirement(
            CONVENTIONAL_COMMENTS_MD,
            &[SUMMARY, "Blocking Comments", "Non-Blocking Comments"],
            &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
        ),
        requirement_with_format(
            GITHUB_COMMENTS_JSON,
            ArtifactFormat::Json,
            &[],
            &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
        ),
        requirement_with_format(
            REVIEW_FINDINGS_JSON,
            ArtifactFormat::Json,
            &[],
            &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
        ),
        requirement(
            MISSING_TESTS_MD,
            &[SUMMARY, "Missing Tests"],
            &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
        ),
        // ── Secondary governance artifacts ─────────────────────────────────
        requirement(
            PR_ANALYSIS_MD,
            &[
                SUMMARY,
                "Scope Summary",
                "Changed Modules",
                "Inferred Intent",
                "Surprising Surface Area",
            ],
            &[GateKind::Risk, GateKind::ReviewDisposition],
        ),
        requirement(
            BOUNDARY_CHECK_MD,
            &[SUMMARY, "Boundary Findings", "Ownership Breaks", "Unauthorized Structural Impact"],
            &[GateKind::Architecture, GateKind::ReviewDisposition],
        ),
        requirement(
            DUPLICATION_CHECK_MD,
            &[SUMMARY, "Duplicate Behavior", "Canonical Owner Conflicts"],
            &[GateKind::ReviewDisposition],
        ),
        requirement(
            CONTRACT_DRIFT_MD,
            &[SUMMARY, "Interface Drift", "Compatibility Concerns"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            PR_DECISION_IMPACT_MD,
            &[SUMMARY, "Implied Decisions", "Absent Decision Records", "Reversibility Concerns"],
            &[GateKind::Risk, GateKind::ReviewDisposition],
        ),
    ]
}

/// Returns the artifact requirements for the [`Verification`](crate::domain::mode::Mode::Verification) mode.
pub(super) fn verification() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            INVARIANTS_CHECKLIST_MD,
            &[SUMMARY, "Claims Under Test", "Invariant Checks"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            CONTRACT_MATRIX_MD,
            &[SUMMARY, "Contract Assumptions", "Verification Outcome"],
            &[GateKind::ReleaseReadiness],
        ),
        requirement(
            ADVERSARIAL_REVIEW_MD,
            &[SUMMARY, "Challenge Findings", "Contradictions"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            VERIFICATION_REPORT_MD,
            &[SUMMARY, "Verified Claims", "Rejected Claims", "Overall Verdict"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            UNRESOLVED_FINDINGS_MD,
            &[SUMMARY, "Open Findings", "Required Follow-Up"],
            &[GateKind::ReleaseReadiness],
        ),
    ]
}

/// Returns the artifact requirements for the [`PolicyShaping`](crate::domain::mode::Mode::PolicyShaping) mode.
pub(super) fn policy_shaping() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            "conformance-impact-report.md",
            &[SUMMARY, "Impact", "Violations"],
            &[GateKind::Risk, GateKind::Architecture],
        ),
        requirement(
            "policy-diff.md",
            &[SUMMARY, "Semantic Changes"],
            &[GateKind::ReleaseReadiness],
        ),
        requirement(
            "04-migration.md",
            &[SUMMARY, "Waiver Policy", "Rollout Phases", "Debt Created"],
            &[GateKind::ReleaseReadiness],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn architecture_has_required_and_optional_artifacts() {
        let reqs = architecture();
        let required_count = reqs.iter().filter(|r| r.required).count();
        let optional_count = reqs.iter().filter(|r| !r.required).count();
        assert!(required_count > 0);
        assert_eq!(optional_count, 4); // component-view.md/mmd + dynamic-view.md/mmd
    }

    #[test]
    fn architecture_primary_artifact_is_overview() {
        assert_eq!(architecture()[0].file_name, ARCHITECTURE_OVERVIEW_MD);
    }

    #[test]
    fn architecture_includes_view_manifest() {
        assert!(architecture().iter().any(|r| r.file_name == VIEW_MANIFEST_JSON));
    }

    #[test]
    fn review_has_expected_artifact_count() {
        assert_eq!(review().len(), 5);
    }

    #[test]
    fn review_all_artifacts_are_required() {
        assert!(review().iter().all(|r| r.required));
    }

    #[test]
    fn pr_review_has_expected_artifact_count() {
        assert_eq!(pr_review().len(), 10);
    }

    #[test]
    fn pr_review_primary_artifact_is_review_summary() {
        assert_eq!(pr_review()[0].file_name, REVIEW_SUMMARY_MD);
    }

    #[test]
    fn verification_has_expected_artifact_count() {
        assert_eq!(verification().len(), 5);
    }

    #[test]
    fn verification_all_artifacts_are_required() {
        assert!(verification().iter().all(|r| r.required));
    }

    #[test]
    fn policy_shaping_has_expected_artifact_count() {
        assert_eq!(policy_shaping().len(), 3);
    }

    #[test]
    fn policy_shaping_all_artifacts_are_required() {
        assert!(policy_shaping().iter().all(|r| r.required));
    }

    #[test]
    fn policy_shaping_primary_artifact_is_conformance_impact_report() {
        assert_eq!(policy_shaping()[0].file_name, "conformance-impact-report.md");
    }
}
