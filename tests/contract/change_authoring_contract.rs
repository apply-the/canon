use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::gate::GateKind;
use canon_engine::domain::mode::Mode;

#[test]
fn change_contract_uses_implementation_plan_heading() {
    let contract = contract_for_mode(Mode::Change);
    let implementation_plan = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "implementation-plan.md")
        .expect("implementation plan requirement");

    assert_eq!(
        implementation_plan.required_sections,
        vec!["Summary", "Implementation Plan", "Sequencing"]
    );
    assert_eq!(implementation_plan.gates, vec![GateKind::Architecture, GateKind::ReleaseReadiness]);
}

#[test]
fn change_contract_uses_decision_record_heading() {
    let contract = contract_for_mode(Mode::Change);
    let decision_record = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "decision-record.md")
        .expect("decision record requirement");

    assert_eq!(
        decision_record.required_sections,
        vec![
            "Summary",
            "Decision Record",
            "Boundary Tradeoffs",
            "Consequences",
            "Unresolved Questions",
        ]
    );
    assert_eq!(decision_record.gates, vec![GateKind::Architecture, GateKind::ReleaseReadiness]);
}
