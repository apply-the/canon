use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::gate::GateKind;
use canon_engine::domain::mode::Mode;

#[test]
fn requirements_contract_uses_authored_problem_and_outcome_sections() {
    let contract = contract_for_mode(Mode::Requirements);
    let problem_statement = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "problem-statement.md")
        .expect("problem statement requirement");

    assert_eq!(problem_statement.required_sections, vec!["Summary", "Problem", "Outcome"]);
    assert_eq!(problem_statement.gates, vec![GateKind::Exploration, GateKind::Risk]);
}

#[test]
fn requirements_contract_preserves_scope_cuts_release_readiness_shape() {
    let contract = contract_for_mode(Mode::Requirements);
    let scope_cuts = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "scope-cuts.md")
        .expect("scope cuts requirement");

    assert_eq!(scope_cuts.required_sections, vec!["Summary", "Scope Cuts", "Deferred Work"]);
    assert_eq!(scope_cuts.gates, vec![GateKind::Exploration, GateKind::ReleaseReadiness]);
}
