//! Canonical section heading strings shared across multiple mode artifact contracts.
//!
//! Every constant here maps to a required `## <Heading>` in a Markdown artifact.
//! Using named constants instead of inline literals prevents silent typo drift
//! and makes cross-mode renaming a single-site change.

// ── Universal ─────────────────────────────────────────────────────────────────

pub(super) const SUMMARY: &str = "Summary";

// ── Problem / framing ─────────────────────────────────────────────────────────

pub(super) const PROBLEM: &str = "Problem";
pub(super) const OUTCOME: &str = "Outcome";
pub(super) const OPTIONS: &str = "Options";
pub(super) const CONSTRAINTS: &str = "Constraints";
pub(super) const TRADEOFFS: &str = "Tradeoffs";
pub(super) const CONSEQUENCES: &str = "Consequences";
pub(super) const RECOMMENDATION: &str = "Recommendation";
pub(super) const WHY_NOT_THE_OTHERS: &str = "Why Not The Others";

// ── Risk / decision ───────────────────────────────────────────────────────────

pub(super) const DECISION: &str = "Decision";
pub(super) const DECISION_DRIVERS: &str = "Decision Drivers";
pub(super) const DECISION_EVIDENCE: &str = "Decision Evidence";
pub(super) const OPEN_QUESTIONS: &str = "Open Questions";
pub(super) const ACCEPTED_RISKS: &str = "Accepted Risks";
pub(super) const UNRESOLVED_QUESTIONS: &str = "Unresolved Questions";
pub(super) const ASSUMPTIONS: &str = "Assumptions";

// ── Architecture ──────────────────────────────────────────────────────────────

pub(super) const RATIONALE: &str = "Rationale";
pub(super) const BOUNDARIES: &str = "Boundaries";
pub(super) const OWNERSHIP: &str = "Ownership";
pub(super) const DEPENDENCIES: &str = "Dependencies";
pub(super) const COMPONENTS: &str = "Components";
pub(super) const DEPLOYMENT: &str = "Deployment";

// ── Change / delivery ─────────────────────────────────────────────────────────

pub(super) const SEQUENCING: &str = "Sequencing";
pub(super) const SCOPE: &str = "Scope";

// ── Verification / evidence ───────────────────────────────────────────────────

pub(super) const INDEPENDENT_CHECKS: &str = "Independent Checks";
pub(super) const DEFERRED_VERIFICATION: &str = "Deferred Verification";
pub(super) const SOURCE_INPUTS: &str = "Source Inputs";
pub(super) const EVIDENCE_GAPS: &str = "Evidence Gaps";
pub(super) const RELEASE_READINESS: &str = "Release Readiness";

// ── Review / disposition ──────────────────────────────────────────────────────

pub(super) const FINAL_DISPOSITION: &str = "Final Disposition";
pub(super) const MISSING_EVIDENCE: &str = "Missing Evidence";

// ── Domain / language ─────────────────────────────────────────────────────────

pub(super) const DOMAIN_SCOPE: &str = "Domain Scope";
pub(super) const UPSTREAM_SOURCES: &str = "Upstream Sources";
pub(super) const DOWNSTREAM_CONSUMERS: &str = "Downstream Consumers";
pub(super) const ADOPTION_RISKS: &str = "Adoption Risks";
pub(super) const HANDOFF_EXPECTATIONS: &str = "Handoff Expectations";
pub(super) const CONSUMER_MODES: &str = "Consumer Modes";
pub(super) const GENERATION_LINEAGE: &str = "Generation Lineage";
pub(super) const HUMAN_AUTHORED_SECTIONS: &str = "Human Authored Sections";
pub(super) const CONFIDENCE_POSTURE: &str = "Confidence Posture";
