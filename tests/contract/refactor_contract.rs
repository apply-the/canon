use canon_engine::artifacts::contract::contract_for_mode;
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
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        sections,
        vec![
            ("preserved-behavior.md", vec!["Summary", "Preserved Behavior", "Approved Exceptions"],),
            ("refactor-scope.md", vec!["Summary", "Refactor Scope", "Allowed Paths"],),
            (
                "structural-rationale.md",
                vec!["Summary", "Structural Rationale", "Untouched Surface"],
            ),
            (
                "regression-evidence.md",
                vec!["Summary", "Safety-Net Evidence", "Regression Findings"],
            ),
            ("contract-drift-check.md", vec!["Summary", "Contract Drift", "Reviewer Notes"],),
            ("no-feature-addition.md", vec!["Summary", "Feature Audit", "Decision"],),
        ]
    );
}
