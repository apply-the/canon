/// Artifact requirements for delivery-class modes: Backlog, Change, Implementation, Refactor.
use crate::domain::artifact::ArtifactRequirement;
use crate::domain::gate::GateKind;

use super::sections::*;
use super::{optional_requirement, requirement};

// ── Artifact file-name constants ──────────────────────────────────────────────

// Backlog
const BACKLOG_OVERVIEW_MD: &str = "backlog-overview.md";
const EPIC_TREE_MD: &str = "epic-tree.md";
const CAPABILITY_TO_EPIC_MAP_MD: &str = "capability-to-epic-map.md";
const DEPENDENCY_MAP_MD: &str = "dependency-map.md";
const DELIVERY_SLICES_MD: &str = "delivery-slices.md";
const SEQUENCING_PLAN_MD: &str = "sequencing-plan.md";
const ACCEPTANCE_ANCHORS_MD: &str = "acceptance-anchors.md";
const PLANNING_RISKS_MD: &str = "planning-risks.md";
const EXECUTION_HANDOFF_MD: &str = "execution-handoff.md";

// Change
const SYSTEM_SLICE_MD: &str = "system-slice.md";
const LEGACY_INVARIANTS_MD: &str = "legacy-invariants.md";
const CHANGE_SURFACE_MD: &str = "change-surface.md";
const IMPLEMENTATION_PLAN_MD: &str = "implementation-plan.md";
const VALIDATION_STRATEGY_MD: &str = "validation-strategy.md";
const CHANGE_DECISION_RECORD_MD: &str = "decision-record.md";

// Implementation
const TASK_MAPPING_MD: &str = "task-mapping.md";
const MUTATION_BOUNDS_MD: &str = "mutation-bounds.md";
const IMPLEMENTATION_NOTES_MD: &str = "implementation-notes.md";
const COMPLETION_EVIDENCE_MD: &str = "completion-evidence.md";
const VALIDATION_HOOKS_MD: &str = "validation-hooks.md";
const ROLLBACK_NOTES_MD: &str = "rollback-notes.md";

// Refactor
const PRESERVED_BEHAVIOR_MD: &str = "preserved-behavior.md";
const REFACTOR_SCOPE_MD: &str = "refactor-scope.md";
const STRUCTURAL_RATIONALE_MD: &str = "structural-rationale.md";
const REGRESSION_EVIDENCE_MD: &str = "regression-evidence.md";
const CONTRACT_DRIFT_CHECK_MD: &str = "contract-drift-check.md";
const NO_FEATURE_ADDITION_MD: &str = "no-feature-addition.md";

// Debugging
const CONTEXT_MAP_MD: &str = "context-map.md";
const REPRODUCTION_HARNESS_MD: &str = "reproduction-harness.md";
const ROOT_CAUSE_ISOLATION_MD: &str = "root-cause-isolation.md";
const FIX_APPLICATION_MD: &str = "fix-application.md";
const VERIFICATION_SUMMARY_MD: &str = "verification-summary.md";

// ── Mode contracts ────────────────────────────────────────────────────────────

