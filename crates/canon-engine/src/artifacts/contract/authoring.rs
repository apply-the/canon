/// Artifact requirements for authoring-class modes: Requirements, Discovery, SystemShaping.
use crate::domain::artifact::ArtifactRequirement;
use crate::domain::gate::GateKind;

use super::requirement;
use super::sections::*;

// ── Artifact file-name constants ──────────────────────────────────────────────

const PROBLEM_STATEMENT_MD: &str = "problem-statement.md";
const CONSTRAINTS_MD: &str = "constraints.md";
const OPTIONS_MD: &str = "options.md";
const TRADEOFFS_MD: &str = "tradeoffs.md";
const SCOPE_CUTS_MD: &str = "scope-cuts.md";
const DECISION_CHECKLIST_MD: &str = "decision-checklist.md";
const PRD_MD: &str = "prd.md";

const PROBLEM_MAP_MD: &str = "problem-map.md";
const UNKNOWNS_AND_ASSUMPTIONS_MD: &str = "unknowns-and-assumptions.md";
const CONTEXT_BOUNDARY_MD: &str = "context-boundary.md";
const EXPLORATION_OPTIONS_MD: &str = "exploration-options.md";
const DECISION_PRESSURE_POINTS_MD: &str = "decision-pressure-points.md";

const SYSTEM_SHAPE_MD: &str = "system-shape.md";
const DOMAIN_MODEL_MD: &str = "domain-model.md";
const ARCHITECTURE_OUTLINE_MD: &str = "architecture-outline.md";
const CAPABILITY_MAP_MD: &str = "capability-map.md";
const DELIVERY_OPTIONS_MD: &str = "delivery-options.md";
const RISK_HOTSPOTS_MD: &str = "risk-hotspots.md";

// ── Mode contracts ────────────────────────────────────────────────────────────

/// Returns the artifact requirements for the [`Requirements`](crate::domain::mode::Mode::Requirements) mode.
pub(super) fn requirements() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            PROBLEM_STATEMENT_MD,
            &[SUMMARY, PROBLEM, OUTCOME],
            &[GateKind::Exploration, GateKind::Risk],
        ),
        requirement(
            CONSTRAINTS_MD,
            &[SUMMARY, CONSTRAINTS, "Non-Negotiables"],
            &[GateKind::Exploration, GateKind::Risk, GateKind::Architecture],
        ),
        requirement(
            OPTIONS_MD,
            &[SUMMARY, OPTIONS, "Recommended Path"],
            &[GateKind::Exploration, GateKind::Architecture],
        ),
        requirement(
            TRADEOFFS_MD,
            &[SUMMARY, TRADEOFFS, CONSEQUENCES],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            SCOPE_CUTS_MD,
            &[SUMMARY, "Scope Cuts", "Deferred Work"],
            &[GateKind::Exploration, GateKind::ReleaseReadiness],
        ),
        requirement(
            DECISION_CHECKLIST_MD,
            &[SUMMARY, "Decision Checklist", OPEN_QUESTIONS],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            PRD_MD,
            &[
                SUMMARY,
                PROBLEM,
                OUTCOME,
                CONSTRAINTS,
                "Recommended Path",
                TRADEOFFS,
                "Scope Cuts",
                "Decision Checklist",
            ],
            &[GateKind::Exploration, GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
    ]
}

