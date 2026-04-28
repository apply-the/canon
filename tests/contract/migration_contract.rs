use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::mode::Mode;

#[test]
fn migration_mode_uses_a_distinct_compatibility_artifact_bundle() {
    let contract = contract_for_mode(Mode::Migration);

    let files = contract
        .artifact_requirements
        .iter()
        .map(|requirement| requirement.file_name.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        files,
        vec![
            "source-target-map.md",
            "compatibility-matrix.md",
            "sequencing-plan.md",
            "fallback-plan.md",
            "migration-verification-report.md",
            "decision-record.md",
        ]
    );
}

#[test]
fn migration_artifacts_require_compatibility_specific_sections() {
    let contract = contract_for_mode(Mode::Migration);

    let sections = contract
        .artifact_requirements
        .iter()
        .map(|requirement| {
            (
                requirement.file_name.as_str(),
                requirement.required_sections.iter().map(String::as_str).collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        sections,
        vec![
            (
                "source-target-map.md",
                vec!["Summary", "Current State", "Target State", "Transition Boundaries"],
            ),
            (
                "compatibility-matrix.md",
                vec![
                    "Summary",
                    "Guaranteed Compatibility",
                    "Temporary Incompatibilities",
                    "Coexistence Rules",
                    "Options Matrix"
                ],
            ),
            (
                "sequencing-plan.md",
                vec!["Summary", "Ordered Steps", "Parallelizable Work", "Cutover Criteria"],
            ),
            (
                "fallback-plan.md",
                vec![
                    "Summary",
                    "Rollback Triggers",
                    "Fallback Paths",
                    "Re-Entry Criteria",
                    "Adoption Implications",
                ],
            ),
            (
                "migration-verification-report.md",
                vec!["Summary", "Verification Checks", "Residual Risks", "Release Readiness"],
            ),
            (
                "decision-record.md",
                vec![
                    "Summary",
                    "Migration Decisions",
                    "Tradeoff Analysis",
                    "Recommendation",
                    "Ecosystem Health",
                    "Deferred Decisions",
                    "Approval Notes",
                ],
            ),
        ]
    );
}
