use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::gate::GateKind;
use canon_engine::domain::mode::Mode;

#[test]
fn implementation_mode_uses_a_distinct_bounded_execution_artifact_bundle() {
    let contract = contract_for_mode(Mode::Implementation);

    let files = contract
        .artifact_requirements
        .iter()
        .map(|requirement| requirement.file_name.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        files,
        vec![
            "task-mapping.md",
            "mutation-bounds.md",
            "implementation-notes.md",
            "completion-evidence.md",
            "validation-hooks.md",
            "rollback-notes.md",
        ]
    );
}

#[test]
fn implementation_artifacts_require_execution_specific_sections() {
    let contract = contract_for_mode(Mode::Implementation);

    let sections = contract
        .artifact_requirements
        .iter()
        .map(|requirement| {
            (
                requirement.file_name.as_str(),
                requirement.required_sections.iter().map(String::as_str).collect::<Vec<_>>(),
                requirement.gates.clone(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        sections,
        vec![
            (
                "task-mapping.md",
                vec!["Summary", "Task Mapping", "Bounded Changes"],
                vec![GateKind::ImplementationReadiness, GateKind::ReleaseReadiness],
            ),
            (
                "mutation-bounds.md",
                vec!["Summary", "Mutation Bounds", "Allowed Paths"],
                vec![GateKind::Risk, GateKind::ImplementationReadiness],
            ),
            (
                "implementation-notes.md",
                vec![
                    "Summary",
                    "Executed Changes",
                    "Options Matrix",
                    "Recommendation",
                    "Task Linkage",
                ],
                vec![GateKind::ImplementationReadiness, GateKind::ReleaseReadiness],
            ),
            (
                "completion-evidence.md",
                vec!["Summary", "Completion Evidence", "Adoption Implications", "Remaining Risks",],
                vec![GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            (
                "validation-hooks.md",
                vec!["Summary", "Ecosystem Health", "Safety-Net Evidence", "Independent Checks",],
                vec![GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            (
                "rollback-notes.md",
                vec!["Summary", "Rollback Triggers", "Rollback Steps"],
                vec![GateKind::Risk, GateKind::ReleaseReadiness],
            ),
        ]
    );
}
