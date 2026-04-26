use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::gate::GateKind;
use canon_engine::domain::mode::Mode;

#[test]
fn refactor_mode_uses_a_distinct_preservation_artifact_bundle() {
    let contract = contract_for_mode(Mode::Refactor);

    let files = contract
        .artifact_requirements
        .iter()
        .map(|requirement| requirement.file_name.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        files,
        vec![
            "preserved-behavior.md",
            "refactor-scope.md",
            "structural-rationale.md",
            "regression-evidence.md",
            "contract-drift-check.md",
            "no-feature-addition.md",
        ]
    );
}

#[test]
fn refactor_artifacts_require_preservation_specific_sections() {
    let contract = contract_for_mode(Mode::Refactor);

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
                "preserved-behavior.md",
                vec!["Summary", "Preserved Behavior", "Approved Exceptions"],
                vec![GateKind::ChangePreservation, GateKind::ReleaseReadiness],
            ),
            (
                "refactor-scope.md",
                vec!["Summary", "Refactor Scope", "Allowed Paths"],
                vec![GateKind::ChangePreservation, GateKind::Risk],
            ),
            (
                "structural-rationale.md",
                vec!["Summary", "Structural Rationale", "Untouched Surface"],
                vec![GateKind::Exploration, GateKind::ChangePreservation],
            ),
            (
                "regression-evidence.md",
                vec!["Summary", "Safety-Net Evidence", "Regression Findings"],
                vec![GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            (
                "contract-drift-check.md",
                vec!["Summary", "Contract Drift", "Reviewer Notes"],
                vec![GateKind::Architecture, GateKind::ChangePreservation],
            ),
            (
                "no-feature-addition.md",
                vec!["Summary", "Feature Audit", "Decision"],
                vec![GateKind::ChangePreservation, GateKind::ReleaseReadiness],
            ),
        ]
    );
}
