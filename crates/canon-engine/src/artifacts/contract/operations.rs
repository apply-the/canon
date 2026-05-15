/// Artifact requirements for operations-class modes: Incident, Migration.
use crate::domain::artifact::ArtifactRequirement;
use crate::domain::gate::GateKind;

use super::requirement;
use super::sections::*;

// ── Artifact file-name constants ──────────────────────────────────────────────

// Incident
const INCIDENT_FRAME_MD: &str = "incident-frame.md";
const HYPOTHESIS_LOG_MD: &str = "hypothesis-log.md";
const BLAST_RADIUS_MAP_MD: &str = "blast-radius-map.md";
const CONTAINMENT_PLAN_MD: &str = "containment-plan.md";
const INCIDENT_DECISION_RECORD_MD: &str = "incident-decision-record.md";
const FOLLOW_UP_VERIFICATION_MD: &str = "follow-up-verification.md";

// Migration
const SOURCE_TARGET_MAP_MD: &str = "source-target-map.md";
const COMPATIBILITY_MATRIX_MD: &str = "compatibility-matrix.md";
const MIGRATION_SEQUENCING_PLAN_MD: &str = "sequencing-plan.md";
const FALLBACK_PLAN_MD: &str = "fallback-plan.md";
const MIGRATION_VERIFICATION_REPORT_MD: &str = "migration-verification-report.md";
const MIGRATION_DECISION_RECORD_MD: &str = "decision-record.md";

// ── Mode contracts ────────────────────────────────────────────────────────────

/// Returns the artifact requirements for the [`Incident`](crate::domain::mode::Mode::Incident) mode.
pub(super) fn incident() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            INCIDENT_FRAME_MD,
            &[SUMMARY, "Incident Scope", "Trigger And Current State", "Operational Constraints"],
            &[GateKind::Risk, GateKind::IncidentContainment, GateKind::Architecture],
        ),
        requirement(
            HYPOTHESIS_LOG_MD,
            &[SUMMARY, "Known Facts", "Working Hypotheses", EVIDENCE_GAPS],
            &[GateKind::IncidentContainment, GateKind::Risk],
        ),
        requirement(
            BLAST_RADIUS_MAP_MD,
            &[SUMMARY, "Impacted Surfaces", "Propagation Paths", "Confidence And Unknowns"],
            &[GateKind::IncidentContainment, GateKind::Architecture],
        ),
        requirement(
            CONTAINMENT_PLAN_MD,
            &[SUMMARY, "Immediate Actions", "Ordered Sequence", "Stop Conditions"],
            &[GateKind::IncidentContainment, GateKind::ReleaseReadiness],
        ),
        requirement(
            INCIDENT_DECISION_RECORD_MD,
            &[SUMMARY, "Decision Points", "Approved Actions", "Deferred Actions"],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            FOLLOW_UP_VERIFICATION_MD,
            &[SUMMARY, "Verification Checks", RELEASE_READINESS, "Follow-Up Work"],
            &[GateKind::ReleaseReadiness],
        ),
    ]
}

/// Returns the artifact requirements for the [`Migration`](crate::domain::mode::Mode::Migration) mode.
pub(super) fn migration() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            SOURCE_TARGET_MAP_MD,
            &[SUMMARY, "Current State", "Target State", "Transition Boundaries"],
            &[GateKind::Exploration, GateKind::Architecture],
        ),
        requirement(
            COMPATIBILITY_MATRIX_MD,
            &[
                SUMMARY,
                "Guaranteed Compatibility",
                "Temporary Incompatibilities",
                "Coexistence Rules",
                "Options Matrix",
            ],
            &[GateKind::Architecture, GateKind::MigrationSafety],
        ),
        requirement(
            MIGRATION_SEQUENCING_PLAN_MD,
            &[SUMMARY, "Ordered Steps", "Parallelizable Work", "Cutover Criteria"],
            &[GateKind::MigrationSafety, GateKind::ReleaseReadiness],
        ),
        requirement(
            FALLBACK_PLAN_MD,
            &[
                SUMMARY,
                "Rollback Triggers",
                "Fallback Paths",
                "Re-Entry Criteria",
                "Adoption Implications",
            ],
            &[GateKind::MigrationSafety, GateKind::Risk],
        ),
        requirement(
            MIGRATION_VERIFICATION_REPORT_MD,
            &[SUMMARY, "Verification Checks", "Residual Risks", RELEASE_READINESS],
            &[GateKind::MigrationSafety, GateKind::ReleaseReadiness],
        ),
        requirement(
            MIGRATION_DECISION_RECORD_MD,
            &[
                SUMMARY,
                "Migration Decisions",
                "Tradeoff Analysis",
                DECISION_EVIDENCE,
                RECOMMENDATION,
                WHY_NOT_THE_OTHERS,
                "Ecosystem Health",
                "Deferred Decisions",
                "Approval Notes",
            ],
            &[GateKind::Architecture, GateKind::Risk],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incident_has_expected_artifact_count() {
        assert_eq!(incident().len(), 6);
    }

    #[test]
    fn incident_primary_artifact_is_frame() {
        assert_eq!(incident()[0].file_name, INCIDENT_FRAME_MD);
    }

    #[test]
    fn incident_all_artifacts_are_required() {
        assert!(incident().iter().all(|r| r.required));
    }

    #[test]
    fn migration_has_expected_artifact_count() {
        assert_eq!(migration().len(), 6);
    }

    #[test]
    fn migration_primary_artifact_is_source_target_map() {
        assert_eq!(migration()[0].file_name, SOURCE_TARGET_MAP_MD);
    }

    #[test]
    fn migration_decision_record_contains_recommendation() {
        let dr =
            migration().into_iter().find(|r| r.file_name == MIGRATION_DECISION_RECORD_MD).unwrap();
        assert!(dr.required_sections.contains(&RECOMMENDATION.to_string()));
        assert!(dr.required_sections.contains(&WHY_NOT_THE_OTHERS.to_string()));
    }
}
