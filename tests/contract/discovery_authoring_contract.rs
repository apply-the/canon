use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::gate::GateKind;
use canon_engine::domain::mode::Mode;

#[test]
fn discovery_contract_uses_repo_surface_for_problem_map() {
    let contract = contract_for_mode(Mode::Discovery);
    let problem_map = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "problem-map.md")
        .expect("problem map requirement");

    assert_eq!(
        problem_map.required_sections,
        vec![
            "Summary",
            "Problem Domain",
            "Repo Surface",
            "Immediate Tensions",
            "Downstream Handoff"
        ]
    );
    assert_eq!(problem_map.gates, vec![GateKind::Exploration, GateKind::Risk]);
}

#[test]
fn discovery_contract_preserves_context_boundary_shape() {
    let contract = contract_for_mode(Mode::Discovery);
    let context_boundary = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "context-boundary.md")
        .expect("context boundary requirement");

    assert_eq!(
        context_boundary.required_sections,
        vec![
            "Summary",
            "In-Scope Context",
            "Repo Surface",
            "Out-of-Scope Context",
            "Translation Trigger"
        ]
    );
    assert_eq!(context_boundary.gates, vec![GateKind::Exploration, GateKind::ReleaseReadiness]);
}