/// Returns the artifact requirements for the [`Backlog`](crate::domain::mode::Mode::Backlog) mode.
pub(super) fn backlog() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            BACKLOG_OVERVIEW_MD,
            &[
                SUMMARY,
                SCOPE,
                "Planning Horizon",
                SOURCE_INPUTS,
                "Delivery Intent",
                "Decomposition Posture",
                "Execution Handoff",
            ],
            &[GateKind::Exploration, GateKind::Risk],
        ),
        requirement(
            EPIC_TREE_MD,
            &[SUMMARY, "Epic Tree", "Scope Boundaries", "Source Trace Links"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            CAPABILITY_TO_EPIC_MAP_MD,
            &[SUMMARY, "Capability Mapping", "Source Trace Links", "Planning Gaps"],
            &[GateKind::Exploration, GateKind::Architecture],
        ),
        requirement(
            DEPENDENCY_MAP_MD,
            &[SUMMARY, DEPENDENCIES, "Slice IDs", "Blocking Edges", "External Dependencies"],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            DELIVERY_SLICES_MD,
            &[SUMMARY, "Delivery Slices", "Slice IDs", "Slice Boundaries", "Dependency Links"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            SEQUENCING_PLAN_MD,
            &[SUMMARY, SEQUENCING, "Slice IDs", "Ordering Rationale", "Readiness Signals"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            ACCEPTANCE_ANCHORS_MD,
            &[SUMMARY, "Acceptance Anchors", "Slice IDs", "Source Trace Links", "Deferred Detail"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            PLANNING_RISKS_MD,
            &[SUMMARY, "Closure Findings", "Planning Risks", "Follow-Up Triggers"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        optional_requirement(
            EXECUTION_HANDOFF_MD,
            &[
                SUMMARY,
                "Selected Slice",
                "Implementation Artifact References",
                "Dependency Prerequisites",
                "Independent Verification Anchors",
                "Execution Boundary",
            ],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
    ]
}

/// Returns the artifact requirements for the [`Change`](crate::domain::mode::Mode::Change) mode.
pub(super) fn change() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            SYSTEM_SLICE_MD,
            &[SUMMARY, "System Slice", "Domain Slice", "Excluded Areas"],
            &[GateKind::Exploration, GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            LEGACY_INVARIANTS_MD,
            &[SUMMARY, "Legacy Invariants", "Domain Invariants", "Forbidden Normalization"],
            &[GateKind::ChangePreservation, GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            CHANGE_SURFACE_MD,
            &[SUMMARY, "Change Surface", OWNERSHIP, "Cross-Context Risks"],
            &[GateKind::ChangePreservation, GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            IMPLEMENTATION_PLAN_MD,
            &[SUMMARY, "Implementation Plan", SEQUENCING],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            VALIDATION_STRATEGY_MD,
            &[SUMMARY, "Validation Strategy", INDEPENDENT_CHECKS],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            CHANGE_DECISION_RECORD_MD,
            &[
                SUMMARY,
                "Decision Record",
                DECISION_DRIVERS,
                "Options Considered",
                DECISION_EVIDENCE,
                "Boundary Tradeoffs",
                RECOMMENDATION,
                WHY_NOT_THE_OTHERS,
                CONSEQUENCES,
                UNRESOLVED_QUESTIONS,
            ],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
    ]
}

/// Returns the artifact requirements for the [`Implementation`](crate::domain::mode::Mode::Implementation) mode.
pub(super) fn implementation() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            TASK_MAPPING_MD,
            &[SUMMARY, "Task Mapping", "Bounded Changes"],
            &[GateKind::ImplementationReadiness, GateKind::ReleaseReadiness],
        ),
        requirement(
            MUTATION_BOUNDS_MD,
            &[SUMMARY, "Mutation Bounds", "Allowed Paths"],
            &[GateKind::Risk, GateKind::ImplementationReadiness],
        ),
        requirement(
            IMPLEMENTATION_NOTES_MD,
            &[
                SUMMARY,
                "Executed Changes",
                "Candidate Frameworks",
                "Options Matrix",
                DECISION_EVIDENCE,
                RECOMMENDATION,
                "Task Linkage",
            ],
            &[GateKind::ImplementationReadiness, GateKind::ReleaseReadiness],
        ),
        requirement(
            COMPLETION_EVIDENCE_MD,
            &[SUMMARY, "Completion Evidence", "Adoption Implications", "Remaining Risks"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            VALIDATION_HOOKS_MD,
            &[SUMMARY, "Ecosystem Health", "Safety-Net Evidence", INDEPENDENT_CHECKS],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            ROLLBACK_NOTES_MD,
            &[SUMMARY, "Rollback Triggers", "Rollback Steps"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
    ]
}

/// Returns the artifact requirements for the [`Refactor`](crate::domain::mode::Mode::Refactor) mode.
pub(super) fn refactor() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            PRESERVED_BEHAVIOR_MD,
            &[SUMMARY, "Preserved Behavior", "Approved Exceptions"],
            &[GateKind::ChangePreservation, GateKind::ReleaseReadiness],
        ),
        requirement(
            REFACTOR_SCOPE_MD,
            &[SUMMARY, "Refactor Scope", "Allowed Paths"],
            &[GateKind::ChangePreservation, GateKind::Risk],
        ),
        requirement(
            STRUCTURAL_RATIONALE_MD,
            &[SUMMARY, "Structural Rationale", "Untouched Surface"],
            &[GateKind::Exploration, GateKind::ChangePreservation],
        ),
        requirement(
            REGRESSION_EVIDENCE_MD,
            &[SUMMARY, "Safety-Net Evidence", "Regression Findings"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            CONTRACT_DRIFT_CHECK_MD,
            &[SUMMARY, "Contract Drift", "Reviewer Notes"],
            &[GateKind::Architecture, GateKind::ChangePreservation],
        ),
        requirement(
            NO_FEATURE_ADDITION_MD,
            &[SUMMARY, "Feature Audit", DECISION],
            &[GateKind::ChangePreservation, GateKind::ReleaseReadiness],
        ),
    ]
}

/// Returns the artifact requirements for the [`Debugging`](crate::domain::mode::Mode::Debugging) mode.
pub(super) fn debugging() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            CONTEXT_MAP_MD,
            &[SUMMARY, "Context Map", "Defect Description", "Stakeholder Impact"],
            &[GateKind::Exploration, GateKind::Risk],
        ),
        requirement(
            REPRODUCTION_HARNESS_MD,
            &[SUMMARY, "Reproduction Harness", "Red State Verification"],
            &[GateKind::Reproduction, GateKind::TestDrivenDevelopment],
        ),
        requirement(
            ROOT_CAUSE_ISOLATION_MD,
            &[SUMMARY, "Root Cause Isolation", "Fault Chain", "Isolation Proof"],
            &[GateKind::RootCause, GateKind::Architecture],
        ),
        requirement(
            FIX_APPLICATION_MD,
            &[SUMMARY, "Fix Application", "Bounded Changes", "Invariant Preservation"],
            &[GateKind::RootCause, GateKind::ReleaseReadiness],
        ),
        requirement(
            VERIFICATION_SUMMARY_MD,
            &[SUMMARY, "Verification Summary", "Green State", "No Regression Evidence"],
            &[GateKind::ReleaseReadiness, GateKind::TestDrivenDevelopment],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backlog_has_expected_artifact_count() {
        assert_eq!(backlog().len(), 9);
    }

    #[test]
    fn backlog_primary_artifact_is_overview() {
        assert_eq!(backlog()[0].file_name, BACKLOG_OVERVIEW_MD);
    }

    #[test]
    fn backlog_handoff_artifact_is_optional() {
        let handoff =
            backlog().into_iter().find(|requirement| requirement.file_name == EXECUTION_HANDOFF_MD);
        assert!(handoff.is_some_and(|requirement| !requirement.required));
    }

    #[test]
    fn change_has_expected_artifact_count() {
        assert_eq!(change().len(), 6);
    }

    #[test]
    fn change_primary_artifact_is_system_slice() {
        assert_eq!(change()[0].file_name, SYSTEM_SLICE_MD);
    }

    #[test]
    fn change_decision_record_contains_all_required_sections() {
        let dr = change().into_iter().find(|r| r.file_name == CHANGE_DECISION_RECORD_MD).unwrap();
        assert!(dr.required_sections.contains(&RECOMMENDATION.to_string()));
        assert!(dr.required_sections.contains(&WHY_NOT_THE_OTHERS.to_string()));
    }

    #[test]
    fn implementation_has_expected_artifact_count() {
        assert_eq!(implementation().len(), 6);
    }

    #[test]
    fn implementation_primary_artifact_is_task_mapping() {
        assert_eq!(implementation()[0].file_name, TASK_MAPPING_MD);
    }

    #[test]
    fn refactor_has_expected_artifact_count() {
        assert_eq!(refactor().len(), 6);
    }

    #[test]
    fn refactor_primary_artifact_is_preserved_behavior() {
        assert_eq!(refactor()[0].file_name, PRESERVED_BEHAVIOR_MD);
    }

    #[test]
    fn refactor_all_artifacts_are_required() {
        assert!(refactor().iter().all(|r| r.required));
    }
}
