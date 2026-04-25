use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::mode::Mode;

#[test]
fn incident_mode_uses_a_distinct_containment_artifact_bundle() {
    let contract = contract_for_mode(Mode::Incident);

    let files = contract
        .artifact_requirements
        .iter()
        .map(|requirement| requirement.file_name.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        files,
        vec![
            "incident-frame.md",
            "hypothesis-log.md",
            "blast-radius-map.md",
            "containment-plan.md",
            "incident-decision-record.md",
            "follow-up-verification.md",
        ]
    );
}

#[test]
fn incident_artifacts_require_containment_specific_sections() {
    let contract = contract_for_mode(Mode::Incident);

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
                "incident-frame.md",
                vec![
                    "Summary",
                    "Incident Scope",
                    "Trigger And Current State",
                    "Operational Constraints"
                ],
            ),
            (
                "hypothesis-log.md",
                vec!["Summary", "Known Facts", "Working Hypotheses", "Evidence Gaps"],
            ),
            (
                "blast-radius-map.md",
                vec![
                    "Summary",
                    "Impacted Surfaces",
                    "Propagation Paths",
                    "Confidence And Unknowns"
                ],
            ),
            (
                "containment-plan.md",
                vec!["Summary", "Immediate Actions", "Ordered Sequence", "Stop Conditions"],
            ),
            (
                "incident-decision-record.md",
                vec!["Summary", "Decision Points", "Approved Actions", "Deferred Actions"],
            ),
            (
                "follow-up-verification.md",
                vec!["Summary", "Verification Checks", "Release Readiness", "Follow-Up Work"],
            ),
        ]
    );
}