/// Returns the artifact requirements for the [`Discovery`](crate::domain::mode::Mode::Discovery) mode.
pub(super) fn discovery() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            PROBLEM_MAP_MD,
            &[
                SUMMARY,
                "Problem Domain",
                "Repo Surface",
                "Immediate Tensions",
                "Downstream Handoff",
            ],
            &[GateKind::Exploration, GateKind::Risk],
        ),
        requirement(
            UNKNOWNS_AND_ASSUMPTIONS_MD,
            &[SUMMARY, "Unknowns", ASSUMPTIONS, "Validation Targets", "Confidence Levels"],
            &[GateKind::Exploration, GateKind::Risk],
        ),
        requirement(
            CONTEXT_BOUNDARY_MD,
            &[
                SUMMARY,
                "In-Scope Context",
                "Repo Surface",
                "Out-of-Scope Context",
                "Translation Trigger",
            ],
            &[GateKind::Exploration, GateKind::ReleaseReadiness],
        ),
        requirement(
            EXPLORATION_OPTIONS_MD,
            &[SUMMARY, OPTIONS, CONSTRAINTS, "Recommended Direction", "Next-Phase Shape"],
            &[GateKind::Exploration, GateKind::Risk],
        ),
        requirement(
            DECISION_PRESSURE_POINTS_MD,
            &[
                SUMMARY,
                "Pressure Points",
                "Blocking Decisions",
                OPEN_QUESTIONS,
                "Recommended Owner",
            ],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
    ]
}

/// Returns the artifact requirements for the [`SystemShaping`](crate::domain::mode::Mode::SystemShaping) mode.
pub(super) fn system_shaping() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            SYSTEM_SHAPE_MD,
            &[SUMMARY, "System Shape", "Boundary Decisions", "Domain Responsibilities"],
            &[GateKind::Exploration, GateKind::Architecture],
        ),
        requirement(
            DOMAIN_MODEL_MD,
            &[
                SUMMARY,
                "Candidate Bounded Contexts",
                "Core And Supporting Domain Hypotheses",
                "Ubiquitous Language",
                "Domain Invariants",
                "Boundary Risks And Open Questions",
            ],
            &[GateKind::Exploration, GateKind::Architecture],
        ),
        requirement(
            ARCHITECTURE_OUTLINE_MD,
            &[SUMMARY, "Structural Options", "Selected Boundaries", RATIONALE, WHY_NOT_THE_OTHERS],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            CAPABILITY_MAP_MD,
            &[SUMMARY, "Capabilities", DEPENDENCIES, "Gaps"],
            &[GateKind::Exploration, GateKind::Architecture],
        ),
        requirement(
            DELIVERY_OPTIONS_MD,
            &[SUMMARY, "Delivery Phases", "Sequencing Rationale", "Risk per Phase"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            RISK_HOTSPOTS_MD,
            &[SUMMARY, "Hotspots", "Mitigation Status", "Unresolved Risks"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requirements_has_expected_artifact_count() {
        assert_eq!(requirements().len(), 7);
    }

    #[test]
    fn requirements_primary_artifact_is_problem_statement() {
        assert_eq!(requirements()[0].file_name, PROBLEM_STATEMENT_MD);
    }

    #[test]
    fn requirements_all_artifacts_are_required() {
        assert!(requirements().iter().all(|r| r.required));
    }

    #[test]
    fn requirements_prd_contains_synthesis_sections() {
        let prd = requirements().into_iter().find(|r| r.file_name == PRD_MD).unwrap();
        assert!(prd.required_sections.contains(&SUMMARY.to_string()));
        assert!(prd.required_sections.contains(&PROBLEM.to_string()));
        assert!(prd.required_sections.contains(&OUTCOME.to_string()));
    }

    #[test]
    fn discovery_has_expected_artifact_count() {
        assert_eq!(discovery().len(), 5);
    }

    #[test]
    fn discovery_primary_artifact_is_problem_map() {
        assert_eq!(discovery()[0].file_name, PROBLEM_MAP_MD);
    }

    #[test]
    fn discovery_all_artifacts_are_required() {
        assert!(discovery().iter().all(|r| r.required));
    }

    #[test]
    fn system_shaping_has_expected_artifact_count() {
        assert_eq!(system_shaping().len(), 6);
    }

    #[test]
    fn system_shaping_primary_artifact_is_system_shape() {
        assert_eq!(system_shaping()[0].file_name, SYSTEM_SHAPE_MD);
    }

    #[test]
    fn system_shaping_all_artifacts_are_required() {
        assert!(system_shaping().iter().all(|r| r.required));
    }
}
